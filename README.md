# Rust RayTracer
A simple Raytracer defined in Peter Shirley's Raytracing in One Weekend Trilogy implemented in Rust. This is my first project using Rust and performance somewhat shows, but after a eventual rewrite I will run comparison tests and optimize bottlenecks in my rendering.
The code is very straightforward it offers primative abstractions for shapes like spheres, quads, and rectangles, but also has additional features such as:
- Parallelized rendering
- Implements texture mapping onto primatives for smoother textures
- Supports dynamic lighting scenarios
- Utilizes various textures like metal, dieletrics, lambertian, diffusive light, and isotropic
# Example Renders
![Cornell Scene rendered with 10000 samples with final version of Raytracer](https://github.com/Meepst/rust-raytracer/blob/main/pngs/cornell_scene.png)
![Book 2 Final scene rendered with 10000 samples with final version of Raytracer](https://github.com/Meepst/rust-raytracer/blob/main/pngs/final_scene.png)
# Example Usage
```
> cargo run --release > out.ppm
  Compiling `release` profile [optimized] target(s) in 0.88s
    Running `target\release\rust-raytracer.exe`
Pixels rendered: 360000
Done.
```
###
References: https://raytracing.github.io/
