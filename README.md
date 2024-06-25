# `miroir`

A minimal yet powerful library for ray reflection simulation in Rust.

Requires the latest stable version of the [Rust Compiler](https://www.rust-lang.org/tools/install).

Powered by [`nalgebra`](https://nalgebra.org/).

## Crates

The core of this project is the [`miroir`](miroir) crate, containing the main `Mirror` trait and primitives used to run simulations. This crate is `#[no_std]` and performs no allocations, making it possible to use it anywhere.

The [`miroir_shapes`](miroir_shapes) crate contains several example implementations of reflective surfaces that can be used in simulations. This is where you should look if you need an example of how to implement your own custom mirror shapes.

There are integrations extending this library with more functionality such as:

- [`miroir_glium`](miroir_glium), which enables running and visualising 2D and 3D simulations using OpenGL.
- [`miroir_numworks`](miroir_numworks), which enables running and visualizing 2D simulations on the screen of your [Numworks Graphing Calculator](https://www.numworks.com/). This serves mainly as an example of [`miroir`](miroir) being used in a bare-metal environment.

Other third-party integrations can easily be created over the simple API of the [`miroir`](miroir) crate. It is advised to check it's documentation:

```shell
cargo doc -p miroir --no-deps --open
```

Be sure to check out the mirrors in the [`miroir_shapes`](miroir_shapes) crate, and make sure to implement for them whatever new functionality you expose.

### Controls for `miroir_glium`

The [`miroir_glium`](miroir_glium) crate allows viewing simulations where you can move around and rotate the camera. Here are the controls:

- Use the WASD keys (or ZQSD) to move forward, left, backward, and right, respectively.
- Use the space bar to move up and the shift key to move down.
- Click and drag your mouse on the screen to look around, and rotate the camera.
- Use the right/left arrow key to increase/decrease camera rotation sensitivity.
- Use the up/down key to increase/decrease movement speed.

Currently, the ray's path is drawn in white, and the portion of the path that loops infinitely (if it exists) is drawn in pink. (TODO: allow user-setting these)

Examples of simulations you can run (and how to create them) can be found in the [`miroir_glium/examples`](miroir_glium/examples) directory. Use the following command to run one.

```shell
cargo run -r -p miroir_glium --example <example_name>
```

where `<example_name>` is the name of the example's source file (without the trailing `.rs`)

#### TODOs and known issues

- Lots and lots of documentation
- [`miroir_glium`](miroir_glium) visualisations lack customization, mainly in the choice of colors...
- 3D simulations in [`miroir_glium`](miroir_glium) lack any kind of lighting, and hence viewing complex, curved, surfaces is awkward. I am not (yet?) well-versed enough in 3D rendering to know how to implement this neatly.
