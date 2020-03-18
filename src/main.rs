#[macro_use]
extern crate clap;
extern crate spinners;
extern crate console;
extern crate image;
extern crate pigmnts;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use console::style;
use std::path::Path;
use std::time::Instant;
use image::GenericImageView;
use pigmnts::{Pixels, color::{LAB, RGB}, weights, pigments_pixels};

fn main() {

  let matches = App::new("Pigmnts")
                  .version("0.3.0")
                  .author("Akash Hamirwasia")
                  .about("Create color palette from image")
                  .arg(Arg::with_name("count")
                        .short("c")
                        .long("count")
                        .value_name("COUNT")
                        .help("Number of colors in the palette")
                        .takes_value(true))
                  .arg(Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .value_name("FILE")
                        .required(true)
                        .index(1))
                  .get_matches();

  let image_path = matches.value_of("INPUT").unwrap();
  let count = value_t!(matches, "count", u8).unwrap_or(5);

  println!(
    "{} {} {} {}",
    style("Creating a palette of").bold(),
    style(count).blue().bold(),
    style("colors from").bold(),
    style(
      Path::new(image_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
    ).blue().bold(),
  );
  
  let img = image::open(image_path).unwrap();
  let mut pixels: Pixels = Vec::new();

  let now = Instant::now();
  for (_, _, pix) in img.pixels() {
    pixels.push(LAB::from(
      &RGB {
        r: pix[0],
        g: pix[1],
        b: pix[2]
      }
    ));
  }

  
  let sp = Spinner::new(Spinners::Dots, String::default());
  
  let weightfn = weights::resolve_mood(&weights::Mood::Dominant);
  // TODO: Generate proper output
  let _output = pigments_pixels(&pixels, count, weightfn);
  sp.stop();


  println!(
    "{} Took {}ms",
    style("✓ Success!").green().bold(),
    now.elapsed().as_millis(),
  )
}