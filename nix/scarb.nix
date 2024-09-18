{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.8.0";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-LodT2SCTrIrY8YGJWYbzgawIz3xJn9VH2rst07av2Bw=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
