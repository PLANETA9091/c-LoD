import javax.tools.JavaCompiler;
import javax.tools.ToolProvider;

/**
 * Local stand-in for `javac` on JRE-only sandboxes: the runtime ships the
 * jdk.compiler module, so ToolProvider.getSystemJavaCompiler() works without
 * a JDK install. Args pass straight through (same flags as
 * scripts/java/build.sh). CI keeps using real javac and diffs the bytecode.
 */
public final class Compile {
    private Compile() {}

    public static void main(String[] args) throws Exception {
        JavaCompiler jc = ToolProvider.getSystemJavaCompiler();
        if (jc == null) {
            System.err.println("FATAL: no system java compiler (jdk.compiler missing)");
            System.exit(2);
        }
        int rc = jc.run(null, null, null, args);
        System.exit(rc);
    }
}
