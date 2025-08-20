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
  },
  args: [portId, channelId, amount, tokenAddress, osmosisAddress],
} = await new Command()
  .env("STARKNET_RPC=<value:string>", "Starknet RPC endpoint", {
    required: true,
  })
  .env(
    "ICS20_CONTRACT_ADDRESS=<value:string>",
    "IC20 contract address",
    {
      required: true,
    },
  )
  .arguments(
    "<portId:string> <channelId:string> <amount:number> <tokenAddress:string> <osmosisAddress:string>",
  )
  .parse(Deno.args);

console.log("Starknet RPC Endpoint:", starknetRpc);
console.log("Starknet IBC ICS20 Contract:", ics20ContractAddress);

const provider = new RpcProvider({ nodeUrl: starknetRpc });

const accountAddress =
  "0x8a1719e7ca19f3d91e8ef50a48fc456575f645497a1d55f30e3781f786afe4";
const privateKey =
  "0x514977443078cf1e0c36bc88b89ada9a46061a5cf728f40274caea21d76f174";

let account = new Account(provider, accountAddress, privateKey);

const tokenClass = await provider.getClassAt(tokenAddress);

const tokenContract = new Contract(
  tokenClass.abi,
  tokenAddress,
  provider,
).typedv2(tokenClass.abi);

console.log(tokenContract.abi);

tokenContract.connect(account);

const approveCall = await tokenContract.approve(
  ics20ContractAddress,
  cairo.uint256(amount),
);

console.log(approveCall);

const ics20Class = await provider.getClassAt(ics20ContractAddress);
const ics20Contract = new Contract(
  ics20Class.abi,
  ics20ContractAddress,
  provider,
).typedv2(ics20Class.abi);

account = new Account(provider, accountAddress, privateKey);

ics20Contract.connect(account);

let prefixedDenom = {
  trace_path: [],
  base: new CairoCustomEnum({
    Native: { address: tokenAddress },
    Hosted: undefined,
  }),
};

const ics20TransferCall = await ics20Contract.send_transfer({
  port_id_on_a: { port_id: portId },
  chan_id_on_a: { channel_id: channelId },
  denom: prefixedDenom,
  amount: cairo.uint256(amount),
  receiver: osmosisAddress,
  memo: { memo: "sample memo" },
  // revision_number needs to be the cosmos one
  timeout_height_on_b: { revision_number: 2273009016, revision_height: 5000 },
  timeout_timestamp_on_b: { timestamp: 0 },
});

console.log(ics20TransferCall);
