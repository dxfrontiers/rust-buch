#![cfg_attr(test, feature(proc_macro_hygiene))]

extern crate mockall;
extern crate cfg_if;

mod hello;
mod hello_mockall;

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
