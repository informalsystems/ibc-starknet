<div align="center">
    <h1>Starknet IBC</h1>
</div>

# Dependencies

- [`scarb`](https://docs.swmansion.com/scarb/download.html)
- [`starknet-devnet`](https://0xspaceshard.github.io/starknet-devnet-rs/docs/running/install)

# Spawning a local devnet

```bash
starknet-devnet --seed=10 --accounts=10 --state-archive-capacity=full
```

This will spawn a local devnet listening on `127.0.0.1:5050`.
`--state-archive-capacity=full` is required for historical state queries.

> [!TIP]\
> Set the environment variables `STARKNET_RPC=http://127.0.0.1:5050` for
> `starkli` or use `--rpc` flag.

One can also use docker to spawn a local devnet:

```bash
docker run -p 5050:5050 -it shardlabs/starknet-devnet-rs --seed=10 --accounts=10 --state-archive-capacity=full
```

# Managing accounts

`starknet-devnet` will output the list of wallets created during the
initialization of the devnet.

```console
$ starknet-devnet
...
Chain ID: SN_SEPOLIA (0x534e5f5345504f4c4941)

| Account address |  0x275c4e8756d0a8bd080d828f29fbb61968981e4db8ad61b9eaeb17174242e14
| Private key     |  0xb8b1cd314c9a6937a5e107569cd95ca2
| Public key      |  0x4db40f58a193b10847a1818f95e30034a49a8134fa023dfc3458e71e2ce3e0e

| Account address |  0x143aa22f30a57b53ba1da7e1be0ef9fbee1a2471a742d76cb4ca7e796109d74
| Private key     |  0x3058ec575eca8f66bfb69dbc47b2d072
| Public key      |  0x1e2b222b37c77404387e06e3100c989f0232c605b00a99a64fabf36783cf52

| Account address |  0xebb585e292959995cca31508b4938d2d191a986fc5b5238184e6544ff5622
| Private key     |  0xc7cf4155cc1d3b424f5029f8696253d6
| Public key      |  0x78e998bbf1e67e79a8df3ffc9ca7f3a67cd9e510b2be921f8fefbfff0535017
...
```

One can use `starkli` to generate the keystores of the initialized wallets:

```bash
mkdir -p deployer
starkli signer keystore from-key --private-key-stdin --password "" deployer/keystore_1.json <<< 0xb8b1cd314c9a6937a5e107569cd95ca2
starkli account fetch 0x275c4e8756d0a8bd080d828f29fbb61968981e4db8ad61b9eaeb17174242e14 --output deployer/account_1.json
```

One may add the other accounts similarly.

> [!TIP]\
> Set the environment variables `STARKNET_ACCOUNT=deployer/account_1.json` and
> `STARKNET_KEYSTORE=deployer/keystore_1.json` for `starkli` or use `--account`
> and `--keystore` flags.

# Compile the contract

```bash
cd contracts
scarb build
```

This will put the contract classes at:
`contracts/targets/dev/*.contract_class.json`

# Declare and deploy the contract

```bash
starkli declare --compiler-version 2.6.2 --nonce 0 contracts/target/dev/starknet_ibc_simple_storage.contract_class.json
```

This will declare the contract class. Note down the `CONTRACT_CLASS` from the
output.

> [!NOTE]\
> `--nonce` is required for
> [`starknet-devnet-rs`](https://0xspaceshard.github.io/starknet-devnet-rs/docs/account-impersonation).

```bash
starkli deploy <CONTRACT_CLASS>
```

This will deploy the contract. Note down the `CONTRACT_ADDRESS` from the output.

# Interact with the contract

## Query the contract

```bash
starkli call <CONTRACT_ADDRESS> get # should print 0x00
```

## Update the contract

```bash
starkli invoke --account deployer/account_1.json <CONTRACT_ADDRESS> set 0x118
starkli call <CONTRACT_ADDRESS> get # should print 0x118
```

# References

- [Getting started with Cairo](https://www.cairo-lang.org/tutorials/getting-started-with-cairo)
- [Compile, deploy, interact](https://book.starknet.io/ch02-02-compile-deploy-interact.html)
