<div align="center">
    <h1>Starknet-IBC Cairo Contracts</h1>
</div>

This directory contains the Cairo contracts that implement the IBC protocol for
Starknet. Currently, the ICS-20 Token Transfer application is implemented, which
operates alongside a custom ERC20 contract. This custom contract offers a
flexible supply through permissioned minting and burning capabilities, unlike
the traditional ERC20 contract with a fixed supply.

You can find the preset implementations of these contracts in the
`cairo-contracts/presets` directory. The directory includes:

- `erc20.cairo` - The ERC20 mintable contract implementation.
- `transfer.cairo` - The ICS-20 transfer contract implementation.

## How to build

To build the contracts, you simply need to run the following command:

```bash
cd cairo-contracts
scarb build
```

The command will compile the contracts and output the compiled contracts as JSON
files in the `cairo-contracts/target/dev` directory, which you can then deploy
to Starknet.

- ERC20 Mintable contract: `starknet_ibc_ERC20.contract_class.json`
- Token Transfer contract: `starknet_ibc_transfer.contract_class.json`

## How to deploy

### Create Starknet account

To deploy the contracts, you first need to setup a Starknet account locally. You
can find the details in the [Starknet
documentation](https://docs.starknet.io/quick-start/set-up-an-account/), but
briefly you have to create a signer and an account descriptor. The signer is a
smart contract with a private key for signing transactions, which can be created
as follows:

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
ERC20_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_ERC20Mintable.contract_class.json}
ICS20_CONTRACT_SRC=${CONTRACT_SRC:-$(pwd)/cairo-contracts/target/dev/starknet_ibc_Transfer.contract_class.json}
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

## How to invoke

To interact with the deployed contracts, you can run the following command:

```bash
./scripts/invoke.sh
```

Currently, the script only invokes the `ibc_token_transfer` interface to confirm
that the contract is working as expected. To interact with other interfaces,
which require passing more complex and lengthy arguments, it is recommended to
use the client-side Rust or Typescript implementation.
