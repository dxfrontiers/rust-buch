use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[tokio::main]
async fn hello_tokio(mut cx: FunctionContext) -> JsResult<JsString> {
    let handle = tokio::spawn(async {
        "Hello from the Tokio Rust world!"
    });

    let out = handle.await.unwrap();

    Ok(cx.string(out))
}

register_module!(mut module, {
    module.export_function("hello", hello)?;
    module.export_function("helloTokio", hello_tokio)?;
    Ok(())
});