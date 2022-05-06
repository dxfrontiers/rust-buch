#[macro_use]
extern crate lazy_static;

use jni::JNIEnv;
use jni::objects::{JObject, JString}; // 1)
use jni::sys::{jstring, jint}; // 2)
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

#[repr("C")]
struct Data {
    name: jstring,
    age: jint
}

#[no_mangle] // 3) Name der Funktion soll nicht verkürzt werden
pub extern "system" fn Java_de_digitalfrontiers_rustbuch_dash_RustDasher_printHello() { // 4) Eine sehr lange Methodensignatur
    println!("Hello from the Rust world via JNI!");
}

#[no_mangle]
pub extern "system" fn Java_de_digitalfrontiers_rustbuch_dash_RustDasher_printHelloAsync(_env: JNIEnv, _obj: JObject)  {
        RUNTIME.block_on(async {
            println!(
                "Hello from the Tokio Runtime!",
            );

            let handle = tokio::spawn(async { "Hello from the spawned and optimized Rust world!" });

            let out = handle.await.unwrap();
            println!("{}", out);
        });
}

#[no_mangle]
pub extern "system" fn Java_de_digitalfrontiers_rustbuch_dash_RustDasher_returnHello(env: JNIEnv, _obj: JObject) -> jstring {
    let output = env
        .new_string(format!("Hello from the Rust world, printend by Java!"))
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_de_digitalfrontiers_rustbuch_dash_RustDasher_printHelloWithName(env: JNIEnv, _obj: JObject, input: JString)  {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    println!("Hello from the Rust world via JNI, dear {}!", input);
}