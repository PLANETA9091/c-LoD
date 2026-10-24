# Big-atlas validation run — atlas 2048 / cell 4 (branch bench/atlas2048)

CI run 40111512080 (green, all 11 jobs incl. e2e-visual on the 16GB runner).

- Profile: scan_mode=async, deadline 500s, atlas_size=2048, cell_size=4,
  stream_chunks=64, fog plane = atlas half = 4096 blocks (DH farFogEnd 1.0).
- Scan: 259081/262144 cells (98%) — the FULL 16384-chunk scan box scanned;
  pack 1,188,727 bytes (2.1x the 1024 pack), sha1 d9cede6c.
- Client joined on the runner; the 8-shot matrix + visual gates passed.
- Evidence here = zoom crops only: the full-frame evidence commit from the
  branch was rejected (HEAD:master non-FF by design); frames verified on the
  run + artifacts.
- Verdict (zoom inspection, NEAREST crops in zoom/):
  * south/east/north horizons: flat DH-like cell colours, river ribbons,
    terraced silhouettes, shaded walls, no speckle, no band truncation;
  * the tall translucent column in 02_yaw_south (x~865) is a legitimate far
    peak behind the fog — identical feature in the v4.8.1 preview (fog 2048),
    more of it visible at fog 4096;
  * aerial above clouds + night camera-channel lighting: correct.
- Consequence: the dynamic march budget (v4.6.1) and the winner pyramids
  (v4.8.0) hold at the product-scale atlas; local 2-core/4GB sandbox cannot
  run this profile together with a client (see worklog Task 15).
