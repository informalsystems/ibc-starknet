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
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-uxXpzxVc89DkAC4VqC/Au3cpBzUbxiSS2KiFKZ+rqdg=";
        "cgp-0.3.1" = "sha256-AOQ+WVQWPlF2ZfYYc5Eq3t7XAljd5P2qExWLYZWNnd8=";
        "ibc-client-cw-0.56.0" = "sha256-EkLxuJfr3vf0busmSZD7DwOS9GfgfhT+sdopi1nNiCs=";
        "ibc-0.56.0" = "sha256-7DPIqu/zs0szjmtJTfXI2eQ0HEkRyvGjArcMZsFWMT4=";
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
