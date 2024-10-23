{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.8.4";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-CGMPFE31s1mwkrGCotqSHx8j5hD3J6twGvvG1Rmc63Q=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
