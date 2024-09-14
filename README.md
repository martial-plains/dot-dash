<h1 align="center">
  <img src="docs/icon-256.png" alt="Dot Dash Icon" width="192" height="192"/>
  <br>
  Dot Dash
</h1>

<p align="center"><strong>Morse Code Translator</strong></p>

<p align="center">
  <img src="/docs/screenshots/dot dash light.png" alt="Preview"/>
</p>

Dot Dash is a powerful and user-friendly application that translates Morse code.
It’s built with Rust, a language renowned for its performance and safety. The
application is designed to be intuitive and easy to use, making Morse code
translation accessible to everyone, from hobbyists to professionals.

The application works by accepting Morse code as input and translating it into
readable text. This can be incredibly useful in a variety of scenarios, such as
deciphering Morse code messages or learning Morse code for amateur radio usage.

Moreover, Dot Dash is not just a local application. It can also be compiled to
WebAssembly (WASM) and published as a web page, making it accessible from any
device with a web browser. This feature broadens the reach of the application,
allowing users to access it anytime, anywhere.

In addition, Dot Dash is designed with a focus on performance. It leverages the
power of Rust to ensure that translations are done quickly and efficiently. This
makes it a reliable tool for anyone who needs to translate Morse code.

## Table of Contents

- [Getting Started](#getting-started)
- [Testing Locally](#testing-locally)
- [Web Locally](#web-locally)
- [Web Deploy](#web-deploy)

## Getting Started

Ensure you are using the latest version of stable Rust. Update your version by
running:

```bash
rustup update
```

Next, lets install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

## Testing Locally

To test the application locally, run:

```bash
cargo run --release
```

### Linux

If you're on Linux, you need to install some packages first:

```bash
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev libxdo-dev libasound2-dev
```

> To enable Github Pages, you need to go to Repository -> Settings -> Pages ->
> Source -> set to `gh-pages` branch and `/` (root). If `gh-pages` is not
> available in `Source`, just create and push a branch called `gh-pages` and it
> should be available.
