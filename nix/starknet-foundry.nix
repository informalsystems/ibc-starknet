{ nixpkgs, starknet-foundry-src }:
let
  starknet-foundry = nixpkgs.rustPlatform.buildRustPackage {
    pname = "starknet-foundry";
    version = "0.37.0";

    src = starknet-foundry-src;

    cargoLock = {
      lockFile = starknet-foundry-src + "/Cargo.lock";
      outputHashes = {
        "starknet-0.11.0" = "sha256-Dgx5Czrzj2JKwmSJ5EvqpikRFwpWwEydkhZl0pnjfWE=";
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
starknet-foundry
