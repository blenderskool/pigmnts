extern crate serde_cbor;
extern crate serde_json;

use std::fs::File;

fn main() {
  // This dataset has been taken from https://github.com/meodai/color-names
  let color_names_file = File::open("./data/colornames.bestof.min.json").unwrap();
  let color_names: serde_json::Value = serde_json::from_reader(color_names_file).unwrap();

  let dest_file = File::create("./data/colornames.cbor").unwrap();
  serde_cbor::to_writer(dest_file, &color_names).unwrap();
}