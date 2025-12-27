# Portfolio

## Descirption

If you don't have Rust nightly, you can install it with
```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```

## Developing your Leptos CSR project

To develop your Leptos CSR project, running

```sh
cargo leptos watch
```

will start a development server at `http://localhost:3000` with hot-reload enabled.


## Deploying your Leptos CSR project

To build a Leptos CSR app for release, use the command

```sh
cargo leptos build --release
```

This will output the files necessary to run your app into the `target/site/` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].

[Leptos]: https://github.com/leptos-rs/leptos
[cargo-leptos]: https://github.com/leptos-rs/cargo-leptos
[deploy-csr]: https://book.leptos.dev/deployment/csr.html
