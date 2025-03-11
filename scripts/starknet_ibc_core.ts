#!/usr/bin/env -S deno run

import "jsr:@std/dotenv/load";
import { Contract, RpcProvider } from "npm:starknet";
import { QueryClient, setupIbcExtension } from "npm:@cosmjs/stargate";
import { Tendermint34Client } from "npm:@cosmjs/tendermint-rpc";

const OSMOSIS_RPC_ENDPOINT = Deno.env.get("OSMOSIS_RPC")!;
const STARKNET_RPC_ENDPOINT = Deno.env.get("STARKNET_RPC")!;
const IBC_CORE_CONTRACT = Deno.env.get("CORE_CONTRACT_ADDRESS")!;
const IBC_CLIENT_CONTRACT = Deno.env.get("COMET_CONTRACT_ADDRESS")!;
const CHANNEL_ID = "channel-2";
const PORT_ID = "transfer";

console.log("Osmosis RPC Endpoint:", OSMOSIS_RPC_ENDPOINT);
console.log("Starknet RPC Endpoint:", STARKNET_RPC_ENDPOINT);
console.log("Starknet IBC Core Contract:", IBC_CORE_CONTRACT);
console.log("Starknet IBC Client Contract:", IBC_CLIENT_CONTRACT);

const starknetProvider = new RpcProvider({ nodeUrl: STARKNET_RPC_ENDPOINT });

async function queryStarknetIbc() {
  console.log("Querying Starknet IBC data...");

  const ibcCoreClass = await starknetProvider.getClassAt(IBC_CORE_CONTRACT);
  const ibcClientClass = await starknetProvider.getClassAt(
    IBC_CLIENT_CONTRACT,
  );

  const ibcCoreContract = new Contract(
    ibcCoreClass.abi,
    IBC_CORE_CONTRACT,
    starknetProvider,
  );
  const ibcClientContract = new Contract(
    ibcClientClass.abi,
    IBC_CLIENT_CONTRACT,
    starknetProvider,
  );

  const channelEnd = await ibcCoreContract.channel_end({
    port_id: PORT_ID,
  }, {
    channel_id: CHANNEL_ID,
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
    portId: PORT_ID,
    channelId: CHANNEL_ID,
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

  const tmClient = await Tendermint34Client.connect(OSMOSIS_RPC_ENDPOINT);
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
