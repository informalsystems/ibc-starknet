{ nixpkgs, cairo-src }:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.7.0";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-yufWRaLxazbo64jjbH4MGLWqF7K7dWZhBjIBHLaRVHw=";
  };
in
cairo
