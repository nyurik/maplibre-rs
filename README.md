<h1 align="center">
  <img width="350px" alt="maplibre-rs" src="./docs/logo/maplibre-rs-with-text.svg">
</h1>

<div align="center">
  <strong>Native Maps for Web, Mobile and Linux</strong>
</div>
<div align="center">
  A map rendering library written in Rust.
</div>

<div align="center">
  <img src="https://img.shields.io/badge/stability-experimental-orange.svg?style=flat-square"
      alt="Stability" />
  <a href="https://github.com/maplibre/maplibre-rs/actions/workflows/on_main_push.yml">
    <img src="https://github.com/maplibre/maplibre-rs/actions/workflows/on_main_push.yml/badge.svg"
        alt="Build status" />
  </a>
  <a href="https://matrix.to/#/#maplibre:matrix.org">
    <img src="https://img.shields.io/static/v1?label=Space&message=%23maplibre&color=blue&logo=matrix"
        alt="Build status" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://maxammann.org/maplibre-rs/webgl">
      WebGL Demo
    </a>
    <span> | </span>
    <a href="https://maxammann.org/maplibre-rs/docs">
      Book
    </a> | </span>
    <a href="https://maxammann.org/maplibre-rs/api-docs/maplibre/">
      API
    </a> | </span>
    <a href="https://matrix.to/#/#maplibre:matrix.org">
      Chat in Matrix Space
    </a>
  </h3>
</div>

## Project State

This project is in a proof-of-concept state. The proof of concept is done except for text rendering.
The Rust ecosystem is suited very well for this project.

In the future, this project could be adopted and supported by [Maplibre](https://github.com/maplibre) to implement a
next-gen mapping solution.

## Description

maplibre-rs is a portable and performant vector maps renderer. We aim to support web, mobile and desktop applications. This
is achieved by the novel [WebGPU](https://www.w3.org/TR/webgpu/) specification. Plenty of native implementations are
already implementing this specification. On the web, it is implemented by Firefox, Chrome and Safari. There are also
standalone implementations that directly use Vulkan, OpenGL or Metal as a backend. Those backends allow maplibre-rs to run on
mobile and desktop applications.

Rust is used as a Lingua-franka on all platforms. This is made possible by WebAssembly, which allows us to use Rust for
web development.

The goal of maplibre-rs is to render maps to visualize data. Right now the goal of maplibre-rs is not to replace existing
vector map renderers like Google Maps, Apple Maps or MapLibre. The current implementation serves as a proof-of-concept
of the used technology stack. It is unclear whether the high-performance requirements of rendering maps using vector
graphics are achievable using the current stack.

## Talk: World in Vectors

https://user-images.githubusercontent.com/905221/163552617-5db04c66-23e3-4915-87c1-25185d96730a.mp4

([YouTube](https://www.youtube.com/watch?v=KFk8bOtJzCM))

## Current Features

* Runs on Linux, Android, iOS, macOS, Firefox and Chrome
* Render a vector tile dataset
* Simple navigation powered by winit
* Multithreaded on all platforms
* Querying feature data

## Missing Features

* Rendering Text
* Per-Feature Rendering
* Rendering:
    * Labels
    * Symbols
    * Raster data
    * 3D terrain
    * Hill-shade (DEM)
* Collision detection
* Support for:
    * GeoJSON
* API for:
    * TypeScript
    * Swift
    * Java/Kotlin

## Building & Running

Now, to clone the project:

```bash
git clone --recursive git@github.com:maplibre/maplibre-rs
```

and then build it for running on a desktop:

```bash
cargo build
```

After that, you can run it on your desktop:

```bash
cargo run -p maplibre-demo
```

More information about building for different platforms can be
found [here](https://maxammann.org/maplibre-rs/docs/development-guide/building.html).

> __Note for Mac__: Before opening the XCode project, you need to build manually using the following command:
> `cargo build --target aarch64-apple-darwin --lib`
>
> After that, open the XCode project and run it.
> (XCode seems to set some environment variables which cause problems with the build directly within XCode)

## Rust Setup

Install [rustup](https://rustup.rs/) because this is the recommended way of setting up Rust toolchains.

The toolchain will be automatically downloaded when building this project.
See [./rust-toolchain.toml](./rust-toolchain.toml) for more details about the toolchain.

## Documentation

This generates the documentation for this crate and opens the browser. This also includes the documentation of every
dependency.

```bash
cargo doc --open
```

You can also view the up-to-date documentation [here](https://maxammann.org/maplibre-rs/docs/).
