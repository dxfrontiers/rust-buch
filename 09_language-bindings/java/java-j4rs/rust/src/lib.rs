use chrono::{DateTime, Local};
use j4rs::prelude::*;
use j4rs::InvocationArg;
use j4rs_derive::call_from_java;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tokio::runtime::Runtime;
use uuid::Uuid;

// 1) Späte statische Variablen, fast Konstanten
lazy_static! {
    // 2) Erstellung der Tokio Runtime
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    // 3) Wir erstellen uns einen Zeitstempel und eine UUID, um zu beweisen,
    // dass wir stets die gleiche Instanz einer Tokio-RUntime verwenden
    static ref NOW: DateTime<Local> = Local::now();
    static ref UUID: Uuid = Uuid::new_v4();
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.printHello")]
fn print_hello() {
    println!("{}", "Hello from the Rust world!");
}

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    age: i8,
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.\
RustDasher.returnHelloWithName")]
fn return_hello_with_name
(input: Instance) -> Result<Instance, String> {
    // 1) Wir verbinden uns mit der aktuellen JNI-Verbindung
    let jvm: Jvm = Jvm::attach_thread().unwrap();

    // 2) Wir konvertieren die Instance in einen Rust String
    let name = match jvm.to_rust(input) {
        Ok(t) => t,
        Err(e) => e.to_string(),
    };

    // 3a) Wir erstellen ein InvocationArg auf Basis eines Strings...
    let ia = InvocationArg::
        try_from(format!(
            "Hello from the Rust world, dear {}!", name))
        .map_err(|error| format!("{}", error))
        .unwrap();
    // 3b) ...und bauen uns aus dem InvocartionArg ene Instance, die wir zurück geben
    Instance::try_from(ia)
        .map_err(|error| format!("{}", error))
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.returnHelloWithData")]
fn return_hello_with_data(input: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();

    println!("{}", input.class_name());

    let data = match jvm.to_rust::<Data>(input) {
        Ok(t) => t,
        Err(e) => {
            panic!("Kaboom - {}", e.to_string())
        }
    };

    let ia = InvocationArg::try_from(format!(
        "Hello from the Rust world, dear {}! {} is no age",
        data.name, data.age
    ))
    .map_err(|error| format!("{}", error))
    .unwrap();
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.returnHelloWithDataAsync")]
#[tokio::main]
async fn print_hello_async(input: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();

    println!("{}", input.class_name());

    let data = match jvm.to_rust::<Data>(input) {
        Ok(t) => t,
        Err(e) => {
            panic!("Kaboom - {}", e.to_string())
        }
    };

    let handle = tokio::spawn(async move {
        format!(
            "Hello from the async Rust world, dear {}! {} is no age",
            data.name, data.age
        )
    });

    let out = handle.await.unwrap();

    let ia = InvocationArg::try_from(out)
        .map_err(|error| format!("{}", error))
        .unwrap();

    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.returnHelloWithDataAsyncOptim")]
fn print_hello_async_optim(input: Instance) -> Result<Instance, String> {
    // 4) Die Verwendung der oben initialisierten Runtime
    RUNTIME.block_on(async {
        // 5) Ausgabe der oben fest gesetzten Werte - sollten immer die selben sein
        println!(
            "Hello from the Tokio Runtime, now: {} {}!",
            NOW.format("%Y-%m-%d][%H:%M:%S%f"),
            UUID.to_hyphenated().to_string()
        );

        // 6) Wie im vorherigen Beispiel holen wir uns den Data-Struct aus der Instance
        let data = {
            let jvm: Jvm = Jvm::attach_thread().unwrap();
            match jvm.to_rust::<Data>(input) {
                Ok(t) => t,
                Err(e) => {
                    panic!("Kaboom - {}", e.to_string())
                }
            }
        };

        // 7) Neuen Tokio Task erstellen, in welchem wir den String formatieren
        let handle = RUNTIME.spawn(async move {
            format!(
                "Hello from the optimized and async Rust world, dear {}! {} is no age",
                data.name, data.age
            )
        });

        // 8) String aus dem Task-Handle holen
        let out = handle.await.unwrap();

        // 9) Wieder in Instance für die Rückgabe verpacken
        let ia = InvocationArg::try_from(out)
            .map_err(|error| format!("{}", error))
            .unwrap();
        Instance::try_from(ia).map_err(|error| format!("{}", error))
    })
}
