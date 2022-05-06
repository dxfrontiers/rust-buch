use j4rs::{InvocationArg, Jvm, JvmBuilder, errors::Result};
use std::convert::TryFrom;

fn example_1_string() -> Result<Jvm> {
    // 1) Starte eine JVM-Instanz und halte die Referenz
    let jvm = JvmBuilder::new().build()?;

    // 2) Erstelle eine String-Instanz in der JVM
    let string_instance = jvm.create_instance(
        "java.lang.String", // 2a) Java-Klasse, zu der eine Instanz erstellt werden soll
        &[InvocationArg::try_from("Hello from JVM")?], // 2b) Die Argumente für den String-Konstruktor als Vector
    )?;

    // 3) Konvertiere die Referenz auf die String-Instanz in eine Rust-String-Instanz
    let rust_string: String = jvm.to_rust(string_instance)?;

    println!("JVM says '{}'", rust_string );

    Ok(jvm)
}

fn example_2_method_invocation() -> Result<Jvm> {
    let jvm = JvmBuilder::new().build()?;
    let string_instance = jvm.create_instance(
        "java.lang.String",
        &[InvocationArg::try_from("I am a java.lang.String")?],
    )?;

    // 1) Wir rufen die Methode "split" auf
    let split_string_instance = jvm.invoke(
        &string_instance, // 1a) Die Referenz auf das Java-Objekt
        "split", // 1b) Die aufzurufene Methode auf der String-Instanz
        &[InvocationArg::try_from(r"java\.lang\.")?], // 1c) Der reguläre Ausdruck, um den String aufzuteilen
    )?;

    // 2) Wir geben das Ergebnis der Aufteilung aus
    let split_string_array: Vec<String> = jvm.to_rust(split_string_instance)?;
    println!("Got split result - 1: '{}' 2: '{}'",
             split_string_array.get(0).unwrap(),
             split_string_array.get(1).unwrap()
    );

    Ok(jvm)
}

fn example_3_clone_instance() -> Result<Jvm> {
    let jvm = JvmBuilder::new().build()?;
    let string_instance = jvm.create_instance(
        "java.lang.String",
        &[InvocationArg::try_from("I am 23 characters long")?],
    )?;

    // 1) Wir klonen die String-Instanz
    let clone_instance = jvm.clone_instance(&string_instance)?;

    // 2) Wir geben die geklonte String-Instanz als Zwischenergebnis aus
    let rust_string: String = jvm.to_rust(clone_instance)?;
    println!("String created with '{}'", rust_string );

    // 3) Wir arbeiten mit der ursprünglichen Instanz weiter und rufen "length()" auf der Instanz auf
    let integer_instance = jvm.invoke(
        &string_instance,
        "length",
        &[],
    )?;

    // 4) Ausgabe des Ergerbnisses von "length()"
    let rust_int: i16 = jvm.to_rust(integer_instance)?;
    println!("Got a String with length of {}", rust_int);

    Ok(jvm)
}

#[allow(unused_must_use)]
fn main() {
    println!("--- Example 1 ---");
    example_1_string();
    println!("--- Example 2 ---");
    example_2_method_invocation();
    println!("--- Example 3 ---");
    example_3_clone_instance();
}
