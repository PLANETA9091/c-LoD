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
//!     winners merge into one datapoint whose TOP and BOTTOM come from the
//!     ACCUMULATED SPANS (not from the winning child's own datapoint — a
//!     child can keep covering spans it loses above, so its private
//!     top/bottom would overlap the neighbours).
//!   - `util/RenderDataPointReducingList.java`: the render-time reducer.
//!     `populate` collects the visible datapoints (alpha >= 16), sorts them
//!     by Y and inserts an AIR NODE (alpha 0) between every pair of
//!     non-touching neighbours — cave gaps are first-class nodes. `reduce`
//!     then runs step 0+1 (merge the smallest segment into a SAME-ALPHA
//!     neighbour — both match -> the SMALLER neighbour, equal size -> the
//!     higher one; the survivor keeps its own colour and extends over the
//!     removed node; air nodes never merge in this step), with a fast path
//!     for segments of size <= SPECIAL_CASES = 2 that walks the size chain
//!     in reverse without re-sorting, then step 2 (erase a segment entirely
//!     when its lower and higher neighbours are equal-alpha to EACH OTHER —
//!     this is how cave air between two solids gets filled, by the UPPER
//!     node's data), then step 3 (force the lowest segment to merge upward
//!     through invisible nodes — `lowY` is kept while invisible nodes are
//!     removed) until `sizeWithoutAir <= target`. `copyTo` emits only the
//!     VISIBLE nodes: a surviving air node is a hole in the column.
//!   - `api/enums/config/EDhApiVerticalQuality.java`: MEDIUM (DH default)
//!     = {6,6,6,4,4,4,4,4,4,4,1} indexed by the datapoint detail level.
//!
//! c-LoD mapping: one atlas cell at octave L is a datapoint of width
//! `cell_size * 2^L` blocks, so its detail level is `log2(cell_size) + L`
//! and the slice target is `MEDIUM[detail]`. The scan produces the level-0
//! run list (top-down, cut on Material change AND on air — DH cuts on
//! blockstate change; Material granularity is the same at LOD resolution);
//! every octave up merges 2x2 with the vote rule and encodes `reduce(list,
//! MEDIUM[d])`.
//!
//! The LAST run of every column reaches the world bottom in DH (full-data
//! columns span to min_y); `span` always maps the last run to the world
//! floor regardless of its stored bottom, and the shader does the same for
//! the deepest band present in the strip.
//!
//! Air gaps between runs are REAL here: the scan cuts bands on air, the
//! runs carry explicit `bottom`s, and the renderer treats a wall hit inside
//! a gap as no-hit (the ray flies through the cave mouth exactly like DH's
//! missing wall face).
//!
//! LIGHT (v4.11.0, `LodDataBuilder.createFromChunk` lines 157-260):
//! every datapoint carries `(blockLight, skyLight)` = the light AT the block
//! ABOVE its top (`getDhBlockLight/getDhSkyLight(x, y+1, z)`); the air column
//! above the surface is hardcoded (0, 15). The 2x2 merge averages the light
//! of the sub-columns that voted FOR the winner
//! (`determineAverageValueInColumnSliceWithId` — light from an outvoted
//! block must not bleed in) and SPLITS the accumulated datapoint when the
//! averaged light changes (`id != lastId || blockLight != last || skyLight
//! != last`). The reducer's light travels with the node untouched. The
//! byte packs `sky << 4 | block` (the terrain vertex convention: sky =
//! meta/16, block = meta%16). The shader samples the vanilla lightmap
//! curve at (sky, block) — enclosed caves read (0,0), day-independent.

/// One vertical run of a column: solid blocks `[bottom, top]` (the top
/// block's Y; the atlas surface plane convention keeps the top face at
/// `height`), rendered flat in `color` (DH: ONE colour per datapoint — the
/// block's first non-empty face average, biome tinted), textured near-field
/// by `tex_top`/`tex_side` ratio tiles. Air between this run's `bottom` and
/// the next run's top is a cave gap (DH's air datapoint).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceRun {
    /// Block Y of the run's topmost block.
    pub top: i16,
    /// Block Y of the run's lowest solid block. The sentinel `i16::MIN`
    /// means "unset — derive from the next run's top" (scan lists before
    /// the reducer; the encoder resolves it before packing).
    pub bottom: i16,
    /// Flat datapoint colour (0xRRGGBB).
    pub color: u32,
    pub tex_top: u16,
    pub tex_side: u16,
    /// Liquid run (water): the band sits under the water surface plane.
    pub liquid: bool,
    /// DH LodQuadBuilder FADE_TO_DIRT: grass-material walls fade to the dirt
    /// colour across the band (EDhApiBlockMaterial.GRASS).
    pub grass: bool,
    /// DH datapoint light: the light at the block ABOVE the run's top —
    /// packed `sky << 4 | block` (0xF0 = open sky, 0x00 = enclosed cave).
    pub light: u8,
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

/// DH datapoint span of a run inside a top-down list: (bottom, top_exclusive).
/// An explicit `bottom` carries the AIR GAP below the run (bottom greater
/// than the next run's top + 1); the sentinel derives the old tiling; the
/// LAST run always reaches the world floor (`-inf` semantics: half
/// `i32::MIN` keeps every subtraction in i64 range without overflow).
fn span(col: &[SliceRun], i: usize) -> (i64, i64) {
    let top = i64::from(col[i].top) + 1;
    let bottom = if i + 1 == col.len() {
        i64::from(i32::MIN) / 2
    } else if col[i].bottom != i16::MIN {
        i64::from(col[i].bottom)
    } else {
        i64::from(col[i + 1].top) + 1
    };
    (bottom, top)
}

/// Block-Y clamps for span coordinates coming out of the merge/reduce.
/// Tops live in `[-64, i16::MAX]`; bottoms below the world floor become the
/// "unset" sentinel (`span` re-derives the floor for the last run).
fn clamp_top(v: i64) -> i16 {
    v.clamp(-64, i64::from(i16::MAX)) as i16
}

fn clamp_bottom(v: i64) -> i16 {
    if v < -65 {
        i16::MIN
    } else {
        v.clamp(-65, i64::from(i16::MAX)) as i16
    }
}

/// DH `FullDataSourceV2.mergeInputTwoByTwoDataColumn`, on top-down lists.
/// Any child may be empty (void/un-scanned — DH's null column votes nothing,
/// i.e. air); a span that falls into a child's AIR GAP also votes nothing.
/// Returns the merged list top-down.
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
    // The accumulated datapoint's block-Y bounds (DH: accumulatedHeight +=
    // sliceHeight; minY/maxY come from the SPANS, not from the winning
    // child's own datapoint, which may extend over spans it lost).
    let mut acc_top = 0i64;
    let mut acc_bottom = 0i64;
    // The accumulated datapoint's AVERAGED light (DH carries lastBlockLight
    // / lastSkyLight through the same loop).
    let mut acc_light = 0u8;

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
                    // sample below this datapoint (inside its air gap)
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
            // an all-air span CLOSES the accumulated datapoint (it still
            // ships — DH's accumulator keeps its data until a different
            // winner replaces it) and breaks the run.
            if let Some(a) = acc {
                out.push(SliceRun {
                    top: clamp_top(acc_top),
                    bottom: clamp_bottom(acc_bottom),
                    light: acc_light,
                    ..a
                });
            }
            acc = None;
            continue;
        };

        // determineAverageValueInColumnSliceWithId: average the light of the
        // sub-columns that voted FOR the winning id (outvoted light must not
        // bleed — "surface water" brightening the sand below); when no voter
        // matches (all four different) DH falls back to averaging all four.
        // Integer floor division per channel, exactly like DH's int math.
        let mut sky_sum = 0u32;
        let mut block_sum = 0u32;
        let mut n = 0u32;
        for v in votes.iter().flatten() {
            if same_run(v, win) {
                sky_sum += u32::from(v.light >> 4);
                block_sum += u32::from(v.light & 0xF);
                n += 1;
            }
        }
        if n == 0 {
            for v in votes.iter().flatten() {
                sky_sum += u32::from(v.light >> 4);
                block_sum += u32::from(v.light & 0xF);
                n += 1;
            }
        }
        let win_light = (((sky_sum / n) << 4) | (block_sum / n)) as u8;

        // Adjacent spans with equal data extend the accumulated datapoint;
        // its bounds ride the SPANS (see the module docs for why the child's
        // own top/bottom would be wrong here). A light change CLOSES the
        // accumulated datapoint (DH: id != lastId || blockLight != last ||
        // skyLight != last -> new datapoint) — cave light transitions stay
        // banded after the merge.
        let span_top_block = slice_max - 1;
        match acc {
            Some(ref a) if same_run(a, win) && a.light == win_light => {
                acc_top = span_top_block;
            }
            _ => {
                if let Some(a) = acc {
                    out.push(SliceRun {
                        top: clamp_top(acc_top),
                        bottom: clamp_bottom(acc_bottom),
                        light: acc_light,
                        ..a
                    });
                }
                acc = Some(SliceRun {
                    light: win_light,
                    ..*win
                });
                acc_light = win_light;
                acc_top = span_top_block;
                acc_bottom = slice_min;
            }
        }
    }
    if let Some(a) = acc {
        out.push(SliceRun {
            top: clamp_top(acc_top),
            bottom: clamp_bottom(acc_bottom),
            light: acc_light,
            ..a
        });
    }
    // Spans were processed bottom-up; DH's ensureDataColumnOrder keeps the
    // same bottom->top order internally, but our lists are TOP-DOWN.
    out.reverse();
    out
}

/// Bound the merged parent list before the next merge (see module docs:
/// DH keeps unreduced full data; we shrink with the DH reducer itself).
pub const RAW_CAP: usize = 24;

/// One reducer node: a span plus its datapoint (`None` = DH's inserted air
/// node, alpha 0). Nodes stay disjoint and bottom-sorted; `dead` marks
/// removed nodes until compaction (the fast path's size-order snapshot
/// references positions across removals, like DH's linked lists).
struct RNode {
    bot: i64,
    top: i64,
    run: Option<SliceRun>,
    dead: bool,
}

/// DH's `SPECIAL_CASES`: segments this small go through the reverse-walk
/// fast path (`mergeVerySmallConnectedSegments`) before the general loop.
const SPECIAL_CASES: usize = 2;

/// DH `RenderDataPointReducingList.reduce(target)` on a top-down list,
/// including the air nodes `populate` inserts between non-touching
/// datapoints (cave gaps): step 0+1 merges same-alpha neighbours only, so
/// solid runs never merge ACROSS a gap in this step; step 2 may erase a gap
/// (filling it with the UPPER node's data) when the neighbours are
/// equal-alpha to each other; step 3 forces the bottom-most node upward
/// through invisible nodes.
#[must_use]
pub fn reduce(list: &[SliceRun], target: usize) -> Vec<SliceRun> {
    if list.len() <= target {
        return list.to_vec();
    }

    // -- populate: visible nodes sorted by position, air nodes inserted --
    let mut solid: Vec<RNode> = Vec::with_capacity(list.len());
    for (i, r) in list.iter().enumerate() {
        let (bot, top) = span(list, i);
        if top > bot {
            solid.push(RNode {
                bot,
                top,
                run: Some(*r),
                dead: false,
            });
        }
    }
    solid.sort_unstable_by_key(|n| n.bot);
    let mut nodes: Vec<RNode> = Vec::with_capacity(solid.len() * 2);
    for (w, n) in solid.iter().enumerate() {
        if w > 0 {
            let prev = &solid[w - 1];
            if prev.top < n.bot {
                // populate's air node: [lowerMaxY, higherMinY], alpha 0
                nodes.push(RNode {
                    bot: prev.top,
                    top: n.bot,
                    run: None,
                    dead: false,
                });
            }
        }
        nodes.push(RNode {
            bot: n.bot,
            top: n.top,
            run: n.run,
            dead: false,
        });
    }
    drop(solid);

    let visible = |nodes: &[RNode]| nodes.iter().filter(|n| !n.dead && n.run.is_some()).count();
    let size = |n: &RNode| n.top - n.bot;
    // Size order of the alive nodes; ties keep the LOWEST-Y node (the
    // documented deterministic projection of DH's size sort).
    let order = |nodes: &[RNode]| -> Vec<usize> {
        let mut idx: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].dead).collect();
        idx.sort_unstable_by(|&a, &b| {
            size(&nodes[a])
                .cmp(&size(&nodes[b]))
                .then(nodes[a].bot.cmp(&nodes[b].bot))
        });
        idx
    };

    // tryMergeStep1: merge the node at alive-position `p` into a same-alpha
    // Y-neighbour; the survivor keeps its own data and extends DOWN over the
    // removed node. Returns false when neither neighbour shares the alpha
    // (air blocks the merge — DH 1c).
    fn step1_merge(nodes: &mut [RNode], alive: &[usize], ci: usize) -> bool {
        let node_size = |i: usize| nodes[i].top - nodes[i].bot;
        let Some(p) = alive.iter().position(|&i| i == ci) else {
            return false;
        };
        let lower = if p > 0 { Some(alive[p - 1]) } else { None };
        let higher = alive.get(p + 1).copied();
        let cur_solid = nodes[ci].run.is_some();
        // 1b: both match -> the SMALLER neighbour, equal size -> the higher.
        // 1a/1c: the single matching side wins; none -> no merge.
        let (survivor, removed) = if higher.is_some_and(|i| nodes[i].run.is_some() == cur_solid)
            && lower.is_some_and(|i| nodes[i].run.is_some() == cur_solid)
        {
            if node_size(higher.unwrap()) <= node_size(lower.unwrap()) {
                (higher.unwrap(), ci)
            } else {
                (ci, lower.unwrap())
            }
        } else if higher.is_some_and(|i| nodes[i].run.is_some() == cur_solid) {
            (higher.unwrap(), ci)
        } else if lower.is_some_and(|i| nodes[i].run.is_some() == cur_solid) {
            (ci, lower.unwrap())
        } else {
            return false;
        };
        nodes[survivor].bot = nodes[removed].bot;
        nodes[removed].dead = true;
        true
    }

    // -- step 0+1, fast path: sizes 1..=SPECIAL_CASES, reverse walk, no
    //    re-sort inside a round (DH mergeVerySmallConnectedSegments) --
    'fast: for special_case in 1..=SPECIAL_CASES {
        // Snapshot in DESCENDING size order; ties walk lowest-Y first (the
        // reverse of an ascending build whose ties order bot DESCENDING).
        let mut visit: Vec<usize> = (0..nodes.len())
            .filter(|&i| !nodes[i].dead && size(&nodes[i]) <= special_case as i64)
            .collect();
        visit.sort_unstable_by(|&a, &b| {
            size(&nodes[b])
                .cmp(&size(&nodes[a]))
                .then(nodes[a].bot.cmp(&nodes[b].bot))
        });
        for &ci in &visit {
            if nodes[ci].dead {
                continue; // removed earlier in this round
            }
            if visible(&nodes) <= target {
                break 'fast; // done — DH returns before steps 2/3
            }
            let alive: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].dead).collect();
            step1_merge(&mut nodes, &alive, ci);
        }
    }
    nodes.retain(|n| !n.dead);

    // -- step 0+1, general path: smallest-first, restart after every merge --
    while visible(&nodes) > target {
        let ord = order(&nodes);
        // Y-neighbours come from the BOTTOM-SORTED alive list (nodes stays
        // bottom-sorted), never from the size order.
        let alive: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].dead).collect();
        let mut merged = false;
        for &ci in &ord {
            if nodes[ci].dead {
                continue;
            }
            if step1_merge(&mut nodes, &alive, ci) {
                merged = true;
                break; // DH: restart from the smallest
            }
        }
        if !merged {
            break; // nothing mergeable -> step 2
        }
        nodes.retain(|n| !n.dead);
    }
    if visible(&nodes) <= target {
        return finish(nodes);
    }

    // -- step 2 (removeLeastImportantSegments): erase the smallest segment
    //    whose lower and higher neighbours are equal-alpha to EACH OTHER
    //    (any data); the HIGHER neighbour extends over both and keeps its
    //    own data. For our lists this is the step that fills cave air with
    //    the upper band's material when the target forces it. --
    while visible(&nodes) > target {
        let ord = order(&nodes);
        let alive: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].dead).collect();
        let mut erased = false;
        for &ci in &ord {
            if nodes[ci].dead {
                continue;
            }
            let Some(p) = alive.iter().position(|&i| i == ci) else {
                continue;
            };
            let (Some(lo), Some(hi)) = (
                p.checked_sub(1).map(|q| alive[q]),
                alive.get(p + 1).copied(),
            ) else {
                continue; // needs both neighbours
            };
            if nodes[lo].run.is_some() == nodes[hi].run.is_some() {
                nodes[hi].bot = nodes[lo].bot;
                nodes[lo].dead = true;
                nodes[ci].dead = true;
                erased = true;
                break; // DH: restart from the smallest
            }
        }
        if !erased {
            break; // -> step 3
        }
        nodes.retain(|n| !n.dead);
    }
    if visible(&nodes) <= target {
        return finish(nodes);
    }

    // -- step 3 (forceBottomToMerge): the bottom-most node merges upward
    //    through invisible nodes (lowY is kept); the first visible node
    //    above extends down and keeps its own data. Our lists never end on
    //    a gap (the floor run reaches down), so the invisible-walk arm is a
    //    defensive port; the no-higher arm clears the column (target 0). --
    while visible(&nodes) > target {
        let alive: Vec<usize> = (0..nodes.len()).filter(|&i| !nodes[i].dead).collect();
        let Some(&lowest) = alive.first() else { break };
        let low_y = nodes[lowest].bot;
        let mut cur = lowest;
        while let Some(p) = alive.iter().position(|&i| i == cur) {
            let Some(&hi) = alive.get(p + 1) else {
                // no node above: target is 0 — clear the column (DH)
                for n in nodes.iter_mut() {
                    n.dead = true;
                }
                break;
            };
            if nodes[hi].run.is_some() {
                nodes[hi].bot = low_y;
                nodes[cur].dead = true;
                break;
            }
            // invisible: eliminate it too, keep walking up (lowY unchanged)
            nodes[cur].dead = true;
            cur = hi;
        }
        nodes.retain(|n| !n.dead);
    }

    finish(nodes)
}

/// The reduced list: visible nodes only, top-down (DH `copyTo` — surviving
/// air nodes are simply dropped, their span stays a hole between runs).
fn finish(nodes: Vec<RNode>) -> Vec<SliceRun> {
    let mut out: Vec<SliceRun> = nodes
        .into_iter()
        .filter(|n| !n.dead)
        .filter_map(|n| {
            let r = n.run?;
            Some(SliceRun {
                top: clamp_top(n.top - 1),
                bottom: clamp_bottom(n.bot),
                ..r
            })
        })
        .collect();
    // nodes are bottom-sorted; our lists are top-down
    out.reverse();
    out
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
            bottom: i16::MIN,
            color,
            tex_top: 0,
            tex_side: 0,
            liquid: false,
            grass: false,
            light: 0xF0, // open sky
        }
    }

    /// grass(1 tall, top 5), dirt(2 tall, top 3), stone(1 tall, top 1).
    fn classic() -> Vec<SliceRun> {
        vec![run(5, 0x00FF00), run(3, 0x8B4513), run(1, 0x808080)]
    }

    /// Overhang column: grass cap [4..5], air gap [1..4), stone [..1].
    fn overhang() -> Vec<SliceRun> {
        vec![
            SliceRun {
                bottom: 4,
                ..run(5, 0x00FF00)
            },
            SliceRun {
                bottom: -65,
                ..run(1, 0x808080)
            },
        ]
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
    fn span_honours_explicit_bottoms_and_floor() {
        let col = overhang();
        // cap: explicit bottom 4 (the air gap [1..4) below it)
        assert_eq!(span(&col, 0), (4, 6));
        // last run reaches the world floor regardless of its stored bottom
        assert_eq!(span(&col, 1), (i64::from(i32::MIN) / 2, 2));
        // unset bottoms derive from the next run's top (old tiling)
        let c = classic();
        assert_eq!(span(&c, 0), (4, 6));
        assert_eq!(span(&c, 1), (2, 4));
        assert_eq!(span(&c, 2), (i64::from(i32::MIN) / 2, 2));
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
        // The merged datapoints carry the SPAN-derived bottoms (the old
        // tiling: bottom = next top + 1; the deepest reaches the floor).
        let expected = vec![
            SliceRun {
                bottom: 4,
                ..run(5, 0x00FF00)
            },
            SliceRun {
                bottom: 2,
                ..run(3, 0x8B4513)
            },
            run(1, 0x808080),
        ];
        assert_eq!(merged, expected);
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
    fn merge_accumulated_bounds_come_from_the_spans() {
        // The DH rule the naive port misses: a merged datapoint's top rides
        // the ACCUMULATED SPANS, not the winning child's own datapoint.
        //   col0: grass top 9 (tall — also covers the span it loses above)
        //   col1/2: stone top 11
        //   col3: grass top 5
        // Span [-inf..6): grass 2:2 tie -> value0 grass (the TALL child).
        // Span [6..10): stone 2:1. The grass datapoint must end at the
        // span top (5) even though its child's private top is 9 — otherwise
        // it overlaps the stone band and corrupts the next merge's spans.
        let a = vec![run(9, 0x00FF00)];
        let b = vec![run(11, 0x808080)];
        let c = vec![run(11, 0x808080)];
        let d = vec![run(5, 0x00FF00)];
        let merged = merge_two_by_two([&a, &b, &c, &d]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].color, 0x808080);
        assert_eq!(merged[0].top, 11);
        assert_eq!(merged[1].color, 0x00FF00);
        assert_eq!(merged[1].top, 5, "accumulated top, not the child's 9");
    }

    #[test]
    fn merge_air_gap_span_breaks_the_accumulator_and_abstains() {
        // Two bands with a cave gap, all four children identical: the gap
        // span votes nothing (every column abstains) and breaks the
        // accumulator — the merged column keeps both bands with the
        // explicit gap bottom intact.
        let c = vec![
            SliceRun {
                bottom: 8,
                ..run(9, 0x00FF00)
            },
            SliceRun {
                bottom: -65,
                ..run(1, 0x808080)
            },
        ];
        let merged = merge_two_by_two([&c, &c, &c, &c]);
        assert_eq!(merged.len(), 2, "the gap survives the merge (all-air span)");
        assert_eq!(merged[0].top, 9);
        assert_eq!(merged[0].bottom, 8, "explicit gap bottom rides along");
        assert_eq!(merged[1].top, 1);
        assert_eq!(merged[1].bottom, i16::MIN, "floor bottom -> sentinel");
    }

    #[test]
    fn reduce_step1_keeps_survivor_colour() {
        // classic() at target 2: smallest = grass (1 tall), no higher
        // neighbour -> extends DOWN over dirt (2 tall): grass [2..6]; the
        // target is met and reduce STOPS (DH: stop as soon as the size fits).
        let out = reduce(&classic(), 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].color, 0x00FF00, "grass survived, kept its colour");
        assert_eq!(out[0].top, 5);
        assert_eq!(out[0].bottom, 2, "grass extended down over dirt");
        assert_eq!(out[1].color, 0x808080, "stone untouched below");
        assert_eq!(out[1].top, 1);
    }

    #[test]
    fn reduce_step1_never_merges_across_a_gap() {
        // overhang() at target 2: the list is already at the target and
        // steps 0+1 must not fill the gap even implicitly (air nodes block
        // same-alpha merges).
        let out = reduce(&overhang(), 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].color, 0x00FF00);
        assert_eq!(out[0].bottom, 4, "the cave gap stays open");
        assert_eq!(out[1].color, 0x808080);
        assert_eq!(out[1].top, 1);
    }

    #[test]
    fn reduce_step2_fills_a_gap_with_the_upper_band() {
        // overhang() at target 1: steps 0+1 can't merge (air blocks both
        // ends); step 2 walks the size order — the AIR node ([2..4), tied
        // with the cap but lower) comes first, its lower/higher are both
        // solid (equal alpha to EACH OTHER) -> erased, and the UPPER band
        // extends over the stone's span. DH fills caves from the top.
        let out = reduce(&overhang(), 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].color, 0x00FF00, "the UPPER band fills the gap");
        assert_eq!(out[0].top, 5);
        assert_eq!(
            out[0].bottom,
            i16::MIN,
            "stone's floor span rides up (last run -> world floor)"
        );
    }

    #[test]
    fn reduce_step1b_prefers_the_smaller_neighbour() {
        // X (2 tall [6..8)) is smallest-tied with M (2 tall [4..6)); the
        // reverse fast-path walk visits the LOWEST-Y tie first (M): both of
        // M's neighbours are solid; the HIGHER (X, 2 tall) <= the LOWER
        // (stone, floor-tall) -> the higher survives and extends down.
        let list = vec![
            SliceRun {
                bottom: 6,
                ..run(7, 0x0000FF)
            },
            SliceRun {
                bottom: 4,
                ..run(5, 0x00FF00)
            },
            run(3, 0x808080),
        ];
        let out = reduce(&list, 2);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].color, 0x0000FF, "the higher neighbour survived");
        assert_eq!(out[0].top, 7);
        assert_eq!(out[0].bottom, 4, "extended down over M's span");
        assert_eq!(out[1].color, 0x808080, "stone untouched");
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
        // One deep band + a thin cap, target 1: the cap's lower neighbour is
        // the only same-alpha match -> the survivor is the HIGHER node (the
        // cap), which extends down over the deep band's span — and the deep
        // band IS the floor run, so the survivor reaches the world floor.
        let list = vec![run(9, 0x00FF00), run(1, 0x808080)];
        let out = reduce(&list, 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].color, 0x00FF00);
        assert_eq!(
            out[0].bottom,
            i16::MIN,
            "the merged floor span rides up (last run -> world floor)"
        );
    }

    #[test]
    fn reduce_target_zero_clears_the_column() {
        // DH forceBottomToMerge: no node above the lowest -> target is 0 ->
        // the whole column clears (copyTo then emits a void datapoint).
        let out = reduce(&classic(), 0);
        assert!(out.is_empty());
    }

    #[test]
    fn reduce_to_one_takes_highest_data() {
        let c = classic();
        let one = reduce_to_one(&c).unwrap();
        assert_eq!(one.color, 0x00FF00, "highest datapoint's colour");
        assert_eq!(one.top, 5);
    }

    // ------------------------------------------------------------------
    // LIGHT (v4.11.0): FullDataSourceV2 lines 887-918 + lines 901-903.
    // ------------------------------------------------------------------

    #[test]
    fn merge_averages_light_from_winners_only() {
        // Surface water (dark column) next to sand (lit column): the shared
        // span votes 2 vs 1 (stone wins over dirt below) — light must come
        // from the two STONE columns only, not from the outvoted dirt.
        let mut a = run(9, 0x808080);
        a.light = 0xA5; // sky 10, block 5
        let mut b = run(9, 0x808080);
        b.light = 0x51; // sky 5, block 1
        let mut c = run(3, 0x8B4513); // dirt: outvoted, light must not bleed
        c.light = 0x0F;
        let merged = merge_two_by_two([&[a], &[b], &[c], &[]]);
        assert_eq!(merged.len(), 1);
        // sky (10+5)/2 = 7, block (5+1)/2 = 3 -> 0x73
        assert_eq!(merged[0].light, 0x73);
    }

    #[test]
    fn merge_splits_datapoints_on_light_change() {
        // Same colour both spans, different averaged light: DH keeps TWO
        // datapoints (id != lastId || light != last -> new datapoint).
        let mut lo = run(1, 0x808080);
        lo.light = 0x00; // enclosed
        let mut hi = run(9, 0x808080);
        hi.light = 0xF0; // open sky
        let merged = merge_two_by_two([&[hi, lo], &[hi, lo], &[hi, lo], &[hi, lo]]);
        assert_eq!(merged.len(), 2, "light change closes the accumulator");
        assert_eq!(merged[0].light, 0xF0);
        assert_eq!(merged[0].top, 9);
        assert_eq!(merged[1].light, 0x00);
    }

    #[test]
    fn merge_same_light_extends_the_datapoint() {
        let mut a = run(9, 0x808080);
        a.light = 0x80;
        let merged = merge_two_by_two([&[a], &[a], &[a], &[a]]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].light, 0x80);
        assert_eq!(merged[0].top, 9);
    }
}
