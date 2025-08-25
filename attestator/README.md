# attestator

Attest valid Ed25519 signatures and sign them using Starknet ECDSA.

## Generate Random Felt

Use this for `PRIVATE_KEY` environment variable.

```sh
cargo run --bin random-key
```

## Run Attestator

```sh
ROCKET_PORT=1234 PRIVATE_KEY=0x1234 cargo run --release --bin attestator
```

## Query the Attestator API

### Fetch the public key

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:1234/public_key
# fetches the public key in hex
"0x26da8d11938b76025862be14fdb8b28438827f73e75e86f7bfa38b196951fa7"
```

### Attest a list of Ed25519 challenges

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:1234/attest --data @- <<EOF
[{
    "message": "af82",
    "signature": "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
    "public_key": "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025"
}]
EOF
# fetches the public key and signature (r and s) in hex
["0x26da8d11938b76025862be14fdb8b28438827f73e75e86f7bfa38b196951fa7",[["0x37063480d38eccdc3f7e606a3afdaa56c7ea9a66199650189968f3d3634f82a","0x66d8f1eabf0b29a1b80cb00f8f48bd4c5111c096fd42a239fe3fb87e7a597de"]]]
```
