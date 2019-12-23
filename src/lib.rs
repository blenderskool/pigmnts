extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod rgba;

use rand::{distributions::WeightedIndex, prelude::*, seq::SliceRandom};
use rgba::{Pixels, RGBA};
use std::collections::HashMap;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

/**
 * Recalculate the means
 */
fn recal_means(colors: &Vec<RGBA>) -> RGBA<u8> {
    let mut new_color: RGBA<i32> = colors[0].clone().into();

    for i in 1..colors.len() {
        new_color.r += colors[i].r as i32;
        new_color.g += colors[i].g as i32;
        new_color.b += colors[i].b as i32;
        new_color.a += colors[i].a as i32;
    }

    new_color.r /= colors.len() as i32;
    new_color.g /= colors.len() as i32;
    new_color.b /= colors.len() as i32;
    new_color.a /= colors.len() as i32;

    return new_color.into();
}

/**
 * K-means++ clustering to create the palette
 */
pub fn pigments_pixels(pixels: &Pixels, k: u8) -> Vec<(RGBA, f32)> {
    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0, pixels.len());
    // let initial = recal_means(&pixels);
    let mut means: Pixels = vec![pixels[i].clone()];

    // Pick the remaining (k-1) means
    for _ in 0..(k - 1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<f32> = pixels
            .iter()
            .map(|color| (color.nearest(&means).1 as f32).powf(2.0))
            .collect();

        // Create a weighted distribution based on distance^2
        // If error occurs, return the means already found
        let dist = match WeightedIndex::new(&distances) {
            Ok(t) => t,
            Err(_) => {
                // Calculate the dominance of each color
                let mut palette: Vec<(RGBA, f32)> =
                    means.iter().map(|c| (c.clone(), 0.0)).collect();

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

    let mut clusters: Vec<Pixels>;
    loop {
        clusters = means.iter().map(|mean| vec![mean.clone()]).collect();

        for color in pixels.iter() {
            clusters[color.nearest(&means).0].push(color.clone());
        }

        let mut changed: bool = false;
        for (i, cluster) in clusters.iter().enumerate() {
            let new_mean = recal_means(cluster);
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
    // 1 is subtracted since cluster includes the mean color as it's first item, which shouldn't
    // be included in the dominance
    // For every mean, the corresponding dominance is added as a tuple item
    let palette: Vec<(RGBA, f32)> = means
        .iter()
        .enumerate()
        .map(|(i, mean)| {
            (
                mean.clone(),
                (clusters[i].len() - 1) as f32 / pixels.len() as f32,
            )
        })
        .collect();

    return palette;
}

#[wasm_bindgen]
pub fn pigments(canvas: HtmlCanvasElement, k: u8, batch_size: Option<u32>) -> JsValue {
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
        .data()
        .to_vec();

    // Convert to Pixels type
    let mut pixels: Pixels = (0..data.len())
        .step_by(4)
        .map(|i| RGBA {
            r: data[i],
            g: data[i + 1],
            b: data[i + 2],
            a: data[i + 3],
        })
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

    let mut palette = HashMap::new();
    // Generate the color palette and conver it to a hashmap
    // with keys as color hex codes, and values as dominance
    for (color, dominance) in pigments_pixels(&pixels, k).iter() {
        palette.insert(color.hex(), *dominance);
    }

    // Convert to a JS value
    return JsValue::from_serde(&palette).unwrap();
}
