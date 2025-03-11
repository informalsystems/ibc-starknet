#!/usr/bin/env -S deno run

import {
  byteArray,
  CairoCustomEnum,
  CallData,
  Contract,
  hash,
  num,
  RpcProvider,
} from "npm:starknet";

const STARKNET_RPC_ENDPOINT =
  "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";
const ICS20_CONTRACT =
  "0x01bc3df6b90ea052fa965fceb983277f8bf1bc7d3484f80c67c219915c72c92e";

const CHANNEL_ID = "channel-2";
const PORT_ID = "transfer";
const OSMO_DENOM = "uosmo";

const starknetProvider = new RpcProvider({ nodeUrl: STARKNET_RPC_ENDPOINT });

const ics20Class = await starknetProvider.getClassAt(ICS20_CONTRACT);
const ics20Contract = new Contract(
  ics20Class.abi,
  ICS20_CONTRACT,
  starknetProvider,
);

const ibc_prefixed_denom = {
  trace_path: [{
    port_id: {
      port_id: byteArray.byteArrayFromString(PORT_ID),
    },
    channel_id: {
      channel_id: byteArray.byteArrayFromString(CHANNEL_ID),
    },
  }],
  base: new CairoCustomEnum({
    Native: undefined,
    Hosted: byteArray.byteArrayFromString(OSMO_DENOM),
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
