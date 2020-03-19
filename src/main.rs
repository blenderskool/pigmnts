#[macro_use]
extern crate clap;
extern crate spinners;
extern crate termion;
extern crate image;
extern crate pigmnts;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use termion::{color, style};
use std::path::Path;
use std::time::Instant;
use image::GenericImageView;
use pigmnts::{Pixels, color::{LAB, RGB}, weights, pigments_pixels};

/// Creates a color palette from image
/// 
/// Image is loaded from `image_path` and a palette of `count` colors are created
fn pigmnts(image_path: &str, count: u8) -> (Vec<(LAB, f32)>, u128) {
  let img = image::open(image_path)
    .unwrap()
    .resize(800, 800, image::imageops::FilterType::CatmullRom);
  let mut pixels: Pixels = Vec::new();

  // Start a timer
  let now = Instant::now();

  for (_, _, pix) in img.pixels() {
    pixels.push(LAB::from(
      &RGB {
        r: pix[0],
        g: pix[1],
        b: pix[2],
      }
    ));
  }

  let weightfn = weights::resolve_mood(&weights::Mood::Dominant);
  let mut output = pigments_pixels(&pixels, count, weightfn);

  // Sort the output colors based on dominance
  output.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

  return (output, now.elapsed().as_millis());
}

fn main() {

  let matches = App::new("Pigmnts")
                  .version(env!("CARGO_PKG_VERSION"))
                  .author(env!("CARGO_PKG_AUTHORS"))
                  .about("Create color palette from image")
                  .arg(Arg::with_name("count")
                        .short("c")
                        .long("count")
                        .value_name("COUNT")
                        .help("Number of colors in the palette")
                        .multiple(true)
                        .number_of_values(1)
                        .takes_value(true))
                  .arg(Arg::with_name("input")
                        .help("Sets the input file to use")
                        .value_name("FILE")
                        .required(true)
                        .multiple(true)
                        .index(1))
                  .arg(Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .visible_alias("silent")
                        .help("Suppress the normal output"))
                  .get_matches();

  let image_paths = matches.values_of("input").unwrap();
  let mut counts = values_t!(matches, "count", u8).unwrap_or_else(|e| e.exit());
  let is_quiet = matches.is_present("quiet");

  // Fill the default count value (5) for every input file if not specified
  loop {
    let diff: i8 = image_paths.len() as i8 - counts.len() as i8;
    if diff > 0 {
      counts.push(5);
    } else {
      break;
    }
  }

  // Enumerate through each image_path and generate palettes
  for (i, image_path) in image_paths.enumerate() {
  
    if is_quiet {
      // Quiet mode only shows the color codes in resulting palette

      let (result, _) = pigmnts(image_path, counts[i]);
      for (color, _) in result.iter() {
        let rgb: RGB = RGB::from(color);
        println!("{}", rgb.hex());
      }

    } else {

      print!("{}{}Creating a palette of ", color::Fg(color::White), style::Bold);
      print!("{}{} ", color::Fg(color::Blue), counts[i]);
      print!("{}colors from ", color::Fg(color::White));
      println!(
        "{}{}{}",
        color::Fg(color::Blue),
        Path::new(image_path)
          .file_stem()
          .unwrap()
          .to_str()
          .unwrap(),
        style::Reset
      );

      // Show the spinner in the terminal
      let sp = Spinner::new(Spinners::Dots, String::default());
      let (result, time) = pigmnts(image_path, counts[i]);
      // Stop the spinner
      sp.stop();

      println!();
      for (color, dominance) in result.iter() {
        let rgb: RGB = RGB::from(color);
        print!("{}  {} ", color::Bg(color::Rgb(rgb.r, rgb.g, rgb.b)), style::Reset);
        print!("{}{}{} ", style::Bold, rgb.hex(), style::Reset);
        println!("--- {}%", dominance * 100.0);
      }
      println!();

      println!(
        "{}{}✓ Success!{} Took {}ms",
        color::Fg(color::Green),
        style::Bold,
        style::Reset,
        time
      );

    }
  }

}