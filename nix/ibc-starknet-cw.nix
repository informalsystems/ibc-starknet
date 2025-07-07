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
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-rYDX+VsORm3EdaeKzBr2g6ODkILCxp8w/IFpgGcezAk=";
        "ibc-0.56.0" = "sha256-LncwFd+apWqWht61qblatD0wvNkjO+GjzdUmqkbna6Y=";
        "ibc-client-cw-0.56.0" = "sha256-s07R9VZ9DsYmE9sQOABfebwmy16A2z7Hw+fE+kfqQSw=";
      };
    };

    doCheck = false;

    nativeBuildInputs = [
      rust
      nixpkgs.binaryen
    ];

    buildPhase = ''
      RUSTFLAGS='-C link-arg=-s' cargo build -p ibc-client-starknet-cw --target wasm32-unknown-unknown --release --lib --locked
      RUSTFLAGS='-C link-arg=-s' cargo build -p starknet-crypto-lib --features contract --target wasm32-unknown-unknown --release --lib --locked
    '';

    installPhase = ''
      mkdir -p $out
      wasm-opt -Oz -o $out/ibc_client_starknet_cw.wasm target/wasm32-unknown-unknown/release/ibc_client_starknet_cw.wasm
      wasm-opt -Oz -o $out/starknet_crypto_lib.wasm target/wasm32-unknown-unknown/release/starknet_crypto_lib.wasm
    '';
  };
in
light-client-cw
