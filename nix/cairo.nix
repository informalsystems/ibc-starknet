{ nixpkgs, cairo-src }:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.11.2";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-1SGbhusL5Ig+/8oVRxicVkdLfuLwK1Z0Bq4oR7Tei3A=";

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
