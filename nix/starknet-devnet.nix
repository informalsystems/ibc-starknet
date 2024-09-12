{
  nixpkgs,
  rust,
  starknet-devnet-src,
}:
let
  rust-platform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };

  starknet-devnet = rust-platform.buildRustPackage {
    name = "starknet-devnet";
    src = starknet-devnet-src;

    cargoLock = {
      lockFile = starknet-devnet-src + "/Cargo.lock";
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
starknet-devnet
