package de.digitalfrontiers.rustbuch;

import de.digitalfrontiers.rustbuch.dash.Dasher;
import de.digitalfrontiers.rustbuch.dash.Data;
import de.digitalfrontiers.rustbuch.dash.RustDasher;

import java.util.Optional;

public class App
{
    public static void main( String[] args )
    {
        Dasher dasher = new RustDasher();
        dasher.sayHello();
        dasher.sayHello("Rustacean");

        Data hugo = new Data("Hugo", 66);

        dasher.sayHello(hugo);

        Data alfred = new Data("Alfred", 1);
        dasher.sayHello(alfred);
    }
}
