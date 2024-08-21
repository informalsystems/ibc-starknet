{
  description = "Nix development dependencies for ibc-starknet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    cairo-nix.url = "github:cairo-nix/cairo-nix";
    cosmos-nix-wasm.url = github:informalsystems/cosmos.nix/jonathan/ibc-go-wasm;

    starknet-devnet-src = {
      url = "github:0xSpaceShard/starknet-devnet-rs";
      flake = false;
    };

    cairo-src = {
      url = "github:starkware-libs/cairo/v2.6.4";
      flake = false;
    };

    scarb-src = {
      url = "github:software-mansion/scarb/v2.6.5";
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

          cosmos-nix-wasm = inputs.cosmos-nix-wasm.packages.${system};

          wasm-simapp = cosmos-nix-wasm.ibc-go-v7-wasm-simapp;

          starknet-devnet = import ./nix/starknet-devnet.nix {
            inherit nixpkgs;
            inherit (inputs) starknet-devnet-src;
          };

          cairo = import ./nix/cairo.nix {
            inherit nixpkgs;
            inherit (inputs) cairo-src;
          };

          scarb = import ./nix/scarb.nix {
            inherit nixpkgs;
            inherit (inputs) scarb-src cairo-src;
          };

          rust = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          rust-wasm = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-wasm.toml;

          rust-nightly = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-nightly.toml;

          starknet-pkgs = {
            inherit starknet-devnet cairo scarb wasm-simapp;
          };

          tools = {
            inherit (nixpkgs)
              pkg-config
              protobuf
              cargo-nextest
              taplo
              just
              ;

            nixfmt = nixpkgs.nixfmt-rfc-style;

          };

          shell-deps = (builtins.attrValues starknet-pkgs) ++ (builtins.attrValues tools);
        in
        {
          packages = {
            inherit
              starknet-devnet
              cairo
              scarb
              rust
              rust-nightly
              rust-wasm
              ;
          } // tools // starknet-pkgs;

          devShells = {
            default = nixpkgs.mkShell { buildInputs = shell-deps; };

            rust = nixpkgs.mkShell { buildInputs = [ rust ] ++ shell-deps; };

            rust-nightly = nixpkgs.mkShell { buildInputs = [ rust-nightly ] ++ shell-deps; };

            rust-wasm = nixpkgs.mkShell { buildInputs = [ rust-wasm ] ++ shell-deps; };
          };
        }
      );
}
