{
  description = "Nix development dependencies for ibc-starknet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    cairo-nix.url = "github:cairo-nix/cairo-nix";
    cosmos-nix.url = "github:informalsystems/cosmos.nix";

    cairo-src = {
      url = "github:starkware-libs/cairo/v2.11.4";
      flake = false;
    };

    universal-sierra-compiler-src = {
      url = "github:software-mansion/universal-sierra-compiler/v2.5.0";
      flake = false;
    };

    starknet-foundry-src = {
      url = "github:foundry-rs/starknet-foundry/v0.42.0";
      flake = false;
    };
  };

  outputs =
    inputs:
    let
      utils = inputs.flake-utils.lib;
    in
    utils.eachSystem
      [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
      ]
      (
        system:
        let
          nixpkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          cosmos-nix = inputs.cosmos-nix.packages.${system};

          rust = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-stable.toml;

          rust-wasm = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-wasm.toml;

          rust-nightly = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-nightly.toml;

          wasm-simapp = cosmos-nix.ibc-go-v8-wasm-simapp;

          osmosis = cosmos-nix.osmosis;

          cairo = import ./nix/cairo.nix {
            inherit nixpkgs;
            inherit (inputs) cairo-src;
          };

          universal-sierra-compiler = import ./nix/universal-sierra-compiler.nix {
            inherit nixpkgs;
            inherit (inputs) universal-sierra-compiler-src;

            inherit rust;
          };

          starknet-foundry = import ./nix/starknet-foundry.nix {
            inherit nixpkgs;
            inherit (inputs) starknet-foundry-src;

            inherit rust;
          };

          ibc-starknet-cw = import ./nix/ibc-starknet-cw.nix {
            inherit nixpkgs;

            rust = rust-wasm;
          };

          starknet-pkgs = {
            inherit
              cairo
              universal-sierra-compiler
              wasm-simapp
              osmosis
              ;
          };

          tools = {
            inherit (nixpkgs)
              protobuf
              cargo-nextest
              taplo
              just
              openssl
              pkg-config
              ;
          };

          mac-deps = nixpkgs.lib.optional nixpkgs.stdenv.isDarwin [
            nixpkgs.libiconv
            nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          shell-deps = (builtins.attrValues starknet-pkgs) ++ (builtins.attrValues tools) ++ mac-deps;
        in
        {
          packages = {
            inherit
              cairo
              rust
              rust-nightly
              rust-wasm
              ibc-starknet-cw
              ;
          }
          // tools
          // starknet-pkgs;

          devShells = {
            default = nixpkgs.mkShell { buildInputs = shell-deps; };

            rust = nixpkgs.mkShell {
              PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";
              buildInputs = [ rust ] ++ shell-deps;
            };

            rust-nightly = nixpkgs.mkShell { buildInputs = [ rust-nightly ] ++ shell-deps; };

            rust-wasm = nixpkgs.mkShell { buildInputs = [ rust-wasm ] ++ shell-deps; };
          };

          formatter = nixpkgs.nixfmt-tree;
        }
      );
}
