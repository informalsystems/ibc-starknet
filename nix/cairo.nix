{
  nixpkgs
, cairo-src
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.6.4";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-vmJvWXecLvty7GvoI26Mn4cFlBh2WswKmwBHxiCXFpE=";
  };
in
cairo