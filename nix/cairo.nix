{
  nixpkgs
, cairo-src
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    pname = "cairo";
    version = "2.6.4";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-yuJGG+PlZ5lQ1poil4sdhoEOJYgTQzBwVUy2UgUiUbc=";
  };
in
cairo