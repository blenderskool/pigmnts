#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate spinners;
extern crate termion;
extern crate image;
extern crate pigmnts;
extern crate reqwest;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use termion::{color, style};
use prettytable::{Table, format, Row};
use std::{time::Instant, process};
use image::GenericImageView;
use pigmnts::{Pixels, color::{LAB, RGB, HSL}, weights, pigments_pixels};

/// Creates a vector of strings with elements added conditonally
/// 
/// # Example
/// ```
/// let myvec = conditional_vec![
///   is_item_1 => "Item 1",
///   is_item_2 => "Item 2"
/// ]
/// ```
macro_rules! conditional_vec {
  ($( $x:expr => $y:expr),*) => {
    {
      let mut temp_vec = Vec::new();
      $(
        if $x {
          temp_vec.push(format!("{}", $y));
        }
      )*
      temp_vec
    }
  };
}

/// Creates a color palette from image
/// 
/// Image is loaded from `image_path` and a palette of `count` colors are created
fn pigmnts(image_path: &str, count: u8) -> Result<(Vec<(LAB, f32)>, u128), Box<dyn std::error::Error>> {
  let mut img;

  if image_path.starts_with("http://") || image_path.starts_with("https://") {
    let mut res = reqwest::blocking::get(image_path)?;
    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf)?;
    img = image::load_from_memory(buf.as_slice())?;
  }
  else {
    img = image::open(image_path)?;
  }
  
  img = img.resize(800, 800, image::imageops::FilterType::CatmullRom);

  // Start a timer
  let now = Instant::now();

  let pixels: Pixels = img
    .pixels()
    .map(|(_, _, pix)| LAB::from(
      &RGB {
        r: pix[0],
        g: pix[1],
        b: pix[2],
      }
    ))
    .collect();

  let weightfn = weights::resolve_mood(&weights::Mood::Dominant);
  let mut output = pigments_pixels(&pixels, count, weightfn, None);

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
                  .arg(Arg::with_name("hex")
                        .short("x")
                        .long("hex")
                        .help("Enable Hex code output of colors"))
                  .arg(Arg::with_name("rgb")
                        .short("r")
                        .long("rgb")
                        .help("Enable RGB output of colors"))
                  .arg(Arg::with_name("hsl")
                        .short("s")
                        .long("hsl")
                        .help("Enable HSL output of colors"))
                  .arg(Arg::with_name("lab")
                        .short("l")
                        .long("lab")
                        .help("Enable L*AB output of colors"))
                  .arg(Arg::with_name("dominance")
                        .short("d")
                        .long("dominance")
                        .help("Enable dominance percentage of colors"))
                  .get_matches();

  let image_paths = matches.values_of("input").unwrap();
  let mut counts = values_t!(matches, "count", u8).unwrap_or(Vec::new());
  let is_quiet = matches.is_present("quiet");
  let is_rgb = matches.is_present("rgb");
  let is_hsl = matches.is_present("hsl");
  let is_lab = matches.is_present("lab");
  let is_dom = matches.is_present("dominance");
  let mut is_hex = matches.is_present("hex");

  // Hex format is enabled when other formats are disabled
  if !is_rgb && !is_hsl & !is_lab {
    is_hex = true;
  }

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
      // Quiet mode only shows the result separated by ':'

      let (result, _) = pigmnts(image_path, counts[i])
                          .unwrap_or_else(|err| {
                            eprintln!("Problem creating palette: {}", err);
                            process::exit(1);
                          });

      for (color, dominance) in result.iter() {
        let rgb = RGB::from(color);

        let record = conditional_vec![
          is_hex => rgb.hex(),
          is_rgb => rgb,
          is_hsl => HSL::from(color),
          is_lab => color,
          is_dom => dominance * 100.0
        ];

        println!("{}", record.join(":"));
      }

    } else {

      print!("{}{}Creating a palette of ", color::Fg(color::White), style::Bold);
      print!("{}{} ", color::Fg(color::Blue), counts[i]);
      print!("{}colors from ", color::Fg(color::White));
      println!("{}{}{}", color::Fg(color::Blue), image_path, style::Reset);

      // Show the spinner in the terminal
      let sp = Spinner::new(Spinners::Dots, String::default());
      let (result, time) = pigmnts(image_path, counts[i])
                            .unwrap_or_else(|e| {
                              eprintln!(
                                "{}{}Problem creating palette:{} {}",
                                color::Fg(color::Red),
                                style::Bold,
                                style::Reset,
                                e
                              );
                              process::exit(1);
                            });

      // Stop the spinner
      sp.stop();
      println!();

      let mut table = Table::new();
      table.set_format(
        format::FormatBuilder::from(*format::consts::FORMAT_CLEAN)
          .padding(2, 2)
          .build()
      );
      let titles = conditional_vec![
        true => "",  // Title for color preview
        is_hex => "Hex",
        is_rgb => "RGB",
        is_hsl => "HSL",
        is_lab => "LAB",
        is_dom => "Dominance"
      ];
      table.set_titles(
        Row::new(
          titles
            .iter()
            .map(|x| cell!(bcFw -> x))
            .collect()
        )
      );
      
      for (color, dominance) in result.iter() {
        let rgb = RGB::from(color);
        let values = conditional_vec![
          is_hex => rgb.hex(),
          is_rgb => rgb,
          is_hsl => HSL::from(color),
          is_lab => color,
          is_dom => format!("{}%", dominance * 100.0)
        ];
        let mut record = row![
          // Color preview is added
          format!("{}  {}", color::Bg(color::Rgb(rgb.r, rgb.g, rgb.b)), style::Reset)
        ];
          
        for value in values.iter() {
          record.add_cell(cell!(value));
        }

        table.add_row(record);
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
    }
  }

}