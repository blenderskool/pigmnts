#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate spinners;
extern crate termion;
extern crate image;
extern crate pigmnts;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use termion::{color, style};
use prettytable::{Table, format};
use std::{path::Path, time::Instant, process};
use image::GenericImageView;
use pigmnts::{Pixels, color::{LAB, RGB}, weights, pigments_pixels};

/// Creates a color palette from image
/// 
/// Image is loaded from `image_path` and a palette of `count` colors are created
fn pigmnts(image_path: &str, count: u8) -> Result<(Vec<(LAB, f32)>, u128), image::ImageError> {
  let img = image::open(image_path)?
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

  return Ok((output, now.elapsed().as_millis()));
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
  let mut counts = values_t!(matches, "count", u8).unwrap_or(Vec::new());
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

      let (result, _) = pigmnts(image_path, counts[i])
        .unwrap_or_else(|err| {
          eprintln!("Problem creating palette: {}", err);
          process::exit(1);
        });

      let mut table = Table::new();
      table.set_format(
        format::FormatBuilder::new()
          .padding(0, 0)
          .borders('\0')
          .column_separator(':')
          .build()
      );

      for (color, dominance) in result.iter() {
        let rgb = RGB::from(color);

        table.add_row(row![
          rgb.hex(),
          dominance * 100.0,
        ]);
      }
      table.printstd();

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
      let output = pigmnts(image_path, counts[i]);
      // Stop the spinner
      sp.stop();
      println!();

      match output {
        Ok((result, time)) => {

          let mut table = Table::new();
          table.set_format(
            format::FormatBuilder::from(*format::consts::FORMAT_CLEAN)
              .padding(2, 2)
              .build()
          );
          table.set_titles(row!["", c -> "Hex", c -> "Dominance"]);
          
          for (color, dominance) in result.iter() {
            let rgb = RGB::from(color);

            table.add_row(row![
              format!("{}  {}", color::Bg(color::Rgb(rgb.r, rgb.g, rgb.b)), style::Reset),
              format!("{}{}{}", style::Bold, rgb.hex(), style::Reset),
              format!("{}%", dominance * 100.0),
            ]);
          }
          table.printstd();
          println!();

          println!(
            "{}{}✓ Success!{} Took {}ms",
            color::Fg(color::Green),
            style::Bold,
            style::Reset,
            time
          );

        },
        Err(e) => {
          eprintln!("{}{}Problem creating palette:{} {}", color::Fg(color::Red), style::Bold, style::Reset, e);
          process::exit(1);
        },
      };

    }
  }

}