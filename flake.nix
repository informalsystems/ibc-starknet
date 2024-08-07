{
  description = "Nix development dependencies for ibc-starknet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    cairo-nix.url = "github:cairo-nix/cairo-nix";

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
            inherit (inputs) scarb-src;
          };
        in
        {
          packages = {
            inherit starknet-devnet cairo scarb;
          };

          devShells = {
            default = nixpkgs.mkShell {
              buildInputs = [
                starknet-devnet
                cairo
                scarb

                nixpkgs.pkg-config
                nixpkgs.protobuf
                nixpkgs.rustc
                nixpkgs.cargo
                nixpkgs.cargo-nextest

                nixpkgs.taplo
                nixpkgs.just
                nixpkgs.nixfmt-rfc-style
              ];
            };
          };
        }
      );
}
