# Getting started with `starknet-devnet` and `cairo` contracts

## Dependencies

- [`scarb`](https://docs.swmansion.com/scarb/download.html)
- [`starknet-devnet`](https://0xspaceshard.github.io/starknet-devnet-rs/docs/running/install)

## Spawning a local devnet

```bash
starknet-devnet --seed=0 --accounts=3 --state-archive-capacity=full
```

This will spawn a local devnet listening on `127.0.0.1:5050`.
`--state-archive-capacity=full` is required for historical state queries.

> [!TIP]\
> Set the environment variables `STARKNET_RPC=http://127.0.0.1:5050` for
> `starkli` or use `--rpc` flag.

You can also use docker to spawn a local devnet:

```bash
docker run -p 5050:5050 -it shardlabs/starknet-devnet-rs --seed=0 --accounts=3 --state-archive-capacity=full
```

## Managing accounts

`starknet-devnet` will output the list of wallets created during the
initialization of the devnet:

```console
$ starknet-devnet
...
Chain ID: SN_SEPOLIA (0x534e5f5345504f4c4941)

| Account address |  0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691
| Private key     |  0x71d7bb07b9a64f6f78ac4c816aff4da9
| Public key      |  0x39d9e6ce352ad4530a0ef5d5a18fd3303c3606a7fa6ac5b620020ad681cc33b

| Account address |  0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1
| Private key     |  0xe1406455b7d66b1690803be066cbe5e
| Public key      |  0x7a1bb2744a7dd29bffd44341dbd78008adb4bc11733601e7eddff322ada9cb

| Account address |  0x49dfb8ce986e21d354ac93ea65e6a11f639c1934ea253e5ff14ca62eca0f38e
| Private key     |  0xa20a02f0ac53692d144b20cb371a60d7
| Public key      |  0xb8fd4ddd415902d96f61b7ad201022d495997c2dff8eb9e0eb86253e30fabc
...
```

You can use `starkli` to generate the keystores of the initialized wallets:

```bash
mkdir -p deployer
starkli signer keystore from-key --private-key-stdin --password "" deployer/keystore_1.json <<< 0x71d7bb07b9a64f6f78ac4c816aff4da9
starkli account fetch 0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691 --output deployer/account_1.json
```

The keystores of the other wallets can be generated similarly.

> [!TIP]\
> Set the environment variables `STARKNET_ACCOUNT=deployer/account_1.json` and
> `STARKNET_KEYSTORE=deployer/keystore_1.json` for `starkli` or use `--account`
> and `--keystore` flags.

## Compile the contract

```bash
cd contracts
scarb build
```

This will compile the contract classes at:
`contracts/targets/dev/*.contract_class.json`

## Declare and deploy the contract

```bash
starkli declare --compiler-version 2.6.2 contracts/target/dev/starknet_ibc_<CONTRACT_NAME>.contract_class.json
```

This will declare the contract class on the _Starknet_. Note down the
`CONTRACT_CLASS` from the output.

## Deploy, query, and update the contract

We will assume the following is our Cairo contract:

```cairo
#[starknet::interface]
trait ISimpleStorage<TContractState> {
     fn set(ref self: TContractState, x: u128);
     fn get(self: @TContractState) -> u128;
}

#[starknet::contract]
mod simple_storage {
    use starknet::get_caller_address;
    use starknet::ContractAddress;

    #[storage]
    struct Storage {
        stored_data: u128
    }

    #[constructor]
    fn constructor(ref self: ContractState, x: u128) {
        self.stored_data.write(x);
    }

    #[abi(embed_v0)]
    impl SimpleStorage of super::ISimpleStorage<ContractState> {
        fn set(ref self: ContractState, x: u128) {
            self.stored_data.write(x);
        }
        fn get(self: @ContractState) -> u128 {
            self.stored_data.read()
        }
    }
}
```

### Deploy the contract with the initial state

```bash
starkli deploy <CONTRACT_CLASS> 0x118 # constructs with 0x118
```

This will deploy the contract on the _Starknet_. Note down the
`CONTRACT_ADDRESS` from the output.

### Query the contract state

```bash
starkli call <CONTRACT_ADDRESS> get # prints 0x118
```

### Update the contract state

```bash
starkli invoke <CONTRACT_ADDRESS> set 0x811 # updates to 0x811
starkli call <CONTRACT_ADDRESS> get # prints 0x811
```

### Passing complex-typed inputs to the contract

In our example contract, the `set` function takes a single argument `x: u128`.
If it was `set(x: u128, y: u128)`, we could have passed the arguments as a list
of numerals `0x118 0x811`. It works the same way for `u8`, `u64` and the other
numeral types as long as they are valid for those types, i.e. passing `256` for
`u8` will fail.

But if the function takes a string or a tuple or an array or a map or a custom
`struct`, we need to serialize these complex input values to a list of
numerals - as `starkli` always takes a list of numerals.

#### String

Cairo uses `felt252` to represent strings. We can use `starkli to-cairo-string`
and `starkli parse-cairo-string` to convert and parse a string to `felt252`,
respectively.

### Tuple

Tuple is serialized as it is.

```
(10, 20, 30): 10, 20, 30
```

#### Array

Array is serialized as a list of numerals where the first numeral is the length
and then the serialized elements.

```
[10,20,30]: 3, 10, 20, 30
```

## References

- [Getting started with Cairo](https://www.cairo-lang.org/tutorials/getting-started-with-cairo)
- [Compile, deploy, interact](https://book.starknet.io/ch02-02-compile-deploy-interact.html)
- [Cairo book](https://book.cairo-lang.org)
- [Serialization of Cairo types](https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types/)
