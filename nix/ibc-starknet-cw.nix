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
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-OrNFK6UQjQ7cpj8HqPipG2YcqCV4/KjiV5oRLnoUUyY=";
      };
    };

    doCheck = false;

    nativeBuildInputs = [
      rust
    ];

    buildPhase = ''
      RUSTFLAGS='-C link-arg=-s' cargo build -p ibc-client-starknet-cw --target wasm32-unknown-unknown --release --lib --locked
    '';

    installPhase = ''
      mkdir -p $out
      cp target/wasm32-unknown-unknown/release/ibc_client_starknet_cw.wasm $out/
    '';
  };
in
light-client-cw
