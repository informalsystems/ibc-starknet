<div align="center">
    <h1>Starknet-IBC Cairo Contracts</h1>
</div>

This directory contains the implementation of Cairo contracts designed to
integrate the IBC protocol with Starknet. In the current architecture, the
transport, authentication, and ordering (TAO) layer of IBC is implemented as a
standalone Cairo contract, referred to as the IBC core contract. This core
contract serves as the central handler for any registered IBC light clients and
applications. Therefore building on the IBC core contract, two groups of
contracts can live:

1. **IBC Light Clients**: These contracts enable the verification of a target
   (counterparty) consensus protocol on Starknet. Currently, this repository
   includes the implementation of the CometBFT (AKA Tendermint) light client.

2. **IBC Applications**: These contracts support for various IBC business logic.
   At present, this project includes an implementation of the ICS-20 token
   transfer application, alongside a custom ERC20 contract. This custom contract
   features permissioned minting and burning capabilities, enabling a flexible
   token supply, unlike the traditional ERC20 contract, which typically enforces
   a fixed supply.

The implementations of these contracts live in the `cairo-contracts/contracts`
directory that includes:

- `core.cairo` - The IBC core contract.
- `erc20.cairo` - The ERC20 mintable contract.
- `apps/transfer.cairo` - The ICS-20 token transfer application.
- `clients/cometbft.cairo` - The CometBFT light client.

## How to build

Install `scarb v2.11.3` with the instruction provided in the
[Scarb Documentation](https://docs.swmansion.com/scarb/download.html). Then, to
build the contracts, you simply need to run the following command:

```bash
cd cairo-contracts
scarb build -p starknet_ibc_contracts
```

The command will compile the contracts and output the compiled contracts as JSON
files in the `cairo-contracts/target/dev` directory, which you can then deploy
to Starknet.

- IBC Core contract: `starknet_ibc_contracts_IBCCore.contract_class.json`
- CometBFT client contract:
  `starknet_ibc_contracts_CometClient.contract_class.json`
- ERC20 Mintable contract:
  `starknet_ibc_contracts_ERC20Mintable.contract_class.json`
- Token Transfer contract:
  `starknet_ibc_contracts_TransferApp.contract_class.json`

## How to deploy

### Create Starknet account

To deploy the contracts, you first need to setup a Starknet account locally. You
can find the details in the
[Starknet documentation](https://docs.starknet.io/quick-start/set-up-an-account/),
but briefly you have to create a signer and an account descriptor. The signer is
a smart contract with a private key for signing transactions, which can be
created as follows:

```bash
# Create the default directory
mkdir -p ~/.starkli-wallets/deployer

# Generate the keystore file from a private key
starkli signer keystore from-key ~/.starkli-wallets/deployer/keystore.json

# Paste the private key of your smart wallet.
# You can obtain the private key from your smart wallet (e.g. Braavos or ArgentX)
Enter private key:

# Enter a password of your choice.
Enter password:

# To view the details of the created keystore file.
cat ~/.starkli-wallets/deployer/keystore.json
```

The account descriptor is a JSON file that contains the info of the signer,
which can be created as follows:

```bash
# Generates the account descriptor file
starkli account fetch <SMART_WALLET_ADDRESS> --output ~/.starkli-wallets/deployer/account.json

# To see the details of your Account Descriptor file.
cat ~/.starkli-wallets/deployer/account.json
```

### Setup environment

Next, create an `.env` file in the root directory of the project. You can use
the `.env.example` file as a template, which contains the following content:

```bash
CORE_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_contracts_IBCCore.contract_class.json}
COMET_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_contracts_CometClient.contract_class.json}
ERC20_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_contracts_ERC20Mintable.contract_class.json}
ICS20_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_contracts_TransferApp.contract_class.json}
RPC_URL=https://starknet-sepolia.public.blastapi.io/rpc/v0_7
ACCOUNT_SRC="${HOME}/.starkli-wallets/deployer/account.json"
KEYSTORE_SRC="${HOME}/.starkli-wallets/deployer/keystore.json"
KEYSTORE_PASS=<KEYSTORE_PASSWORD>
ERC20_CLASS_HASH=""
ICS20_CLASS_HASH=""
CONTRACT_ADDRESS=""
```

Make sure to replace `<KEYSTORE_PASSWORD>` with the password of your Starknet
account.

If you have previously declared the contracts on-chain, you can fill in the
`ERC20_CLASS_HASH` and `ICS20_CLASS_HASH` fields to use that particular versions
for deploying the contracts. Otherwise, the script will declare the contracts as
well.

### Deploy contracts

Now that you have set up the environment, you can deploy the contracts by
running the following command:

```bash
./scripts/deploy.sh
```

The script will deploy a single instance of the ICS20 contract to the Starknet
network and return the contract address.

> [!NOTE]
> The deployment will, by default, occur on the Starknet Sepolia testnet. To
> ensure compatibility, verify that your current Scarb version is supported by
> the Starknet testnet by reviewing the
> [Starknet release notes.](https://docs.starknet.io/starknet-versions/version-notes/#starknet_environments)

## How to invoke

To interact with the deployed contracts, you can run the following command:

```bash
./scripts/invoke.sh
```

Currently, the script only invokes the `ibc_token_address` interface to confirm
that the contract responses as expected. To interact with other interfaces,
which require passing lengthy encoded arguments, it is recommended to use a
client-side Rust or Typescript implementation.
