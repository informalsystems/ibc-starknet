{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.9.2";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-kk7s2GEZKsG98ej1OWymhwP/NK2k7BJPBxIBM2vIuRE=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
