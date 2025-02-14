use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;
use crate::types::messages::ibc::channel::PortId;

#[derive(HasField)]
pub struct MsgRegisterClient {
    pub client_type: Felt,
    pub contract_address: StarknetAddress,
}

pub struct EncodeRegisterClient;

delegate_components! {
    EncodeRegisterClient {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("client_type"), UseContext>,
                EncodeField<symbol!("contract_address"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeRegisterClient {
    type From = Product![Felt, StarknetAddress];
    type To = MsgRegisterClient;

    fn transform(product![client_type, contract_address]: Self::From) -> MsgRegisterClient {
        MsgRegisterClient {
            client_type,
            contract_address,
        }
    }
}

#[derive(HasField)]
pub struct MsgRegisterApp {
    pub port_id: PortId,
    pub contract_address: StarknetAddress,
}

pub struct EncodeRegisterApp;

delegate_components! {
    EncodeRegisterApp {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("port_id"), UseContext>,
                EncodeField<symbol!("contract_address"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeRegisterApp {
    type From = Product![PortId, StarknetAddress];
    type To = MsgRegisterApp;

    fn transform(product![port_id, contract_address]: Self::From) -> MsgRegisterApp {
        MsgRegisterApp {
            port_id,
            contract_address,
        }
    }
}
