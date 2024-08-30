{ nixpkgs, snforge-src }:
let
  snforge = nixpkgs.rustPlatform.buildRustPackage {
    name = "snforge";
    version = "0.26.0";

    src = snforge-src;

    cargoLock = {
      lockFile = snforge-src + "/Cargo.lock";
      outputHashes = {
        "starknet-0.9.0" = "sha256-E3FyLhVxauojw2L2AFvDlntwXFGnsGKfi/YHWHrJRy0=";
        "trace-data-0.3.0" = "sha256-wtAdQ4Z/p8s9f8T32FXV4TqKmVbCFDBPx/Xjet4UgsI=";
      };
    };

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [ nixpkgs.pkg-config ];

    doCheck = false;
  };
in
snforge
