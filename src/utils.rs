extern crate serde_cbor;

use pigmnts::color::{LAB, RGB};
use std::collections::HashMap;

/// Coverts a hex string to RGB color
fn hex_to_rgb(s: &str) -> RGB {
  let hex = s.replace("#", "").to_lowercase();

  let hex_num = usize::from_str_radix(&hex, 16).unwrap();
  return RGB {
      r: (hex_num >> 16) as u8,
      g: ((hex_num >> 8) & 0x00FF) as u8,
      b: (hex_num & 0x0000_00FF) as u8,
  };
}

lazy_static! {
  static ref COLOR_NAMES: (Vec<String>, Vec<LAB>) = {
      let data: HashMap<String, String> = serde_cbor::from_slice(include_bytes!("../data/colornames.cbor")).unwrap();
      let values: Vec<LAB> = data
          .iter()
          .map(|(val, _)| LAB::from(&hex_to_rgb(val)))
          .collect();

      return (data.values().cloned().collect(), values);
  };
}

/// Returns nearest name of a color
pub fn near_color_name(color: &LAB) -> &String {
  return &COLOR_NAMES.0[color.nearest(&COLOR_NAMES.1).0];
}