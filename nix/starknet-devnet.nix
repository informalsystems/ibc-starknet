{
  nixpkgs
, starknet-devnet-src
}:
let
  starknet-devnet = nixpkgs.rustPlatform.buildRustPackage {
    name = "starknet-devnet";
    src = starknet-devnet-src;

    cargoLock = {
      lockFile = starknet-devnet-src + "/Cargo.lock";
    };

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [
      nixpkgs.pkg-config
    ];

    doCheck = false;
  };
in
starknet-devnet