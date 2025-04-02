#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_starknet_chain_components::components::chain::StarknetChainComponents;
    use StarknetChainComponents::re_exports::*;

    use crate::impls;
    use crate::types::{StarknetMessage, TxResponse};

    StarknetChainComponents::with_components! {
        [
            MessageTypeProviderComponent,
            TxResponseTypeProviderComponent,
            ContractCallerComponent,
            ContractDeclarerComponent,
            ContractDeployerComponent,
            ContractInvokerComponent,
            InvokeContractMessageBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            BlockEventsQuerierComponent,
            BlockQuerierComponent,
            ChainStatusQuerierComponent,
            TxResponseQuerierComponent,
        ],
        | Components | {
            cgp_preset! {
                MadaraChainPreset {
                    Components:
                        StarknetChainComponents::Provider,
                    MessageTypeProviderComponent:
                        UseType<StarknetMessage>,
                    TxResponseTypeProviderComponent:
                        UseType<TxResponse>,
                    ContractCallerComponent:
                        impls::CallStarknetContract,
                    ContractDeclarerComponent:
                        impls::DeclareSierraContract,
                    ContractDeployerComponent:
                        impls::DeployStarknetContract,
                    ContractInvokerComponent:
                        impls::InvokeStarknetContract,
                    InvokeContractMessageBuilderComponent:
                        impls::BuildInvokeContractCall,
                    UpdateClientPayloadBuilderComponent:
                        impls::BuildStarknetUpdateClientPayload,
                    BlockEventsQuerierComponent:
                        impls::GetStarknetBlockEvents,
                    BlockQuerierComponent:
                        impls::QueryStarknetBlock,
                    ChainStatusQuerierComponent:
                        impls::QueryStarknetChainStatus,
                    TxResponseQuerierComponent:
                        impls::QueryTransactionReceipt,
                }
            }
        }
    }
}
