{ nixpkgs, cairo-src }:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "cairo";
    version = "2.11.4";

    doCheck = false;

    src = cairo-src;

    cargoHash = "sha256-5LLVrjOBZjBTa8ZFI3mAdg6cnn5GKEMz9azc7eB8uSw=";

    OPENSSL_NO_VENDOR = 1;
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    nativeBuildInputs = [
      nixpkgs.pkg-config
    ];

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
