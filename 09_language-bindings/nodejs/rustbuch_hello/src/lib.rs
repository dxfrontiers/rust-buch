use neon::prelude::*;
use tokio::runtime::Runtime;
use lazy_static::lazy_static;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {

    // 1) Holen der Funktionsargumente
    let name = cx.argument::<JsString>(0)?.value(&mut cx);
    let age = cx.argument::<JsNumber>(1)?.value(&mut cx);

    // 2) Formatierung des Strings für die Ausgabe
    let out = format!(
        "Hello from the Rust world, dear {}! {} is no age",
        name, age
    );

    // 3) Rückgabe des String
    Ok(cx.string(out))
}

// 1) Einsatz von lazy_static, um die Runtime von Tokio nur einmal erstellen zu müssen.
lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

fn hello_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {

    let name = cx.argument::<JsString>(0)?.value(&mut cx);
    let age = cx.argument::<JsNumber>(1)?.value(&mut cx);

    // 2) Holen der übergebenen Callback-Funktion
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    // 3) Holen eines Channels, um eine Funktion in die Event loop von node.js zu geben
    let channel = cx.channel();

    // 4) Beginn der Nebenläufkeit - gestützt durch einen Tokio-Thread
    RUNTIME.spawn(async move {
        let result = format!(
            "Hello from the async Rust world, dear {}! {} is no age",
            name, age
        );

        // 5) Platzierung eines node.js-Tasks auf der node.js-Loop
        channel.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);

            // 6) Für das JS-this weisen wir undefined zu
            let this = cx.undefined();

            // 7) Wir übergeben der Callback den String als Parameter und bereiten dafür einen Vektor vor
            let args: Vec<Handle<JsValue>> = vec![
                cx.string(result as String).upcast()
            ];

            // 8) Der tatsächliche Aufruf der Callback-Funktion
            callback.call(&mut cx, this, args)?;

            Ok(())
        });
    });

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("helloAsync", hello_async)?;
    Ok(())
}
