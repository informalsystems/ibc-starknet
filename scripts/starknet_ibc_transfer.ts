#!/usr/bin/env -S deno run

import "jsr:@std/dotenv/load";
import { Command } from "jsr:@cliffy/command@^1.0.0-rc.7";
import {
  Account,
  cairo,
  CairoCustomEnum,
  Contract,
  RpcProvider,
} from "npm:starknet";

const {
  options: {
    starknetRpc,
    ics20ContractAddress,
    privateKey,
  },
  args: [
    portId,
    channelId,
    accountAddress,
    amount,
    tokenAddress,
    osmosisAddress,
  ],
} = await new Command()
  .env("STARKNET_RPC=<value:string>", "Starknet RPC endpoint", {
    required: true,
  })
  .env(
    "ICS20_CONTRACT_ADDRESS=<value:string>",
    "ICS20 contract address",
    {
      required: true,
    },
  )
  .env(
    "PRIVATE_KEY=<value:string>",
    "Private key for the account",
    {
      required: true,
    },
  )
  .arguments(
    "<portId:string> <channelId:string> <accountAddress:string> <amount:number> <tokenAddress:string> <osmosisAddress:string>",
  )
  .parse(Deno.args);

console.log("Starknet RPC Endpoint:", starknetRpc);
console.log("Starknet IBC ICS20 Contract:", ics20ContractAddress);

const provider = new RpcProvider({ nodeUrl: starknetRpc });

const account = new Account(
  provider,
  accountAddress,
  privateKey,
);

const tokenClass = await provider.getClassAt(tokenAddress);

const tokenContract = new Contract(
  tokenClass.abi,
  tokenAddress,
  provider,
).typedv2(tokenClass.abi);

tokenContract.connect(account);

const approveCall = await tokenContract.approve(
  ics20ContractAddress,
  cairo.uint256(amount),
);

const approveReceipt = await account.waitForTransaction(
  approveCall.transaction_hash,
);

console.log(`Included in block ${approveReceipt.block_number}`);

const ics20Class = await provider.getClassAt(ics20ContractAddress);
const ics20Contract = new Contract(
  ics20Class.abi,
  ics20ContractAddress,
  provider,
).typedv2(ics20Class.abi);

ics20Contract.connect(account);

let prefixedDenom = {
  trace_path: [],
  base: new CairoCustomEnum({
    Native: { address: tokenAddress },
    Hosted: undefined,
  }),
};

const currentBlock = await provider.getBlock();

// timeout is 10 mins in future
const timestampTimeoutSecs = currentBlock.timestamp + (10 * 60);

const ics20TransferCall = await ics20Contract.send_transfer({
  port_id_on_a: { port_id: portId },
  chan_id_on_a: { channel_id: channelId },
  denom: prefixedDenom,
  amount: cairo.uint256(amount),
  receiver: osmosisAddress,
  memo: { memo: "sample memo" },
  // if timeout_height is non-empty, revision_number needs to be the cosmos one
  timeout_height_on_b: { revision_number: 0, revision_height: 0 },
  // timeout_timestamp is in nanoseconds
  timeout_timestamp_on_b: { timestamp: timestampTimeoutSecs * 1e9 },
});

const ics20TransferReceipt = await account.waitForTransaction(
  ics20TransferCall.transaction_hash,
);

console.log(`Included in block ${ics20TransferReceipt.block_number}`);
