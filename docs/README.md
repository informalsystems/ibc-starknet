# Get started with `starknet-devnet` and `cairo` contracts

## Dependencies

- [`scarb`](https://docs.swmansion.com/scarb/download.html)
- [`starknet-devnet`](https://0xspaceshard.github.io/starknet-devnet-rs/docs/running/install)
- [`cainome`](https://github.com/cartridge-gg/cainome)

## Spawn a local devnet

```bash
starknet-devnet --seed=0 --accounts=3 --state-archive-capacity=full
```

This will spawn a local devnet listening on `127.0.0.1:5050`.
`--state-archive-capacity=full` is required for historical state queries.

> [!TIP]\
> Set the environment variables `STARKNET_RPC=http://<HOST>:<PORT>` for
> `starkli` or use `--rpc` flag.

You can also use docker to spawn a local devnet:

```bash
docker run -p 5050:5050 -it shardlabs/starknet-devnet-rs --seed=0 --accounts=3 --state-archive-capacity=full
```

## Manage accounts and contracts

`starknet-devnet` will output the list of wallets created during the
initialization of the devnet:

```console
$ starknet-devnet --seed 0 --accounts 3 --state-archive-capacity full
Predeployed FeeToken
ETH Address: 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7
STRK Address: 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7
Class Hash: 0x046ded64ae2dead6448e247234bab192a9c483644395b66f2155f2614e5804b0

Predeployed UDC
Address: 0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF
Class Hash: 0x7B3E05F48F0C69E4A65CE5E076A66271A527AFF2C34CE1083EC6E1526997A69

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

Predeployed accounts using class with hash: 0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f
Initial balance of each account: 1000000000000000000000 WEI and FRI
Seed to replicate this account sequence: 0
2024-07-24T16:44:22.432749Z  INFO starknet_devnet: Starknet Devnet listening on 127.0.0.1:5050
```

There are two Openzeppelin ERC20 contracts deployed on the devnet:

- ETH: `0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7`
- STRK: `0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7`

> [!TIP]\
> The `*.contract_class.json` file for these contracts can be fetched by
> `starkli class-at <CONTRACT_ADDRESS>`. The contract class contains the ABI
> information of the contract types and functions - which are necessary for
> value (de)serialization.

There is also a UDC or Universal Deployment Contract deployed on the devnet at
`0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF` which allows
deploying new contracts.

Also, there are three accounts created and deployed from Openzeppelin account
contract class. The account config of these accounts can be fetched from the
devnet.

```bash
starkli account fetch <ACCOUNT_ADDRESS> --output <ACCOUNT_CONFIG_FILE>
```

> [!TIP]\
> Set the environment variables `STARKNET_ACCOUNT=<ACCOUNT_CONFIG_FILE>` and
> `STARKNET_PRIVATE_KEY=<PRIVATE_KEY>` or use `--account` and `--private-key`
> flag in `starkli` to submit a transaction from an account.

## Manage ERC20 contracts

Everything on the Starknet is a contract. The accounts and tokens are deployed
as contracts as any other normal contracts. So you have to use the general
purpose contract APIs to manage a token.

I am sure there are specialized client applications for the Openzeppelin
contracts. But for now, we will use `starkli` and its subcommands: `call` and
`invoke`.

### Check balance

```bash
starkli call <ERC20_CONTRACT_ADDRESS> balance_of <ACCOUNT_ADDRESS>
```

Note that the result is two `u128` values - because the balance is stored in
`u256`. We need to deserialize it to get the actual balance.

### Transfer fund

```bash
starkli invoke --private-key <SENDER_PRIVATE_KEY> <ERC20_CONTRACT_ADDRESS> transfer <RECIPIENT_ADDRESS> <VALUE1> <VALUE2>
```

Note that the `VALUE1` and `VALUE2` are two `u128` in the serialized form of the
`u256` value of the amount to transfer.

> [!NOTE]\
> For more details, check the [serialization of Cairo types](#references) in the
> references.

Apart from `balance_of` and `transfer`, the ERC20 contract has other functions,
such as `mint`, `approve`, `allowance`, `total_supply` etc.

## Compile a custom contract

Considering the project structure, you can execute the following commands to
compile the contract:

```bash
cd contracts
scarb build
```

We will assume we are compiling the following Cairo contract:

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

This will compile the contract classes at:
`contracts/targets/dev/starknet_ibc_simple_storage.contract_class.json`

## Declare and deploy the contract

```bash
starkli declare --compiler-version 2.6.2 contracts/target/dev/starknet_ibc_simple_storage.contract_class.json
```

This will declare the contract class on the _Starknet_. Note down the
`CONTRACT_CLASS` from the output.

## Deploy, query, and update the contract

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

### Pass complex-typed inputs to the contract

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

#### Tuple

Tuple is serialized as it is.

```
(10, 20, 30): 10, 20, 30
```

#### Array

Array is serialized as a list of numerals where the first numeral is the length
and then the serialized elements.

```
[10, 20, 30]: 3, 10, 20, 30
```

## Generate Rust bindings for a contract

To simplify the (de)serialization process, we can generate Rust bindings for the
contract calls and types using
[`cainome`](https://github.com/cartridge-gg/cainome). It is very similar to
generating Rust code for protobuf definitions using `protoc` and `tonic`.

The following will generate a Rust code for the Openzeppelin ERC20 contract at
`gen/erc20.rs`.

```console
$ cargo install --features=build-binary --git https://github.com/cartridge-gg/cainome
$ mkdir codegen
$ cainome --rust --output-dir gen --contract-name erc20 --execution-version v3 --rpc-url http://127.0.0.1:5050 \
    --contract-address 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7
# or, from local contract class file
$ cainome --rust --output-dir gen --execution-version v3 --artifacts-path contracts/target/dev
```

You can also use `cainome::rs::abigen` macro to generate the bindings in
runtime.

```rust
use cainome::rs::abigen;

abigen!(
    StarknetIbcSimpleStorage,
    "<ABSOLUTE_PATH_OF_PROJECT>/target/dev/starknet_ibc_simple_storage.contract_class.json"
);
```

With this, you can write client code in Rust to interact with the contract.

```rust
use <IMPORT_PATH_TO_CODEGEN>::{StarknetIbcSimpleStorage, StarknetIbcSimpleStorageReader};
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::types::Felt,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use std::sync::Arc;
use url::Url;

const CONTRACT_ADDRESS: &str = "<CONTRACT_ADDRESS>";
const ACCOUNT_0: &str = "0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691";
const PRIVKEY_0: &str = "0x71d7bb07b9a64f6f78ac4c816aff4da9";
const CHAIN_ID: &str = "0x534e5f5345504f4c4941";

#[tokio::main]
async fn main() {
    let rpc_url = Url::parse("http://0.0.0.0:5050").expect("Expecting Starknet RPC URL");
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let contract_address = Felt::from_hex(CONTRACT_ADDRESS).unwrap();

    let contract = StarknetIbcSimpleStorageReader::new(contract_address, &provider);

    let stored_data: u128 = contract.get().call().await.expect("Call to `get` failed");
    println!("initial values: {:?}", stored_data);

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(PRIVKEY_0).unwrap(),
    ));
    let account = Arc::new(SingleOwnerAccount::new(
        provider,
        signer,
        Felt::from_hex(ACCOUNT_0).unwrap(),
        Felt::from_hex(CHAIN_ID).unwrap(),
        ExecutionEncoding::New,
    ));

    let contract = StarknetIbcSimpleStorage::new(contract_address, account);

    // increment the value
    let set_call = contract.set(&(stored_data + 1));

    let estimated_fee = set_call.estimate_fee().await.expect("Fail to estimate");
    println!("Estimated fee: {:?}", estimated_fee);

    let tx_res = set_call.send().await.expect("Call to `set` failed");
    println!("Transaction sent {:?}", tx_res);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let stored_data: u128 = contract.get().call().await.expect("Call to `get` failed");
    println!("values after invoke: {:?}", stored_data);
}
```

## References

- [Getting started with Cairo](https://www.cairo-lang.org/tutorials/getting-started-with-cairo)
- [Compile, deploy, interact with Cairo contracts](https://book.starknet.io/ch02-02-compile-deploy-interact.html)
- [Cairo book](https://book.cairo-lang.org)
- [Serialization of Cairo types](https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types)
- [Parsing Cairo ABI](https://www.starknetjs.com/docs/guides/automatic_cairo_abi_parsing)
