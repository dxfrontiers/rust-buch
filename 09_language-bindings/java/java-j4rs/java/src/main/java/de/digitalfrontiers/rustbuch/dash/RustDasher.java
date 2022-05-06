package de.digitalfrontiers.rustbuch.dash;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

public class RustDasher implements Dasher {

    // 1) Wir laden den nativen Rust-Code
    static {
        System.loadLibrary("rustbuch_bindings_java_j4rs");
    }

    // 2) Wir definieren eine native Methode
    private static native void printHello();
    private static native Instance returnHelloWithName(Instance<String> s);
    private static native Instance returnHelloWithData(Instance<Data> s);

    private static native Instance returnHelloWithDataAsync(Instance<Data> s);
    private static native Instance returnHelloWithDataAsyncOptim(Instance<Data> s);

    @Override
    public void sayHello() {
        printHello();

        //printHelloAsync();

        //printHelloAsyncOptim();
    }

    @Override
    public void sayHello(String name) {
        // 1) Java2RustUtils macht aus String eine j4rs Instance
        Instance<String> inputInstance = Java2RustUtils.createInstance(name);

        // 2) Aufruf der nativen Funktion und damit des Rust-Codes
        Instance outputInstance = returnHelloWithName(inputInstance);

        // 3) Java2RustUtils macht aus einer j4rs Instance einen String
        String hello = Java2RustUtils.getObjectCasted(outputInstance);

        System.out.println(hello + " - presented by Java");
    }

    @Override
    public void sayHello(Data data) {
        Instance<Data> inputInstance = Java2RustUtils.createInstance(data);

        Instance outputInstance = returnHelloWithDataAsyncOptim(inputInstance);

        String hello = Java2RustUtils.getObjectCasted(outputInstance);

        System.out.println(hello + " - presented by Java");
    }
}
