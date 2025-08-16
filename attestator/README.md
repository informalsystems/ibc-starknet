# attestator

Attest valid Ed25519 signatures and sign them using Starknet ECDSA.

## Generate Random Felt

Use this for `PRIVATE_KEY` environment variable.

```sh
cargo run --example random_felt
```

## Run Attestator

```sh
PRIVATE_KEY=0x1234 cargo run
```

## Query the Attestator API

### Fetch the public key

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:8000/public_key
```

### Attest a list of Ed25519 challenges

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:8000/attest \
    --data '[{"message": "[...]", "signature": [...], "public_key": "[...]"}, ...]'
# prints the signature (r and s) in bytes
```
