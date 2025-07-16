use hermes_cosmos_core::test_components::bootstrap::traits::{
    CosmosSdkConfigModifier, CosmosSdkConfigModifierComponent,
};
use hermes_prelude::*;
use toml::Value;

pub struct ModifyCosmosSdkConfigForOsmosis;

#[cgp_provider(CosmosSdkConfigModifierComponent)]
impl<Bootstrap> CosmosSdkConfigModifier<Bootstrap> for ModifyCosmosSdkConfigForOsmosis
where
    Bootstrap: HasAsyncErrorType,
{
    fn modify_cosmos_sdk_config(
        _bootstrap: &Bootstrap,
        cosmos_sdk_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        // Increase the gas limit for Osmosis mempool transactions to accommodate
        // the larger gas requirements when uploading the wasm light client
        cosmos_sdk_config
            .get_mut("osmosis-mempool")
            .unwrap()
            .as_table_mut()
            .unwrap()
            .insert("max-gas-wanted-per-tx".to_owned(), "90000000".into());

        Ok(())
    }
}
