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

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [ nixpkgs.pkg-config ];

    doCheck = false;
  };
in
starknet-devnet
