{ nixpkgs, cairo-src }:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.8.4";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-P694ATBnAA9O4aoMDG/3bmcJXOLK0qXQGy0nE9uFMMM=";

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [
      nixpkgs.pkg-config
    ];

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
