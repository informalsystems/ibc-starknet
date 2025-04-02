#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_starknet_chain_components::components::chain::StarknetChainComponents;
    use starknet_v13::core::types::contract::SierraClass;
    use StarknetChainComponents::re_exports::*;

    use crate::impls;
    use crate::types::{StarknetMessage, TxResponse};

    StarknetChainComponents::with_components! {
        [
            MessageTypeProviderComponent,
            TxResponseTypeProviderComponent,
            ContractClassTypeProviderComponent,
            ContractCallerComponent,
            ContractDeclarerComponent,
            ContractDeployerComponent,
            ContractInvokerComponent,
            InvokeContractMessageBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            BlockEventsQuerierComponent,
            BlockQuerierComponent,
            NonceQuerierComponent,
            ChainStatusQuerierComponent,
            MessagesWithSignerAndNonceSenderComponent,
            TxMessageResponseParserComponent,
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
                    ContractClassTypeProviderComponent:
                        UseType<SierraClass>,
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
                    NonceQuerierComponent:
                        impls::QueryStarknetNonce,
                    ChainStatusQuerierComponent:
                        impls::QueryStarknetChainStatus,
                    [
                        MessagesWithSignerAndNonceSenderComponent,
                        TxMessageResponseParserComponent,
                    ]:
                        impls::SendStarknetMessages,
                    TxResponseQuerierComponent:
                        impls::QueryTransactionReceipt,
                }
            }
        }
    }
}
