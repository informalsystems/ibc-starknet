{ nixpkgs, snforge-src, cairo-src }:
let
  snforge = nixpkgs.rustPlatform.buildRustPackage {
    name = "snforge";
    version = "0.26.0";

    doCheck = false;

    src = snforge-src;

    cargoHash = "";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";
  };
in
cairo
