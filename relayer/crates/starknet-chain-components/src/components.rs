use cgp_core::prelude::*;
pub use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;

use crate::impls::contract::call::CallStarknetContract;
use crate::impls::contract::invoke::InvokeStarknetContract;
use crate::impls::types::address::ProvideFeltAddressType;
use crate::impls::types::blob::ProvideFeltBlobType;
use crate::impls::types::method::ProvideFeltMethodSelector;
use crate::impls::types::tx_hash::ProvideFeltTxHash;
pub use crate::traits::contract::call::ContractCallerComponent;
pub use crate::traits::contract::invoke::ContractInvokerComponent;
pub use crate::traits::types::address::AddressTypeComponent;
pub use crate::traits::types::blob::BlobTypeComponent;
pub use crate::traits::types::method::MethodSelectorTypeComponent;

define_components! {
    StarknetChainComponents {
        AddressTypeComponent:
            ProvideFeltAddressType,
        BlobTypeComponent:
            ProvideFeltBlobType,
        TransactionHashTypeComponent:
            ProvideFeltTxHash,
        MethodSelectorTypeComponent:
            ProvideFeltMethodSelector,
        ContractCallerComponent:
            CallStarknetContract,
        ContractInvokerComponent:
            InvokeStarknetContract,
    }
}
