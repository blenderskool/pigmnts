pub mod color;
pub mod weights;

use rand::{distributions::WeightedIndex, prelude::*};
use color::LAB;
use weights::WeightFn;

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{CanvasRenderingContext2d, HtmlCanvasElement},
    weights::{Mood, resolve_mood},
    color::{RGB, HSL},
    serde_derive::Serialize,
};

#[cfg(not(target_arch = "wasm32"))]
use {
    std::cmp,
    crossbeam_utils::thread,
};

pub type Pixels = Vec<LAB>;

/// Recalculates the means using a weight function
fn recal_means(colors: &Vec<LAB>, weight: WeightFn) -> LAB {
    let mut new_color = LAB {
        l: 0.0,
        a: 0.0,
        b: 0.0
    };
    let mut w_sum = 0.0;

    for col in colors.iter() {
        let w = weight(col);
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

#[cfg(target_arch = "wasm32")]
fn find_clusters(pixels: &Pixels, means: &Pixels, k: usize) -> Vec<Pixels> {
    let mut clusters: Vec<Pixels> = vec![Vec::new(); k];

    for color in pixels.iter() {
        clusters[color.nearest(means).0].push(color.clone());
    }

    return clusters;
}

#[cfg(not(target_arch = "wasm32"))]
fn find_clusters(pixels: &Pixels, means: &Pixels, k: usize) -> Vec<Pixels> {
    const NUM_THREADS: usize = 5;

    let num_pixels = pixels.len();
    let sample_size = num_pixels / NUM_THREADS;
    let mut clusters: Vec<Pixels> = vec![Vec::new(); k as usize];
    
    thread::scope(|s| {
        let mut threads = vec![];
        
        // Data parallelism where each thread operates on a sample of data
        for i in 0..NUM_THREADS {
            let start = i * sample_size;
            let end = cmp::min(start + sample_size, num_pixels);

            // Each thread is responsible in finding the nearest cluster mean for each point in the sample (map phase)
            threads.push(
                s.spawn(move |_| {
                    let mut clusters: Vec<Vec<LAB>> = vec![Vec::new(); k];
                    for pixel_idx in start..end {
                        let color = &pixels[pixel_idx];
                        clusters[color.nearest(&means).0].push(color.clone());
                    }
                    return clusters;
                })
            );
        }

        // Results from each thread is combined (or reduced)
        for t in threads {
            let mut mid_clusters = t.join().unwrap();
            for (i, cluster) in mid_clusters.iter_mut().enumerate() {
                clusters[i].append(cluster);
            }
        }

    }).unwrap();

    return clusters;
}

/// Parallelized K-means++ clustering to create the palette from pixels
pub fn pigments_pixels(pixels: &Pixels, k: u8, weight: WeightFn, max_iter: Option<u16>) -> Vec<(LAB, f32)> {
    // Values referenced from https://scikit-learn.org/stable/modules/generated/sklearn.cluster.KMeans.html
    const TOLERANCE: f32 = 1e-4;
    const MAX_ITER: u16 = 300;

    let mut rng = rand::thread_rng();
    let k = k as usize;

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

    let mut clusters: Vec<Vec<LAB>>;
    let mut iters_left = max_iter.unwrap_or(MAX_ITER);

    loop {
        // Assignment step: Clusters are formed in current iteration
        clusters = find_clusters(pixels, &means, k);

        // Updation step: New cluster means are calculated
        let mut changed: bool = false;
        for i in 0..clusters.len() {
            let new_mean = recal_means(&clusters[i], weight);
            if means[i].distance(&new_mean) > TOLERANCE {
                changed = true;
            }

            means[i] = new_mean;
        }

        iters_left -= 1;

        if !changed || iters_left <= 0 {
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


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn pigments(canvas: HtmlCanvasElement, k: u8, mood: Mood, batch_size: Option<u32>) -> JsValue {

    #[derive(Serialize)]
    struct PaletteColor {
        pub dominance: f32,
        pub hex: String,
        pub rgb: RGB,
        pub hsl: HSL,
    }

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
        .map(|i| LAB::from_rgb(data[i], data[i+1], data[i+2]))
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
    let weight: WeightFn = resolve_mood(&mood);
    let palettes: Vec<PaletteColor> = pigments_pixels(&pixels, k, weight, None)
        .iter()
        .map(|(color, dominance)| {
            let rgb = RGB::from(color);
            PaletteColor {
                dominance: *dominance,
                hex: rgb.hex(),
                rgb: rgb,
                hsl: HSL::from(color),
            }
        })
        .collect();

    // Convert to a JS value
    return JsValue::from_serde(&palettes).unwrap();
}
