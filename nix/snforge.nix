{ nixpkgs, snforge-src }:
let
  snforge = nixpkgs.rustPlatform.buildRustPackage {
    pname = "forge";
    version = "0.31.0";

    src = snforge-src;

    cargoLock = {
      lockFile = snforge-src + "/Cargo.lock";
      outputHashes = {
        "starknet-0.11.0" = "sha256-Dgx5Czrzj2JKwmSJ5EvqpikRFwpWwEydkhZl0pnjfWE=";
        "trace-data-0.4.0" = "sha256-C5rgp+wthWkjNBkY1PlHfLkGexrmjOQpUgbPKPrKf7g=";
      };
    };

    doCheck = false;

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [ nixpkgs.pkg-config ];

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
snforge
