package de.digitalfrontiers.rustbuch;

import com.sun.jna.Library;
import com.sun.jna.Native;


public class Treble {
    public interface CTreble extends Library {
        CTreble INSTANCE = (CTreble) Native.loadLibrary("/Users/marcel/Development/rust-buch/repos/dxfrontiers/09_language-bindings/java-jna/rust/target/debug/librustbuch_bindings_jna.dylib",CTreble.class);

        int treble(int value);
    }

    public static void main(String[] args) {
        System.out.println("trebling: 10 " + CTreble.INSTANCE.treble(10));
    }
}
