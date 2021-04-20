extern crate mockall;

mod kapitel_5_5_1_ohne_framework;
mod kapitel_5_5_2_1_traits;
mod kapitel_5_5_2_2_externe_traits;
mod kapitel_5_5_2_4_generische_methoden;
mod kapitel_5_5_2_3_generische_traits;
mod kapitel_5_5_2_6_module;
mod kapitel_5_5_2_7_gemockte_module_verwenden;
mod kapitel_5_6_1_schoene_behauptungen;

fn main() {
    println!("{}", kapitel_5_5_2_6_module::a_module::return_string());
    println!("{}", kapitel_5_5_2_7_gemockte_module_verwenden::use_module());
}
