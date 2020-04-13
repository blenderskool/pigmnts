# 🎨 Pigmnts
Pigmnts is a library to create a color palette from an image built using Rust, compiled to WebAssembly. This allows for super-fast color palette extraction from an image on the web. It uses the [K-means++](https://en.wikipedia.org/wiki/K-means%2B%2B) clustering algorithm to select the most commonly occuring colors from the image.


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
#### pigments(canvas: `HtmlCanvasElement`, k: `number`, mood: `Mood|number`, batch_size: `number`)

##### Arguments
- `canvas` canvas element which has the image to be processed. Internally, the pixel data is taken from the canvas, and then clustered to create the color palette.  
- `k` defines the number of colors to be gathered from the image.  
- `mood` defines the weight function to use. Only 'dominant' mood is supported which has a value of `0`
- `batch_size` (optional) defines the number of pixels to randomly sample from the image. It should be greater than the total number of pixels in the image and the `k`. By default, all the pixels in the image are processed.

##### Return
Returns an Array of Objects where each Object is a color of the following format.
```javascript
[
  {
    dominance: 0.565    // Dominance of color in image(From 0 to 1)
    hex: '#6DDAD0'      // 6-digit Hex color code
    rgb: {              // Equivalent RGB color
      r: 109,
      g: 218,
      b: 208
    },
    hsl: {             // Equivalent HSL color (Normalized to 0-1)
      h: 0.48333,
      s: 0.6,
      l: 0.64,
    }
  },
  // Other colors
  {
    ...
  }
]
```

If this crate is used in some Rust projects, then following function is also available
#### pigments_pixels(pixels: `&Vec<LAB>`, k: `u8`, weight: `fn(&LAB) -> f32`, max_iter: `Option<u16>`) -> `Vec<(LAB, f32)>`

This function can be used when color data is gathered from an image decoded using [image-rs](https://github.com/image-rs/image).

##### Arguments
- `pixels` reference to a Vector of colors in `LAB` format.
- `k` defines the number of colors to be gathered from the image.
- `weight` defines the weight function to use. `src/weights.rs` file has few implemented weight functions.
- `max_iter` defines the maximum iterations that algorithm makes, default is `300`

##### Return
Returns a vector of tuples with colors as `LAB` and dominance(as percentage) of each color found in the image.


## License
Pigmnts is [MIT Licensed](https://github.com/blenderskool/pigmnts/blob/master/LICENSE.md)