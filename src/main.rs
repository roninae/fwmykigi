#[macro_use]
extern crate clap;
use clap::App;
use std::{env, fs};
use wasmer::{Store, Module, Instance, Value, imports};



fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let wasm_bytes = std::fs::read("/Users/mak/Desktop/demo.wasm").unwrap();
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();

    let add_one = instance.exports.get_function("add_one").unwrap().native::<i32, i32>().unwrap();

    println!("Calling `add_one` function...");
    let result = add_one.call(3).unwrap();

    println!("Results of `add_one`: {:?}", result);

}

