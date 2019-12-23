# 🎨 Pigmnts
Pigmnts is a color palette generator built using Rust, compiled to WebAssembly. This allows for super-fast color palette extraction from an image on the web. It uses the [K-means++](https://en.wikipedia.org/wiki/K-means%2B%2B) clustering algorithm to select the most commonly occuring colors from the image.

## Functions
Pigmnts exposes following function in WebAssembly
### pigments(canvas: `HtmlCanvasElement`, num_colors: `u8`, batch_size: `Option<u32>`)
> Returns an object with **[8-digit Hex](https://css-tricks.com/8-digit-hex-codes/) color codes** as keys and dominance(as percentage) of each color as value found in the image. Eg. {"#6DDAD0FF": 0.3, "#FF3A940A": 0.7}

- `canvas` canvas element which has the image to be processed. Internally, the pixel data is taken from the canvas, and then clustered to create the color palette.  
- `num_colors` defines the number of colors to be gathered from the image.  
- `batch_size` (optional) defines the number of pixels to randomly sample from the image. It should be greater than the total number of pixels in the image and the `num_colors`. By default, all the pixels in the image are processed.

If this crate is used in some Rust projects, then following function is also available
### pigments_pixels(pixels: `&Vec<RGBA>`, num_colors: `u8`) -> `Vec<(RGBA, f32)>`
> Returns a vector of tuples with **[8-digit Hex](https://css-tricks.com/8-digit-hex-codes/) color codes** as strings and dominance(as percentage) of each color found in the image. Eg. [("#6DDAD0FF", 0.3), ("#FF3A940A", 0.7)]

This function takes a reference to a Vector of `RGBA` which contains the color data, and `num_colors` to limit the number of colors found. This function can be used when color data is gathered from an image decoded using [image-rs](https://github.com/image-rs/image).  
**NOTE**: The `RGBA` struct is different from implementations in other crates such as image-rs.

## License
Pigmnts is [MIT Licensed](https://github.com/blenderskool/pigmnts/blob/master/LICENSE.md)