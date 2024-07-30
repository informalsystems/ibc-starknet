use cgp_core::prelude::*;

use crate::impls::contract::call::CallStarknetContract;
use crate::impls::types::address::ProvideFeltAddressType;
use crate::impls::types::blob::ProvideFeltBlobType;
use crate::impls::types::method::ProvideFeltMethodSelector;
pub use crate::traits::contract::call::ContractCallerComponent;
pub use crate::traits::types::address::AddressTypeComponent;
pub use crate::traits::types::blob::BlobTypeComponent;
pub use crate::traits::types::method::MethodSelectorTypeComponent;

define_components! {
    StarknetChainComponents {
        AddressTypeComponent:
            ProvideFeltAddressType,
        BlobTypeComponent:
            ProvideFeltBlobType,
        MethodSelectorTypeComponent:
            ProvideFeltMethodSelector,
        ContractCallerComponent:
            CallStarknetContract,
    }
}
