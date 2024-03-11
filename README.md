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

## Testing Locally

To test the application locally, run:

```bash
cargo run --release
```

### Linux

If you're on Linux, you need to install some packages first:

```bash
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

### Fedora Rawhide

If you're on Fedora Rawhide, you need to install some packages first:

```bash
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

## Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly)
and publish it as a web page. We use [Trunk](https://trunkrs.dev/) to build for
the web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will
   rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser.

> Note: The `assets/sw.js` script will try to cache our app, and loads the
> cached version when it cannot connect to the server allowing your app to work
> offline (like PWA). Appending `#dev` to `index.html` will skip this caching,
> allowing us to load the latest builds during development.

## Web Deploy

1. Run `trunk build --release` to generate a `dist` directory as a "static html"
   website.
2. Upload the `dist` directory to any of the numerous free hosting websites
   including
   [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
3. We already provide a workflow that auto-deploys our app to GitHub pages if
   you enable it.

> To enable Github Pages, you need to go to Repository -> Settings -> Pages ->
> Source -> set to `gh-pages` branch and `/` (root). If `gh-pages` is not
> available in `Source`, just create and push a branch called `gh-pages` and it
> should be available.
