package de.digitalfrontiers.rustbuch.dash;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

import java.util.Optional;

public class RustDasher implements Dasher {

    private static native void printHello();
    private static native void printHelloWithName(Instance<String> s);

    static {
        System.loadLibrary("rust_buch_language_bindings");
    }

    @Override
    public void sayHello() {
        printHello();
    }

    @Override
    public void sayHello(Optional<String> name) {

        printHelloWithName(Java2RustUtils.createInstance(name.orElse("Bob")));
    }
}
