{
  nixpkgs,
  scarb-src,
  cairo-src,
}:
let
  cairo = nixpkgs.rustPlatform.buildRustPackage {
    name = "scarb";
    version = "2.6.5";

    doCheck = false;

    src = scarb-src;

    cargoHash = "sha256-Lg+Ggc51u73Y7O3KAcTMSO7FhcidKPc0aAcVrTr8lf8=";

    SCARB_CORELIB_LOCAL_PATH = cairo-src + "/corelib";
  };
in
cairo
