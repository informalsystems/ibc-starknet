#!/usr/bin/env -S deno run

import "jsr:@std/dotenv/load";
import { Command } from "jsr:@cliffy/command@^1.0.0-rc.7";
import {
  byteArray,
  CairoCustomEnum,
  CallData,
  Contract,
  hash,
  num,
  RpcProvider,
} from "npm:starknet";

const {
  options: { starknetRpc, ics20ContractAddress },
  args: [portId, channelId, baseDenom],
} = await new Command()
  .env("STARKNET_RPC=<value:string>", "Starknet RPC endpoint", {
    required: true,
  })
  .env("ICS20_CONTRACT_ADDRESS=<value:string>", "ICS20 contract address", {
    required: true,
  })
  .arguments("<portId:string> <channelId:string> <baseDenom:string>")
  .parse(Deno.args);

console.log("Starknet RPC Endpoint:", starknetRpc);
console.log("ICS20 Contract Address:", ics20ContractAddress);

const starknetProvider = new RpcProvider({
  nodeUrl: starknetRpc,
});

const ics20Class = await starknetProvider.getClassAt(ics20ContractAddress);
const ics20Contract = new Contract(
  ics20Class.abi,
  ics20ContractAddress,
  starknetProvider,
);

const ibc_prefixed_denom = {
  trace_path: [{
    port_id: {
      port_id: byteArray.byteArrayFromString(portId),
    },
    channel_id: {
      channel_id: byteArray.byteArrayFromString(channelId),
    },
  }],
  base: new CairoCustomEnum({
    Native: undefined,
    Hosted: byteArray.byteArrayFromString(baseDenom),
  }),
};

{
  const serialized_ibc_prefixed_denom = CallData.compile(ibc_prefixed_denom);

  const callDataAbi = new CallData(
    ics20Contract.abi,
  );

  const deserializedPrefixedDenom = callDataAbi.decodeParameters(
    "starknet_ibc_apps::transfer::types::PrefixedDenom",
    serialized_ibc_prefixed_denom,
  );

  console.log(deserializedPrefixedDenom);
}

const serialized_ibc_prefixed_denom = [];

// transfer/channel-0/transfer/channel-23/coin
// compile(transfer/channel-0):compile(transfer/channel-23):compile(coin)
for (const trace_prefix of ibc_prefixed_denom.trace_path.slice().reverse()) {
  serialized_ibc_prefixed_denom.push(...CallData.compile(trace_prefix));
}

serialized_ibc_prefixed_denom.push(
  ...CallData.compile({ base: ibc_prefixed_denom.base }),
);

const ibc_prefixed_denom_key = hash.computePoseidonHashOnElements(
  serialized_ibc_prefixed_denom,
);

const ibc_token_address = num.toHex(
  await ics20Contract.ibc_token_address(
    ibc_prefixed_denom_key,
  ),
);

console.log("ERC20 token address:", ibc_token_address);

const ics20TokenClass = await starknetProvider.getClassAt(ibc_token_address);
const ics20TokenContract = new Contract(
  ics20TokenClass.abi,
  ibc_token_address,
  starknetProvider,
);

console.log("  name:", await ics20TokenContract.name());
console.log("  symbol:", await ics20TokenContract.symbol());
console.log("  decimals:", await ics20TokenContract.decimals());
console.log("  total_supply:", await ics20TokenContract.total_supply());
