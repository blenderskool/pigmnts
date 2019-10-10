extern crate wasm_bindgen;
extern crate rand;
extern crate web_sys;

pub mod rgba;

use wasm_bindgen::{prelude::*, JsCast};
use rand::{prelude::*, distributions::WeightedIndex};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, ImageData};
use rgba::{RGBA, Pixels};

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
pub fn pigments_pixels(pixels: &Pixels, k: u8) -> Pixels {
    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0, pixels.len());
    // let initial = recal_means(&pixels);
    let mut means: Pixels = vec!( pixels[i].clone() );

    // Pick the remaining (k-1) means
    for _ in 0..(k-1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<u64> = pixels
            .iter()
            .map(|color| (color.nearest(&means).1 as u64).pow(2))
            .collect();

        // Using the distances as weights, pick a color and use it as a cluster center
        let dist = WeightedIndex::new(&distances).unwrap();
        means.push(
            pixels[dist.sample(&mut rng)].clone()
        );
    }

    loop {
        let mut clusters: Vec<Pixels> = means
            .iter()
            .map(|mean| vec![ mean.clone() ])
            .collect();

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
    
    // Sort the colors based on their brightness
    means.sort_by(|a, b| b.brightness().partial_cmp(&a.brightness()).unwrap());
    return means;
}


#[wasm_bindgen]
pub fn pigments(canvas: HtmlCanvasElement, k: u8) -> Vec<JsValue> {
    // Get context from canvas element
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Image data gathered from the canvas
    let data = ctx.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        .unwrap()
        .data()
        .to_vec();

    // Convert to Pixels type
    let pixels: Pixels = (0..data.len())
        .step_by(4)
        .map(|i| RGBA {
            r: data[i],
            g: data[i+1],
            b: data[i+2],
            a: data[i+3]
        }).collect();

    // Generate the color palette
    let palette = pigments_pixels(&pixels, k);

    // Convert it to a Vector of Hex color code strings
    return palette
        .iter()
        .map(|c| str::replace(&format!("#{:2X}{:2X}{:2X}{:2X}", c.r, c.g, c.b, c.a), " ", "0").into())
        .collect();

}