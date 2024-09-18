{ nixpkgs, snforge-src }:
let
  snforge = nixpkgs.rustPlatform.buildRustPackage {
    pname = "forge";
    version = "0.27.0";

    src = snforge-src;

    cargoLock = {
      lockFile = snforge-src + "/Cargo.lock";
      outputHashes = {
        "starknet-0.10.0" = "sha256-/cDjAPsNQNtO/kTUK6PpaxyTgAMc6LhfXxrcfom20fE=";
        "trace-data-0.3.0" = "sha256-wtAdQ4Z/p8s9f8T32FXV4TqKmVbCFDBPx/Xjet4UgsI=";
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
