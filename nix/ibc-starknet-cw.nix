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
        "hermes-cosmos-encoding-components-0.1.0" = "sha256-4gB3VDFB4oiHfN1DFsVBoqCzLrkp392jIQpaAoVNQ4k=";
        "ibc-client-cw-0.56.0" = "sha256-DA3AB8ejUrx4ksBtN/vaOznjpKE0+0F6vGA7JmWyHWA=";
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
