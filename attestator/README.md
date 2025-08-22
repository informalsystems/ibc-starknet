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
```

### Attest a list of Ed25519 challenges

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:1234/attest \
    --data '[{"message": "[...]", "signature": [...], "public_key": "[...]"}, ...]'
# prints the signature (r and s) in bytes
```
