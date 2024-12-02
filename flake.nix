{
  description = "Nix development dependencies for ibc-starknet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    cairo-nix.url = "github:cairo-nix/cairo-nix";
    cosmos-nix.url = "github:informalsystems/cosmos.nix";

    starknet-devnet-src = {
      url = "github:0xSpaceShard/starknet-devnet-rs";
      flake = false;
    };

    cairo-src = {
      url = "github:starkware-libs/cairo/v2.8.4";
      flake = false;
    };

    scarb-src = {
      url = "github:software-mansion/scarb/v2.8.4";
      flake = false;
    };

    universal-sierra-compiler-src = {
      url = "github:software-mansion/universal-sierra-compiler/v2.3.0";
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

          rust = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          rust-wasm = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-wasm.toml;

          rust-nightly = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-nightly.toml;

          rust-1_79 = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-1.79.toml;

          wasm-simapp = cosmos-nix.ibc-go-v8-wasm-simapp;

          starknet-devnet = import ./nix/starknet-devnet.nix {
            inherit nixpkgs;
            inherit (inputs) starknet-devnet-src;

            rust = rust-1_79;
          };

          cairo = import ./nix/cairo.nix {
            inherit nixpkgs;
            inherit (inputs) cairo-src;
          };

          scarb = import ./nix/scarb.nix {
            inherit nixpkgs;
            inherit (inputs) scarb-src cairo-src;
          };

          universal-sierra-compiler = import ./nix/universal-sierra-compiler.nix {
            inherit nixpkgs;
            inherit (inputs) universal-sierra-compiler-src;

            rust = rust-1_79;
          };

          ibc-starknet-cw = import ./nix/ibc-starknet-cw.nix {
            inherit nixpkgs;

            rust = rust-wasm;
          };

          starknet-pkgs = {
            inherit
              starknet-devnet
              cairo
              scarb
              universal-sierra-compiler
              wasm-simapp
              ;
          };

          tools = {
            inherit (nixpkgs)
              protobuf
              cargo-nextest
              taplo
              just
              ;

            nixfmt = nixpkgs.nixfmt-rfc-style;
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
              starknet-devnet
              cairo
              scarb
              rust
              rust-nightly
              rust-wasm
              ibc-starknet-cw
              ;
          } // tools // starknet-pkgs;

          devShells = {
            default = nixpkgs.mkShell { buildInputs = shell-deps; };

            rust = nixpkgs.mkShell { buildInputs = [ rust ] ++ shell-deps; };

            rust-nightly = nixpkgs.mkShell { buildInputs = [ rust-nightly ] ++ shell-deps; };

            rust-wasm = nixpkgs.mkShell { buildInputs = [ rust-wasm ] ++ shell-deps; };
          };

          formatter = tools.nixfmt;
        }
      );
}
