use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_error::impls::ProvideHermesError;
use hermes_starknet_chain_components::components::*;
use hermes_starknet_chain_components::traits::client::JsonRpcClientGetter;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::types::address::HasAddressType;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasMethodSelectorType;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use url::Url;

use crate::impls::error::HandleStarknetError;

pub struct StarknetChain {
    pub rpc_client: JsonRpcClient<HttpTransport>,
}

pub struct StarknetChainContextComponents;

impl StarknetChain {
    pub fn new(json_rpc_url: Url) -> Self {
        let rpc_client = JsonRpcClient::new(HttpTransport::new(json_rpc_url));

        Self { rpc_client }
    }
}

impl HasComponents for StarknetChain {
    type Components = StarknetChainContextComponents;
}

delegate_components! {
    StarknetChainContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
    }
}

with_starknet_chain_components! {
    delegate_components! {
        StarknetChainContextComponents {
            @StarknetChainComponents: StarknetChainComponents,
        }
    }
}

impl JsonRpcClientGetter<StarknetChain> for StarknetChainContextComponents {
    fn json_rpc_client(chain: &StarknetChain) -> &JsonRpcClient<HttpTransport> {
        &chain.rpc_client
    }
}

pub trait CanUseStarknetChain:
    HasAddressType<Address = Felt>
    + HasMethodSelectorType<MethodSelector = Felt>
    + HasBlobType<Blob = Vec<Felt>>
    + CanCallContract
{
}

impl CanUseStarknetChain for StarknetChain {}
