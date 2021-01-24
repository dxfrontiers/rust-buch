#![cfg_attr(test, feature(proc_macro_hygiene))]

extern crate mockall;
extern crate cfg_if;

mod hello;
mod hello_mockall;
mod reference;
mod kapitel_1_6_1_without_mockall;
mod kapitel_1_6_2_1_traits;
mod kapitel_1_6_2_2_external_traits;
mod kapitel_1_6_2_3_generic_methods;
mod kapitel_1_6_2_3_generic_traits;
mod kapitel_1_6_2_4_modules;
mod kapitel_1_6_2_5_other_modules;

use hello_mockall::to_be_mocked_wrapper_module;
use hello_mockall::lets_mock_module;
use crate::hello_mockall::lets_mock_module::MyStruct;

fn main() {
    println!("{}", hello::hello());

    println!("{}", to_be_mocked_wrapper_module::prepared_module::hello_world());

    println!("{}", lets_mock_module::do_something());

    let my = MyStruct {};
    println!("{}", lets_mock_module::call_with_four(&my));
}
