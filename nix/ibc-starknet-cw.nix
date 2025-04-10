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
        "cgp-0.4.0" = "sha256-zqkD2Wxesvdlk98ZxCZdrY/iM+AY6yVcNoMnUAyQFGM=";
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-17kXXPs3Gw+DrddHIBHbNpw6CRTsAEaY3r6NfK9k0e4=";
        "ibc-client-cw-0.56.0" = "sha256-83IMkALyfAiTIJonK+0ti/teoJXR6iEepLfPaI+gE0I=";
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
