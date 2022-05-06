use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    // 1) Wir starten die Runtime/Engine mir der Standard-Konfguration
    let engine = Engine::default();

    // 2) Wir laden das WebAssembly-Modul aus der Datei embed_lib.wasm
    let module = Module::from_file(&engine, "embed_lib.wasm")?;

    // 3) Ein Store ist unser Kommunikationsmedium mit der WebAssembly-Runtime
    let mut store = Store::new(&engine, ());

    // 4) Wir erstellen eine Instanz des Moduls, um es zu verwenden
    let instance = Instance::new(&mut store, &module, &[])?;

    // 5) Über die Instanz erhalten Zugriff auf die Funktionen, welches durch das
    // WebAssembly-Modul zur Verfügung gestellt werden
    let add = instance.get_func(&mut store, "add")
        .expect("`add` was not an exported function");

    // 6) Wir weisen der bislang Typen-losen Funktion die Typen der Parameter und
    // des Rückgabe-Wertes zu
    let add = add.typed::<(i32, i32), i32, _>(&store)?;

    // 7) Schließlich rufen wir add auf, prüfen das Ergebnis und geben es
    // mit Udo Jürgens im Sinn aus.
    let result = add.call(&mut store, (33, 33))?;
    assert_eq!(result, 66);
    println!("Mit wieviel Jahren fängt das Leben an? - Mit {:?}", result);
    Ok(())
}