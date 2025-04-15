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
    version = "0.41.0";

    src = starknet-foundry-src;

    cargoLock = {
      lockFile = starknet-foundry-src + "/Cargo.lock";
      outputHashes = {
        "blockifier-0.14.0-rc.3" = "sha256-q4Ook6bAFWh1ACr2cjjMO8xsLlLxgnHj7P4cPurkgYs=";
        "starknet-0.14.0" = "sha256-zu0LkPx/ddK0E5S3cHd5krb7FpcULkD6/cQEXW14zTI=";
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
