{
  description = "Nix development dependencies for ibc-starknet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    starknet-devnet-src = {
      url = "github:0xSpaceShard/starknet-devnet-rs";
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
        in
        {
          packages = {
            inherit starknet-devnet;
          };

          devShells = {
            default = nixpkgs.mkShell {
              buildInputs = with nixpkgs; [
                starknet-devnet

                pkg-config
                protobuf
                rustc
                cargo
                cargo-nextest

                taplo
                just
                nixfmt-rfc-style
              ];
            };
          };
        }
      );
}
