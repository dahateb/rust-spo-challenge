mod objects;

use std::env;
use objects::Input;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_raw = args.get(1).unwrap_or_else(|| panic!("input is required!"));
    let input : Input = serde_json::from_str(&input_raw).map_err(|err| panic!("wrong format: {}", err)).unwrap();

    let output = input.calculate();

    println!("{}", serde_json::to_string(&output).unwrap());
}
