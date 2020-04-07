extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;

pub mod color;
pub mod weights;

use rand::{distributions::WeightedIndex, prelude::*, seq::SliceRandom};
use color::{RGB, HSL, LAB};
use weights::{Mood, WeightFn, resolve_mood};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub type Pixels = Vec<LAB>;

/**
 * Recalculate the means
 */
fn recal_means(colors: &Vec<&LAB>, weight: WeightFn) -> LAB {
    let mut new_color = LAB {
        l: 0.0,
        a: 0.0,
        b: 0.0
    };
    let mut w_sum = 0.0;

    for col in colors.iter() {
        let w = weight(*col);
        w_sum += w;
        new_color.l += w * col.l;
        new_color.a += w * col.a;
        new_color.b += w * col.b;
    }

    new_color.l /= w_sum;
    new_color.a /= w_sum;
    new_color.b /= w_sum;

    return new_color;
}

/**
 * K-means++ clustering to create the palette
 */
pub fn pigments_pixels(pixels: &Pixels, k: u8, weight: WeightFn) -> Vec<(LAB, f32)> {
    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0, pixels.len());
    let mut means: Pixels = vec![pixels[i].clone()];

    // Pick the remaining (k-1) means
    for _ in 0..(k - 1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<f32> = pixels
            .iter()
            .map(|color| (color.nearest(&means).1).powi(2))
            .collect();

        // Create a weighted distribution based on distance^2
        // If error occurs, return the means already found
        let dist = match WeightedIndex::new(&distances) {
            Ok(t) => t,
            Err(_) => {
                // Calculate the dominance of each color
                let mut palette: Vec<(LAB, f32)> = means.iter().map(|c| (c.clone(), 0.0)).collect();

                let len = pixels.len() as f32;
                for color in pixels.iter() {
                    let near = color.nearest(&means).0;
                    palette[near].1 += 1.0 / len;
                }

                return palette;
            }
        };

        // Using the distances^2 as weights, pick a color and use it as a cluster center
        means.push(pixels[dist.sample(&mut rng)].clone());
    }

    let mut clusters: Vec<Vec<&LAB>>;
    loop {
        clusters = vec![Vec::new(); k as usize];

        for color in pixels.iter() {
            clusters[color.nearest(&means).0].push(color);
        }

        let mut changed: bool = false;
        for i in 0..clusters.len() {
            let new_mean = recal_means(&clusters[i], weight);
            if means[i] != new_mean {
                changed = true;
            }

            means[i] = new_mean;
        }

        if !changed {
            break;
        }
    }

    // The length of every cluster divided by total pixels gives the dominance of each mean
    // For every mean, the corresponding dominance is added as a tuple item
    return clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            (
                means[i].clone(),
                cluster.len() as f32 / pixels.len() as f32,
            )
        })
        .collect();
}


#[derive(Serialize)]
struct PaletteColor {
    pub dominance: f32,
    pub hex: String,
    pub rgb: RGB,
    pub hsl: HSL,
}

#[wasm_bindgen]
pub fn pigments(canvas: HtmlCanvasElement, k: u8, mood: Mood, batch_size: Option<u32>) -> JsValue {
    // Get context from canvas element
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Image data gathered from the canvas
    let data = ctx
        .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        .unwrap()
        .data();

    // Convert to Pixels type
    let mut pixels: Pixels = (0..data.len())
        .step_by(4)
        .map(|i| LAB::from(
            &RGB {
                r: data[i],
                g: data[i + 1],
                b: data[i + 2]
            }
        ))
        .collect();

    // Randomly choose a sample of batch size if given
    let batch = batch_size.unwrap_or(0);
    if batch != 0 && batch < canvas.width() * canvas.height() && batch > k.into() {
        let mut rng = rand::thread_rng();
        pixels = pixels
            .choose_multiple(&mut rng, batch as usize)
            .cloned()
            .collect();
    }
    
    // Generate the color palette and store it in a Vector of PaletteColor
    let mut palettes = Vec::new();
    let weight: WeightFn = resolve_mood(&mood);
    for (color, dominance) in pigments_pixels(&pixels, k, weight).iter() {
        let rgb = RGB::from(color);
        palettes.push(
            PaletteColor {
                dominance: *dominance,
                hex: rgb.hex(),
                rgb: rgb,
                hsl: HSL::from(color),
            }
        );
    }

    // Convert to a JS value
    return JsValue::from_serde(&palettes).unwrap();
}
