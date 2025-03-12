#!/usr/bin/env -S deno run

import "jsr:@std/dotenv/load";
import { Command } from "jsr:@cliffy/command@^1.0.0-rc.7";
import { Contract, RpcProvider } from "npm:starknet";
import { QueryClient, setupIbcExtension } from "npm:@cosmjs/stargate";
import { Tendermint34Client } from "npm:@cosmjs/tendermint-rpc";

const {
  options: {
    starknetRpc,
    osmosisRpc,
    coreContractAddress,
    cometContractAddress,
  },
  args: [portId, channelId],
} = await new Command()
  .env("STARKNET_RPC=<value:string>", "Starknet RPC endpoint", {
    required: true,
  })
  .env("OSMOSIS_RPC=<value:string>", "Osmosis RPC endpoint", {
    required: true,
  })
  .env(
    "CORE_CONTRACT_ADDRESS=<value:string>",
    "IBC Core contract address",
    {
      required: true,
    },
  )
  .env(
    "COMET_CONTRACT_ADDRESS=<value:string>",
    "IBC Client contract address",
    {
      required: true,
    },
  )
  .arguments("<portId:string> <channelId:string>")
  .parse(Deno.args);

console.log("Osmosis RPC Endpoint:", osmosisRpc);
console.log("Starknet RPC Endpoint:", starknetRpc);
console.log("Starknet IBC Core Contract:", coreContractAddress);
console.log("Starknet IBC Client Contract:", cometContractAddress);

const starknetProvider = new RpcProvider({ nodeUrl: starknetRpc });

async function queryStarknetIbc() {
  console.log("Querying Starknet IBC data...");

  const ibcCoreClass = await starknetProvider.getClassAt(coreContractAddress);
  const ibcClientClass = await starknetProvider.getClassAt(
    cometContractAddress,
  );

  const ibcCoreContract = new Contract(
    ibcCoreClass.abi,
    coreContractAddress,
    starknetProvider,
  );
  const ibcClientContract = new Contract(
    ibcClientClass.abi,
    cometContractAddress,
    starknetProvider,
  );

  const channelEnd = await ibcCoreContract.channel_end({
    port_id: portId,
  }, {
    channel_id: channelId,
  });

  const connectionId = channelEnd.connection_id;

  const connectionEnd = await ibcCoreContract.connection_end(
    connectionId,
  );

  const clientId = connectionEnd.client_id;
  const clientSequence = clientId.sequence;

  const clientType = await ibcClientContract.client_type();
  const clientLatestHeight = await ibcClientContract.latest_height(
    clientSequence,
  );
  const clientLatestTimestamp = await ibcClientContract.latest_timestamp(
    clientSequence,
  );
  const clientStatus = await ibcClientContract.status(clientSequence);
  const clientState = await ibcClientContract.client_state(
    clientSequence,
  );
  const consensusState = await ibcClientContract.consensus_state(
    clientSequence,
    clientLatestHeight,
  );
  const consensusStateRoot = await ibcClientContract
    .consensus_state_root(
      clientSequence,
      clientLatestHeight,
    );

  return {
    portId: portId,
    channelId: channelId,
    connectionId: connectionId,
    clientId: clientId,
    clientSequence: clientSequence,
    channelEnd: channelEnd,
    connectionEnd: connectionEnd,
    clientType: clientType,
    latestHeight: clientLatestHeight,
    latestTimestamp: clientLatestTimestamp,
    clientStatus: clientStatus,
    clientState: clientState,
    consensusState: consensusState,
    consensusStateRoot: consensusStateRoot,
  };
}

async function queryOsmosisIbc(
  portId: string,
  channelId: string,
) {
  console.log("Querying Osmosis IBC data...");

  const tmClient = await Tendermint34Client.connect(osmosisRpc);
  const queryClient = QueryClient.withExtensions(tmClient, setupIbcExtension);

  const channelEnd = await queryClient.ibc.channel.channel(
    portId,
    channelId,
  );

  const connectionId = channelEnd.channel!.connectionHops[0];

  const connectionEnd = await queryClient.ibc.connection.connection(
    connectionId,
  );

  const clientId = connectionEnd.connection!.clientId!;
  const clientState = await queryClient.ibc.client.state(
    clientId!,
  );

  return {
    portId: portId,
    channelId: channelId,
    connectionId: connectionId,
    clientId: clientId,
    channelEnd: channelEnd,
    connectionEnd: connectionEnd,
    clientState: clientState,
  };
}

const starknetData = await queryStarknetIbc();

const osmosisData = await queryOsmosisIbc(
  starknetData.channelEnd.remote.port_id.port_id,
  starknetData.channelEnd.remote.channel_id.channel_id,
);

console.log("\n=== Starknet IBC Data ===");

console.log("Channel:");
console.log(`  Port ID:`, starknetData.portId);
console.log(`  Channel ID:`, starknetData.channelId);
console.log("Channel End Details:");
console.log(starknetData.channelEnd);

console.log("\nConnection:");
console.log(`  Connection ID:`, starknetData.connectionId);
console.log("Connection End Details:");
console.log(starknetData.connectionEnd);

console.log("\nClient:");
console.log(`  Client ID:`, starknetData.clientId);
console.log(`  Client Sequence:`, starknetData.clientSequence);
console.log(`  Client Type:`, starknetData.clientType);
console.log(`  Latest Height:`, starknetData.latestHeight);
console.log(`  Latest Timestamp:`, starknetData.latestTimestamp);
console.log(`  Status:`, starknetData.clientStatus);
console.log("Client State Details:");
console.log(starknetData.clientState);

console.log("\nConsensus State:");
console.log(starknetData.consensusState);

console.log("\nConsensus State Root:");
console.log(starknetData.consensusStateRoot);

console.log("\n=== Osmosis IBC Data ===");

console.log("\nChannel:");
console.log(`  Port ID:`, osmosisData.portId);
console.log(`  Channel ID:`, osmosisData.channelId);
console.log("Channel End Details:");
console.log(osmosisData.channelEnd);

console.log("\nConnection:");
console.log(`  Connection ID:`, osmosisData.connectionId);
console.log("Connection End Details:");
console.log(osmosisData.connectionEnd);

console.log("Client:");
console.log(`  Client ID:`, osmosisData.clientId);
console.log("Client State Details:");
console.log(osmosisData.clientState);
