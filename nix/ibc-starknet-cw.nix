{
  nixpkgs,
  rust,
}:
let
  light-client-cw = nixpkgs.rustPlatform.buildRustPackage {
    name = "ibc-startknet-cw";
    src = ./../light-client;

    cargoLock = {
      lockFile = ./../light-client/Cargo.lock;
      outputHashes = {
        "cgp-0.4.0" = "sha256-YaK4YaT/1jbudEh7YnQkH2KrPmjbSI5vBL8HYU1eREg=";
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-WLhdFcAi3WVg/ZrMC/2UISstLaJAfeGhEtJQ7+xjhOE=";
        "ibc-0.56.0" = "sha256-6ebsNVErJrLnxijsRo+xqDMZQH+Ef160OyRFBNhCT4U=";
        "ibc-client-cw-0.56.0" = "sha256-xJgM0wyKlN5gaCKgA+y9R/l4QpZ3GbNsoJ8ZrPLvvJ8=";
      };
    };

    doCheck = false;

    nativeBuildInputs = [
      rust
      nixpkgs.binaryen
    ];

    buildPhase = ''
      cargo build -p ibc-client-starknet-cw --target wasm32-unknown-unknown --release --lib --locked
      cargo build -p starknet-crypto-lib --features contract --target wasm32-unknown-unknown --release --lib --locked
    '';

    installPhase = ''
      mkdir -p $out
      wasm-opt -Oz -o $out/ibc_client_starknet_cw.wasm target/wasm32-unknown-unknown/release/ibc_client_starknet_cw.wasm
      wasm-opt -Oz -o $out/starknet_crypto_lib.wasm target/wasm32-unknown-unknown/release/starknet_crypto_lib.wasm
    '';
  };
in
light-client-cw
