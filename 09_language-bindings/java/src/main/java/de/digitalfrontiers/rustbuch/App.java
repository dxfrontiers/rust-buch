package de.digitalfrontiers.rustbuch;

import de.digitalfrontiers.rustbuch.dash.Dasher;
import de.digitalfrontiers.rustbuch.dash.RustDasher;

import java.util.Optional;

public class App
{
    public static void main( String[] args )
    {
        Dasher dasher = new RustDasher();
        dasher.sayHello();
        dasher.sayHello(Optional.of("Rustacean"));
        dasher.sayHello(Optional.empty());
    }
}
