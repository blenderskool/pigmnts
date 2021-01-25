use crate::color;

use wasm_bindgen::{prelude::*};
use color::{LAB};

pub type WeightFn = fn(&LAB) -> f32;

#[wasm_bindgen]
pub enum Mood {
    Dominant,
}

// Weight function to calculate dominant colors
// All colors are given the same weight
fn dominant(_: &LAB) -> f32 {
    1.0
}

// Resolve the mood to return appropriate weight function
pub fn resolve_mood(mood: &Mood) -> WeightFn {
    match mood {
        Mood::Dominant => dominant,
    }
}