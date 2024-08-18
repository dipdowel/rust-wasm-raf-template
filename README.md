# Rust WASM requestAnimationFrame() template

This is a template for a Rust / WASM project which needs to call `requestAnimationFrame()` from within rust code in order to animate HTML canvas.
The template is based on the [brilliant example](https://gist.github.com/sifyfy/2802e0e7f072c02b0268b123c73779e9) by [sifyfy](https://gist.github.com/sifyfy).

## Live demo
[See the template in action](https://codument.com/rust-raf/).<br>


## How to build
```sh
git clone git@github.com:dipdowel/rust-wasm-raf-template.git
cd rust-wasm-raf-template
wasm-pack build --target web
```


## How to run
While in `rust-wasm-raf-template`:
```sh 
python3 -m http.server
```
Then open your browser and go to `http://localhost:8000`.


## Keywords
Rust, wasm, WebAssembly requestAnimationFrame, requestAnimationFrame(), request_animation_frame, request_animation_frame() canvas, animation, template