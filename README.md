# 🎨 Pigmnts
Pigmnts is a color palette creator from image built using Rust. It uses the [K-means++](https://en.wikipedia.org/wiki/K-means%2B%2B) clustering algorithm to select the most commonly occuring colors from the image.

Pigmnts library is compiled to WebAssembly which allows for super-fast color palette extraction from an image on the web.
The library can be found in the `lib` directory.

## Project Structure
The development of the Pigmnts library is in the `lib` directory.
The root project is a Cargo workspace that uses the Pigmnts library to build a CLI.

## License
Pigmnts is [MIT Licensed](https://github.com/blenderskool/pigmnts/blob/master/LICENSE.md)