# 🎨 Pigmnts
Pigmnts is a color palette creator from an image built using Rust. It uses the [K-means++](https://en.wikipedia.org/wiki/K-means%2B%2B) clustering algorithm to select the most commonly occurring colors from the image.

**Pigmnts library** is compiled to WebAssembly which allows for super-fast color palette extraction from an image on the web.
The library can be found in the `lib` directory.

## Pigmnts CLI
Pigmnts CLI is a tool designed to create color palettes from an image right on your terminal. It supports various image formats like `JPEG`, `PNG`, `GIF`, `WebP`, `TIFF` along with external HTTP(S) image URLs. It provides a beautiful terminal output to preview the colors in the palette.

Pigmnts CLI comes with various output modes and provides on-demand data of the palette generated while maintaining high speeds.

### Installing the CLI
CLI can be installed using `cargo` and `pigmnts-cli` crate on crates.io
```bash
cargo install pigmnts-cli
```


### Output modes

#### Default mode
The default mode displays the palette in a user-friendly way with a small preview and corresponding color codes in a tabular structure. This is meant for the common use of the CLI.

![](https://user-images.githubusercontent.com/21107799/77250424-f112d600-6c6d-11ea-82ef-4ebb32d86ee0.png)

### Quiet (or Silent) mode
This mode displays only the essential output without the intermediate logs. The output is in plain text format with each data item separated by `:`. This is meant for use in a pipeline where the output of the CLI is used as input for another process. It can be activated by the `-q or --quiet` flag.

![](https://user-images.githubusercontent.com/21107799/77250518-801fee00-6c6e-11ea-9086-b077447fd4d1.png)


### Flags and options in the CLI
The following flags and options are supported by the latest release of the CLI.
```
FLAGS:
    -d, --dominance    Enable dominance percentage of colors
    -h, --help         Prints help information
    -x, --hex          Enable Hex code output of colors
    -s, --hsl          Enable HSL output of colors
    -l, --lab          Enable L*AB output of colors
    -q, --quiet        Suppress the normal output [aliases: silent]
    -r, --rgb          Enable RGB output of colors
    -n, --name         Nearest name for the color
    -V, --version      Prints version information

OPTIONS:
    -c, --count <COUNT>...    Number of colors in the palette
```

#### Examples of these flags

- `pigmnts pic-1.jpg -c 5 pic-2.jpg -c 8`  
  Generate a palette of 5 colors from pic-1.jpg and 8 colors from pic-2.jpg.

- `pigmnts pic-1.jpg -rxdl`  
  Generate a palette of 5 colors from pic-1.jpg and show the RGB code, hex code, dominance, LAB code for each color in the palette.

- `pigmnts pic-1.jpg --count 10 --name --hex`  
  Generate a palette of 10 colors from pic-1.jpg and show the name, hex code for each color in the palette.

- `pigmnts pic-1.jpg pic-2.jpg -sxq`  
  Generate a palette of 5 colors from pic-1.jpg and pic-2.jpg. For each color in the palette show the HSL code, hex code in `quiet` mode.



## Contributing
This repository is a Cargo workspace that includes the development of both core Pigmnts library and the CLI.

### Project Structure
- **Pigmnts Library** - development of the core library is in the `lib` directory.
- **Pigmnts CLI** - root project is Pigmnts CLI that uses the Pigmnts library.


## License
Pigmnts is [MIT Licensed](https://github.com/blenderskool/pigmnts/blob/master/LICENSE.md)  
The dataset for color names used in Pigmnts CLI is taken from https://github.com/meodai/color-names