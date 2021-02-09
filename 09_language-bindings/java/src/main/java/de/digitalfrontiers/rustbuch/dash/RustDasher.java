package de.digitalfrontiers.rustbuch.dash;

public class RustDasher implements Dasher {

    private static native void printHello();

    static {
        System.loadLibrary("rust_buch_language_bindings");
    }

    @Override
    public void sayHello() {
        printHello();
    }
}
