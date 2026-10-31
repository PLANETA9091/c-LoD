package net.minecraft.world.level.levelgen.densityfunction;

/** Mock volume with precomputed terrain values. Two index layouts selected
 *  by -Dclod.mock.layout: "affine" (vanilla-like, linear in i) and "weird"
 *  (a fixed Fisher-Yates permutation — forces the helper's per-column stride
 *  verification to fail and exercises the exact v5.8 fallback). Whatever the
 *  layout, buf.get(indexUnchecked(dx,i,dz)) == density(ox+dx, oy+i, oz+dz). */
public class DensityVolume {
    public final int sx, sy, sz, ox, oy, oz;
    private final double[] v;
    private final int[] perm; // weird layout only

    public DensityVolume(int sx, int sy, int sz, int ox, int oy, int oz) {
        this.sx = sx;
        this.sy = sy;
        this.sz = sz;
        this.ox = ox;
        this.oy = oy;
        this.oz = oz;
        int n = sx * sy * sz;
        this.v = new double[n];
        boolean weird = "weird".equals(System.getProperty("clod.mock.layout", "affine"));
        if (weird) {
            java.util.Random rnd = new java.util.Random(909091);
            perm = new int[n];
            for (int i = 0; i < n; i++) {
                perm[i] = i;
            }
            for (int i = n - 1; i > 0; i--) {
                int j = rnd.nextInt(i + 1);
                int t = perm[i];
                perm[i] = perm[j];
                perm[j] = t;
            }
        } else {
            perm = null;
        }
        for (int dx = 0; dx < sx; dx++) {
            for (int i = 0; i < sy; i++) {
                for (int dz = 0; dz < sz; dz++) {
                    v[indexUnchecked(dx, i, dz)] = clodmock.PreflightTerrain.density(ox + dx, oy + i, oz + dz);
                }
            }
        }
    }

    public int indexUnchecked(int dx, int i, int dz) {
        int linear = (i * sz + dz) * sx + dx;
        return perm != null ? perm[linear] : linear;
    }

    public int blockY(int i) {
        return oy + i;
    }

    public double at(int idx) {
        return v[idx];
    }
}
