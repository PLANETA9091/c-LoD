package clodmock;

/** Deterministic mock terrain shared by the mock volume + aquifer.
 *  h(x,z) = 64 + waves; density = (y - h)/8; VOID_CHUNK is all air. */
public final class PreflightTerrain {
    private PreflightTerrain() {}

    public static final int VOID_CHUNK = -9; // chunk cx == -9: no terrain at all

    public static double height(int x, int z) {
        if ((x >> 4) == VOID_CHUNK && (z >> 4) == VOID_CHUNK) {
            return -10000; // pure void chunk (walk finds no surface)
        }
        return 64 + 12 * Math.sin(x * 0.05) + 8 * Math.cos(z * 0.073)
                + 5 * Math.sin((x + z) * 0.021);
    }

    public static double density(int x, int y, int z) {
        return (height(x, z) - y) / 8.0; // vanilla sign: positive below the surface
    }
}
