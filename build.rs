use std::fs::File;
use std::env;

fn main() {
  // This dataset has been taken from https://github.com/meodai/color-names
  let color_names_file = File::open("./data/colornames.bestof.min.json").unwrap();
  let color_names: serde_json::Value = serde_json::from_reader(color_names_file).unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  let dest_file = File::create(format!("{}/colornames.cbor", out_dir)).unwrap();
  serde_cbor::to_writer(dest_file, &color_names).unwrap();
}