# attestator

Attest valid Ed25519 signatures and sign them using Starknet ECDSA.

To run the attestator server:

```sh
PRIVATE_KEY=0x1234 cargo run
```

From `curl`:

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:8000/public_key
```

```sh
curl --header 'Content-Type: application/json' http://127.0.0.1:8000/attest \
    --data '[{"message": "[...]", "signature": [...], "public_key": "[...]"}, ...]'
```
