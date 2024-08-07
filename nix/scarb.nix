{
  nixpkgs
, scarb-src
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    pname = "scarb";
    version = "2.6.5";

    doCheck = false;

    src = scarb-src;

    cargoHash = "";
  };
in
cairo