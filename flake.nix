{
  description = "Portfolio Rust — Leptos 0.8 full-stack (SSR + CSR/WASM)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        rustToolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.latest.rustc
          fenix.packages.${system}.latest.cargo
          fenix.packages.${system}.latest.rust-src
          fenix.packages.${system}.latest.clippy
          fenix.packages.${system}.latest.rustfmt
          fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std
        ];

        rustAnalyzer = fenix.packages.${system}.latest.rust-analyzer;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain
            rustAnalyzer

            # Build cargo-leptos (SSR + WASM all in one)
            pkgs.cargo-leptos

            # wasm-bindgen-cli
            pkgs.wasm-bindgen-cli

            # Tailwind CSS v4 (tailwind-input-file in leptos config)
            pkgs.tailwindcss_4

            # Reqwest dependencies
            pkgs.pkg-config
            pkgs.openssl.dev
          ];

          buildInputs = [
            pkgs.openssl
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          RUST_LOG = "info";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "→ cargo leptos watch   : dev  with hot-reload (port 3000 / reload 3001)"
            echo "→ cargo leptos build   : production build target/site"
          '';
        };
      }
    );
}
