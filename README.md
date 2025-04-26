# Personal Website
Personal website made in Rust as a hobby project and place to collect a writeup of my projects.

## Technologies
- Rust & Yew
- Gloo for DOM bindings
- Trunk for building and bundling
- PureCSS, Google Fonts, Font Awesome

## Prerequisites
- Rust
- WASM target:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
- Trunk CLI:
    ```bash
    cargo install trunk
    ```

## Getting started
1. Clone repo
2. Install deps and build
    ```bash
    trunk build
    ```
3. Serve locally
    ```bash
    trunk serve
    ```