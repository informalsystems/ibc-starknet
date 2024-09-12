{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.7.1";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-cAEwCX1zGJu4+ufdVSqBbksa1FLZWVNt2TLZ5JlGISk=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";

    buildInputs = nixpkgs.lib.optionals nixpkgs.stdenv.isDarwin [
      nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];
  };
in
cairo
