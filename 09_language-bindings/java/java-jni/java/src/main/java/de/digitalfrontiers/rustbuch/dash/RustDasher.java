package de.digitalfrontiers.rustbuch.dash;

import java.util.Optional;

public class RustDasher implements Dasher {

    // 1) Wir laden den nativen Rust-Code
    static {
        System.loadLibrary("rustbuch_bindings_java_jni");
    }

    // 2) Wir definieren eine native Methode
    private static native void printHello();
    private static native String returnHello();
    private static native void printHelloAsync();
    private static native void printHelloWithName(String s);

    @Override
    public void sayHello() {
        printHello();

        String hello = returnHello();
        System.out.println(hello);

        printHelloAsync();
    }

    @Override
    public void sayHello(Optional<String> name) {

        printHelloWithName(name.orElse("Bob"));
    }
}
