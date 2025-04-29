#[cgp::re_export_imports]
mod preset {
    use hermes_prelude::*;
    use hermes_starknet_chain_components::components::StarknetChainComponents;
    use starknet_v13::core::types::contract::SierraClass;
    use StarknetChainComponents::re_exports::*;

    use crate::impls;
    use crate::types::TxResponse;

    StarknetChainComponents::with_components! {
        [
            TxResponseTypeProviderComponent,
            ContractClassTypeProviderComponent,
            ContractCallerComponent,
            ContractDeclarerComponent,
            ContractDeployerComponent,
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
