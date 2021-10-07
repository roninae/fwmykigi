#[macro_use]
extern crate clap;
use clap::App;
use std::{env, fs};
use wasmer::{Store, Module, Instance, Value, imports};



fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let wasm_file = matches.value_of("wasm").unwrap();
    let wasm_bytes = std::fs::read(wasm_file).unwrap();
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let fab_events = instance.exports.get_function("fab_events").unwrap().native::<i32, i32>().unwrap();
    println!("Calling `fab_events` function with default params [10000]...");
    let result = fab_events.call(10000).unwrap();
    println!("Results of `fab_events`: {:?}", result);

}

