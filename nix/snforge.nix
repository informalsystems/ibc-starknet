{ nixpkgs, snforge-src }:
let
  snforge = nixpkgs.rustPlatform.buildRustPackage {
    pname = "forge";
    version = "0.29.0";

    src = snforge-src;

    cargoLock = {
      lockFile = snforge-src + "/Cargo.lock";
      outputHashes = {
        "starknet-0.10.0" = "sha256-/cDjAPsNQNtO/kTUK6PpaxyTgAMc6LhfXxrcfom20fE=";
        "trace-data-0.4.0" = "sha256-C5rgp+wthWkjNBkY1PlHfLkGexrmjOQpUgbPKPrKf7g=";
      };
    };

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [ nixpkgs.pkg-config ];

    doCheck = false;
  };
in
snforge
