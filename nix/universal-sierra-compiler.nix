{
  nixpkgs,
  rust,
  universal-sierra-compiler-src,
}:

let
  rust-platform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };

  universal-sierra-compiler = rust-platform.buildRustPackage {
    pname = "universal-sierra-compiler";
    version = "2.2.0";

    src = universal-sierra-compiler-src;

    cargoLock = {
      lockFile = universal-sierra-compiler-src + "/Cargo.lock";
      outputHashes = {
        "cairo-lang-casm-1.0.0-alpha.6" = "sha256-U4kTAAktXK7bFEkQAISQK3954hDkyxdsJU9c5hXmzpU=";
      };
    };

    doCheck = false;
  };
in
universal-sierra-compiler
