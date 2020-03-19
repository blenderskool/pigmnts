# 🎨 Pigmnts
Pigmnts is a color palette generator built using Rust, compiled to WebAssembly. This allows for super-fast color palette extraction from an image on the web. It uses the [K-means++](https://en.wikipedia.org/wiki/K-means%2B%2B) clustering algorithm to select the most commonly occuring colors from the image.


## Examples with Web Assembly

### As a JavaScript module

```html
<script type="module">
  // Import the functions from CDN
  import init, { pigments } from 'https://unpkg.com/pigmnts/pigmnts.js';

  async function run() {
    /**
     * Load the wasm file.
     * Replace the URL with a different path if you want to use
     * self hosted wasm file 
     */
    await init('https://unpkg.com/pigmnts/pigmnts_bg.wasm');

    // Canvas element from which palette should be created
    const canvas = document.querySelector('canvas');

    // Call the pigments wasm function
    const palette = pigments(canvas, 5);
  }
  run();

</script>
```

### In Node.js

1. Start by installing the npm package
```bash
npm install pigmnts
```

2. Configure your build process to copy the wasm file in the your build directory.

3. Use it in code

```javascript
import init, { pigments } from 'pigmnts';

async function run() {
  // Load the wasm file from path where wasm file was copied by the bundler
  await init('<path to the wasm file>');

  // Canvas element from which palette should be created
  const canvas = document.querySelector('canvas');

  // Call the pigments wasm function
  const palette = pigments(canvas, 5);
}
run();
```


## Functions
Pigmnts exposes following function in WebAssembly
#### pigments(canvas: `HtmlCanvasElement`, num_colors: `u8`, batch_size: `Option<u32>`)
> Returns an object with **[8-digit Hex](https://css-tricks.com/8-digit-hex-codes/) color codes** as keys and dominance(as percentage) of each color as value found in the image. Eg. {"#6DDAD0FF": 0.3, "#FF3A940A": 0.7}

- `canvas` canvas element which has the image to be processed. Internally, the pixel data is taken from the canvas, and then clustered to create the color palette.  
- `num_colors` defines the number of colors to be gathered from the image.  
- `batch_size` (optional) defines the number of pixels to randomly sample from the image. It should be greater than the total number of pixels in the image and the `num_colors`. By default, all the pixels in the image are processed.

If this crate is used in some Rust projects, then following function is also available
#### pigments_pixels(pixels: `&Vec<LAB>`, num_colors: `u8`) -> `Vec<(LAB, f32)>`
> Returns a vector of tuples with colors as `LAB` and dominance(as percentage) of each color found in the image.

- `pixels` reference to a Vector of colors in `LAB` format.
- `num_colors` defines the number of colors to be gathered from the image.

This function can be used when color data is gathered from an image decoded using [image-rs](https://github.com/image-rs/image).

## License
Pigmnts is [MIT Licensed](https://github.com/blenderskool/pigmnts/blob/master/LICENSE.md)