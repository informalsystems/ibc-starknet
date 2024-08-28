{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.7.0";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-EYepqNEixfuwBQDWUAkIUpJcyTwkh6bnBck+M4VROMY=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";
  };
in
cairo
