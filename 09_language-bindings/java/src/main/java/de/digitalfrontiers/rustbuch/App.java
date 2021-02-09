package de.digitalfrontiers.rustbuch;

import de.digitalfrontiers.rustbuch.dash.Dasher;
import de.digitalfrontiers.rustbuch.dash.RustDasher;

/**
 * Hello world!
 *
 */
public class App
{
    public static void main( String[] args )
    {
        Dasher dasher = new RustDasher();
        dasher.sayHello();
    }
}
