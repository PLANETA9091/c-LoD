//! DH vertical slices — 1:1 port of the Distant Horizons 3.3.4/3.3.5
//! datapoint-column machinery that our single-heightmap cell collapses.
//!
//! Sources (analysis/dh/src, coreSubProjects/core):
//!   - `dataObjects/fullData/sources/FullDataSourceV2.java`:
//!     `mergeInputTwoByTwoDataColumn` + `determineMostCommonValueInColumnSlice`
//!     (the full-data 2x2 pyramid merge) — every y boundary of the 4 child
//!     columns becomes a slice span; each column votes the datapoint that
//!     covers the span's MIDDLE (`sampleY = minY + height/2`); air always
//!     loses ("always overwrite air to prevent holes in hollow structures");
//!     ties resolve to value0 (the first column); adjacent spans with equal
//!     winners merge into one datapoint.
//!   - `util/RenderDataPointReducingList.java`: the render-time reducer.
//!     `reduce(target)` runs step 0+1 (merge the smallest segment into an
//!     equal-alpha neighbour — both match -> the SMALLER neighbour, equal
//!     size -> the higher one; the survivor keeps its own colour and extends
//!     over the removed node), step 2 (erase a segment between equal-alpha
//!     neighbours), step 3 (force the lowest segment to merge upward) until
//!     `sizeWithoutAir <= target`.
//!   - `api/enums/config/EDhApiVerticalQuality.java`: MEDIUM (DH default)
//!     = {6,6,6,4,4,4,4,4,4,4,1} indexed by the datapoint detail level.
//!
//! c-LoD mapping: one atlas cell at octave L is a datapoint of width
//! `cell_size * 2^L` blocks, so its detail level is `log2(cell_size) + L`
//! and the slice target is `MEDIUM[detail]`. The scan produces the level-0
//! run list (top-down, cut on Material change — DH cuts on blockstate
//! change; Material granularity is the same at LOD resolution); every octave
//! up merges 2x2 with the vote rule and encodes `reduce(list, MEDIUM[d])`.
//!
//! The LAST run of every column reaches the world bottom in DH (full-data
//! columns span to min_y); our scan walk stops at a depth floor, so the same
//! "extends to -infinity" bottom is applied to the last run — wall hits
//! below the walk floor then pick the deepest band, exactly like DH's
//! bottom datapoint covers the deep wall.
//!
//! Documented deviations (posteffect heightfield, not a DH mesh):
//!   - DH stores cave air datapoints (invisible, walls get holes); we merge
//!     solid runs ACROSS air in the scan, so a wall renders the upper band
//!     extended over the gap. That matches DH's cave-CULLED look (the culler
//!     removes enclosed air; open cave mouths in DH show overdraw, which a
//!     heightfield cannot represent at all).
//!   - DH keeps unreduced full data per level and reduces only at render
//!     time; we reduce the merged parent to [`RAW_CAP`] runs before the next
//!     merge to bound memory (the reducer itself is the DH-blessed shrinker).

/// One vertical run of a column: blocks (next_run.top, top] (the top block's
/// Y; the atlas surface plane convention keeps the top face at `height`),
/// rendered flat in `color` (DH: ONE colour per datapoint — the block's
/// first non-empty face average, biome tinted), textured near-field by
/// `tex_top`/`tex_side` ratio tiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceRun {
    /// Block Y of the run's topmost block.
    pub top: i16,
    /// Flat datapoint colour (0xRRGGBB).
    pub color: u32,
    pub tex_top: u16,
    pub tex_side: u16,
    /// Liquid run (water): the band sits under the water surface plane.
    pub liquid: bool,
    /// DH LodQuadBuilder FADE_TO_DIRT: grass-material walls fade to the dirt
    /// colour across the band (EDhApiBlockMaterial.GRASS).
    pub grass: bool,
}

/// EDhApiVerticalQuality.MEDIUM.maxVerticalData — DH default quality,
/// indexed by the datapoint detail level (0 = 1x1 block datapoints).
pub const MEDIUM: [i32; 11] = [6, 6, 6, 4, 4, 4, 4, 4, 4, 4, 1];

/// `calculateMaxNumberOfVerticalSlicesAtDetailLevel`, clamped like
/// `MathUtil.clamp(0, dataDetail, len-1)`.
#[must_use]
pub fn slices_at_detail(detail: i64) -> usize {
    let idx = detail.clamp(0, MEDIUM.len() as i64 - 1) as usize;
    MEDIUM[idx] as usize
}

/// Identity used for the slice vote (DH votes the datapoint id = the
/// (biome, blockstate) pair; we vote the baked appearance tuple — two runs
/// with identical colour/tiles/material flags are indistinguishable pixels).
fn same_run(a: &SliceRun, b: &SliceRun) -> bool {
    a.color == b.color
        && a.tex_top == b.tex_top
        && a.tex_side == b.tex_side
        && a.liquid == b.liquid
        && a.grass == b.grass
}

/// DH datapoint span of a run inside a top-down list: (bottom, top_exclusive)
/// with the LAST run extending to the world floor (`-inf` semantics: half
/// i32::MIN to keep every subtraction in i64 range without overflow).
fn span(col: &[SliceRun], i: usize) -> (i64, i64) {
    let top = i64::from(col[i].top) + 1;
    let bottom = col
        .get(i + 1)
        .map_or(i64::from(i32::MIN) / 2, |below| i64::from(below.top) + 1);
    (bottom, top)
}

/// DH `FullDataSourceV2.mergeInputTwoByTwoDataColumn`, on top-down lists.
/// Any child may be empty (void/un-scanned — DH's null column votes nothing,
/// i.e. air). Returns the merged list top-down.
#[must_use]
pub fn merge_two_by_two(cols: [&[SliceRun]; 4]) -> Vec<SliceRun> {
    // yTransitions: every child datapoint boundary, sorted bottom-up (DH).
    let mut transitions: Vec<i64> = Vec::with_capacity(32);
    for col in &cols {
        for i in 0..col.len() {
            let (bottom, top) = span(col, i);
            transitions.push(bottom);
            transitions.push(top);
        }
    }
    if transitions.is_empty() {
        return Vec::new();
    }
    transitions.sort_unstable();
    transitions.dedup();

    // Per-column index trackers, starting at the TOP-most datapoint (DH).
    let mut idx: [isize; 4] = [
        cols[0].len() as isize - 1,
        cols[1].len() as isize - 1,
        cols[2].len() as isize - 1,
        cols[3].len() as isize - 1,
    ];

    let mut out: Vec<SliceRun> = Vec::new();
    let mut acc: Option<SliceRun> = None;

    for w in 0..transitions.len().saturating_sub(1) {
        let slice_min = transitions[w];
        let slice_max = transitions[w + 1];
        let slice_h = slice_max - slice_min;
        if slice_h <= 0 {
            continue;
        }
        let sample_y = slice_min + slice_h / 2; // DH: the span's middle

        // Per-column vote: the datapoint covering sampleY (air = none).
        let mut votes: [Option<&SliceRun>; 4] = [None; 4];
        for (i, col) in cols.iter().enumerate() {
            while idx[i] >= 0 {
                let (bottom, top) = span(col, idx[i] as usize);
                if sample_y >= top {
                    // sample above this datapoint -> next (lower) one
                    idx[i] -= 1;
                } else if sample_y >= bottom && sample_y < top {
                    votes[i] = Some(&col[idx[i] as usize]);
                    break;
                } else {
                    // sample below this datapoint
                    break;
                }
            }
        }

        // determineMostCommonValueInColumnSlice: count occurrences per value,
        // air skipped entirely; ties resolve to the EARLIEST column (value0
        // wins whenever count0 == maxCount, checked first in DH).
        let mut best: Option<&SliceRun> = None;
        let mut best_count = 0usize;
        for (i, v) in votes.iter().enumerate() {
            let Some(v) = v else { continue }; // air: "always overwrite air"
            if votes[..i].iter().any(|p| p.is_some_and(|r| same_run(r, v))) {
                continue; // same value already counted at its first slot
            }
            let count = votes
                .iter()
                .filter(|p| p.is_some_and(|r| same_run(r, v)))
                .count();
            if count > best_count {
                best_count = count;
                best = Some(v);
            }
            // a tie keeps the earlier winner (best stays)
        }
        let Some(win) = best else {
            acc = None; // an all-air span breaks any accumulated run
            continue;
        };

        // Adjacent spans with equal data extend the accumulated datapoint
        // (DH: accumulatedHeight += sliceHeight); the run's top boundary is
        // where the datapoint started.
        match acc {
            Some(ref a) if same_run(a, win) => {}
            _ => {
                if let Some(a) = acc {
                    out.push(a);
                }
                acc = Some(*win);
            }
        }
    }
    if let Some(a) = acc {
        out.push(a);
    }
    // Spans were processed bottom-up; DH's ensureDataColumnOrder keeps the
    // same bottom->top order internally, but our lists are TOP-DOWN.
    out.reverse();
    out
}

/// Bound the merged parent list before the next merge (see module docs:
/// DH keeps unreduced full data; we shrink with the DH reducer itself).
pub const RAW_CAP: usize = 24;

/// DH `RenderDataPointReducingList.reduce(target)` on a top-down list.
/// All our runs are visible (alpha 255) — air nodes never exist in our lists
/// (the scan merges across air), so the populate/air machinery collapses
/// away and `sizeWithoutAir == sizeWithAir`.
#[must_use]
pub fn reduce(list: &[SliceRun], target: usize) -> Vec<SliceRun> {
    if list.len() <= target {
        return list.to_vec();
    }
    // Nodes as (bottom, top_exclusive, run), lowest->highest (DH populate's
    // position order). Bottoms: the next run's top+1; the LAST run's bottom
    // is the world floor (half i32::MIN — see `span`).
    let mut nodes: Vec<(i64, i64, SliceRun)> = list
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let top = i64::from(r.top) + 1;
            let bottom = list
                .get(i + 1)
                .map_or(i64::from(i32::MIN) / 2, |below| i64::from(below.top) + 1);
            (bottom, top, *r)
        })
        .collect();
    // DH populate sorts by position (lowest->highest); our input lists are
    // top-down, so sort before the reduction walks the size order.
    nodes.sort_by_key(|&(bottom, _, _)| bottom);

    let size = |n: &(i64, i64, SliceRun)| n.1 - n.0;

    // Step 0+1: repeatedly merge the smallest segment into a neighbour.
    while nodes.len() > target {
        // smallest node; ties keep the LOWEST one (DH's stable size order
        // sees lower-Y nodes first — they were inserted first in populate).
        let mut si = 0usize;
        for i in 1..nodes.len() {
            if size(&nodes[i]) < size(&nodes[si]) {
                si = i;
            }
        }
        let can_lower = si > 0;
        let can_higher = si + 1 < nodes.len();
        if can_lower && can_higher {
            // both neighbours are equal-alpha (all visible) -> merge with
            // the SMALLER one (1b); equal size -> the higher one (1b1).
            let s_lower = size(&nodes[si - 1]);
            let s_higher = size(&nodes[si + 1]);
            if s_higher <= s_lower {
                // survivor = the higher neighbour, extended DOWN over si
                let bottom = nodes[si].0;
                nodes[si + 1].0 = bottom;
                nodes.remove(si);
            } else {
                // survivor = si, extended DOWN over the lower neighbour
                let bottom = nodes[si - 1].0;
                nodes[si].0 = bottom;
                nodes.remove(si - 1);
            }
        } else if can_lower {
            let bottom = nodes[si - 1].0;
            nodes[si].0 = bottom;
            nodes.remove(si - 1);
        } else if can_higher {
            let bottom = nodes[si].0;
            nodes[si + 1].0 = bottom;
            nodes.remove(si);
        } else {
            break; // single node left
        }
    }

    // Step 2 (removeLeastImportantSegments): erase a segment whose lower and
    // higher neighbours are equal to EACH OTHER (the outer pair extends).
    while nodes.len() > target {
        let mut si = 0usize;
        for i in 1..nodes.len() {
            if size(&nodes[i]) < size(&nodes[si]) {
                si = i;
            }
        }
        if si > 0 && si + 1 < nodes.len() && same_run(&nodes[si - 1].2, &nodes[si + 1].2) {
            let bottom = nodes[si - 1].0;
            nodes[si + 1].0 = bottom;
            nodes.remove(si + 1);
            nodes.remove(si);
        } else {
            break;
        }
    }

    // Step 3 (forceBottomToMerge): the lowest node merges upward, no
    // restrictions, until the target is met.
    while nodes.len() > target && nodes.len() > 1 {
        let bottom = nodes[0].0;
        nodes[1].0 = bottom;
        nodes.remove(0);
    }

    // Back to top-down runs: top = maxY - 1 (block Y of the top block).
    // Spans are disjoint, so the tops strictly descend by construction.
    nodes
        .into_iter()
        .rev()
        .map(|(_, max_y, r)| SliceRun {
            top: (max_y - 1) as i16,
            ..r
        })
        .collect()
}

/// DH `RenderDataPointReducingList.reduceToOne`: one datapoint with the
/// HIGHEST datapoint's data and the LOWEST datapoint's bottom — the
/// heightmap projection (verified equal to the pyramid winner's surface).
/// (Heightmap projections go through `atlas::pyramid_winners`, whose strict
/// first-max rule is the verified projection of this DH function onto our
/// per-cell heights; kept for the DH-source 1:1 audit and unit tests.)
#[cfg_attr(not(test), allow(dead_code))]
#[must_use]
pub fn reduce_to_one(list: &[SliceRun]) -> Option<SliceRun> {
    let mut top_run: Option<&SliceRun> = None;
    for r in list {
        if top_run.is_none_or(|t| r.top > t.top) {
            top_run = Some(r);
        }
    }
    top_run.copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(top: i16, color: u32) -> SliceRun {
        SliceRun {
            top,
            color,
            tex_top: 0,
            tex_side: 0,
            liquid: false,
            grass: false,
        }
    }

    /// grass(1 tall, top 5), dirt(2 tall, top 3), stone(1 tall, top 1).
    fn classic() -> Vec<SliceRun> {
        vec![run(5, 0x00FF00), run(3, 0x8B4513), run(1, 0x808080)]
    }

    #[test]
    fn medium_table_matches_dh() {
        assert_eq!(MEDIUM, [6, 6, 6, 4, 4, 4, 4, 4, 4, 4, 1]);
        assert_eq!(slices_at_detail(0), 6);
        assert_eq!(slices_at_detail(2), 6);
        assert_eq!(slices_at_detail(3), 4);
        assert_eq!(slices_at_detail(10), 1);
        assert_eq!(slices_at_detail(99), 1, "clamped to the last entry");
        assert_eq!(slices_at_detail(-3), 6, "clamped to the first entry");
    }

    #[test]
    fn merge_votes_the_middle_and_air_loses() {
        // stone, air, grass, stone: the shared span votes stone (2) vs
        // grass (1); air (the empty column) never counts.
        let c0 = vec![run(5, 0x808080)];
        let c2 = vec![run(5, 0x00FF00)];
        let c3 = vec![run(5, 0x808080)];
        let merged = merge_two_by_two([&c0, &[], &c2, &c3]);
        assert_eq!(merged.len(), 1, "one run wins the whole column");
        assert_eq!(merged[0].color, 0x808080, "stone (value0) beats grass 2:1");
    }

    #[test]
    fn merge_tie_resolves_to_the_first_column() {
        let a = vec![run(5, 0x0000FF)];
        let b = vec![run(5, 0xFF0000)];
        let merged = merge_two_by_two([&a, &b, &[], &[]]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].color, 0x0000FF, "1-1 tie -> value0");
    }

    #[test]
    fn merge_equal_children_reproduce_the_runs() {
        let c = classic();
        let merged = merge_two_by_two([&c, &c, &c, &c]);
        assert_eq!(merged, classic());
    }

    #[test]
    fn merge_top_band_follows_the_tallest_child() {
        // 3 fields at 5 + one tower at 9: the shared span below 6 votes
        // grass 3:1; the tower's own span [6..10] votes stone; the boundary
        // between the two bands is the field top (the tower's deep bottom
        // loses every vote under the fields).
        let f = vec![run(5, 0x00FF00)];
        let t = vec![run(9, 0x808080)];
        let merged = merge_two_by_two([&f, &f, &f, &t]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].color, 0x808080, "the tower keeps its own band");
        assert_eq!(merged[0].top, 9);
        assert_eq!(merged[1].color, 0x00FF00, "grass wins the shared span");
        assert_eq!(merged[1].top, 5);
    }

    #[test]
    fn merge_deep_spans_follow_the_deepest_band() {
        // The last run reaches the world floor: deep spans below the other
        // children's tops vote too — and the MAJORITY rule sends every span
        // under the field tops to grass (3 columns vs the tower's deep run),
        // which is exactly DH's air/field vote behaviour.
        let a = vec![run(5, 0x00FF00)];
        let b = vec![run(9, 0x808080), run(1, 0x404040)];
        let merged = merge_two_by_two([&a, &a, &a, &b]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].color, 0x808080, "tower's own span [6..10]");
        assert_eq!(merged[0].top, 9);
        assert_eq!(
            merged[1].color, 0x00FF00,
            "everything below votes grass 3:1"
        );
        assert_eq!(merged[1].top, 5);
    }

    #[test]
    fn reduce_step1_keeps_survivor_colour() {
        // classic(): smallest = grass (1 tall) with no higher neighbour ->
        // extends DOWN over dirt (2 tall): grass [2..6]; the list is now at
        // the target (2) and reduce STOPS (DH: stop as soon as the size fits).
        let out = reduce(&classic(), 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].color, 0x00FF00, "grass survived, kept its colour");
        assert_eq!(out[0].top, 5);
        assert_eq!(out[1].color, 0x808080, "stone untouched below");
        assert_eq!(out[1].top, 1);
    }

    #[test]
    fn reduce_step1_prefers_the_smaller_neighbour() {
        // grass(3 tall), dirt(1 tall), stone(1 tall), target 2:
        // smallest tie dirt/stone -> dirt (lower first): neighbours grass(3)
        // and stone(1) -> the SMALLER neighbour is stone -> survivor stone
        // extends down over dirt: stone [3..4]. Then grass(3) vs stone(1):
        // stone merges up into grass -> grass [..6]. 2 nodes: grass top 5 +
        // stone top 1.
        let list = vec![run(5, 0x00FF00), run(3, 0x8B4513), run(1, 0x808080)];
        let out = reduce(&list, 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].color, 0x00FF00, "top band = grass");
        assert_eq!(out[1].color, 0x808080, "deep band = stone");
    }

    #[test]
    fn reduce_force_bottom_merges_upward() {
        let list = vec![run(7, 1), run(5, 2), run(3, 3), run(1, 4)];
        let out = reduce(&list, 2);
        assert_eq!(out.len(), 2);
        assert!(out[0].top > out[1].top);
        assert_eq!(out[0].top, 7, "the top band keeps its top");
    }

    #[test]
    fn reduce_noop_under_target() {
        let c = classic();
        let out = reduce(&c, 6);
        assert_eq!(out, c);
    }

    #[test]
    fn reduce_tops_strictly_descend() {
        let list: Vec<SliceRun> = (0..10)
            .map(|i| run((20 - i * 2) as i16, (i + 1) as u32))
            .collect();
        let out = reduce(&list, 4);
        assert_eq!(out.len(), 4);
        for w in 1..out.len() {
            assert!(out[w - 1].top > out[w].top, "tops must strictly descend");
        }
    }

    #[test]
    fn reduce_single_band_bottom_reaches_floor() {
        // One deep band + a thin cap, target 1: the cap wins (higher), the
        // deep bottom merges up into it (forceBottom).
        let list = vec![run(9, 0x00FF00), run(1, 0x808080)];
        let out = reduce(&list, 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].color, 0x00FF00);
    }

    #[test]
    fn reduce_to_one_takes_highest_data() {
        let c = classic();
        let one = reduce_to_one(&c).unwrap();
        assert_eq!(one.color, 0x00FF00, "highest datapoint's colour");
        assert_eq!(one.top, 5);
    }
}
