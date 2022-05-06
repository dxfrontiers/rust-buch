package de.digitalfrontiers.rustbuch.dash;

import java.util.Optional;

public interface Dasher {
    public void sayHello();
    public void sayHello(final String name);
    void sayHello(final Data data);
}
