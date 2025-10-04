FROM rust:latest

# Install nightly toolchain and WASM target
RUN rustup toolchain install nightly --allow-downgrade && \
  rustup default nightly && \
  rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install --locked cargo-leptos

# Keep container running for CI jobs
CMD ["bash"]

