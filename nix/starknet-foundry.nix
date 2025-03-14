{
  nixpkgs,
  rust,
  starknet-foundry-src,
}:
let
  rust-platform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };

  starknet-foundry = rust-platform.buildRustPackage {
    pname = "starknet-foundry";
    version = "0.38.3";

    src = starknet-foundry-src;

    cargoLock = {
      lockFile = starknet-foundry-src + "/Cargo.lock";
      outputHashes = {
        "blockifier-0.14.0-rc.1" = "sha256-5BsLlwTJeRkAeO3o8bOqb2FEcRiOy1Nvf1dQ78qRP58=";
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
