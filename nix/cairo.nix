{ nixpkgs, cairo-src }:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.7.1";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-piabK7UNyt2hWoqR5IDnCZoo8+VvSthSao3sQXKjE0o=";

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
