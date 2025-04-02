use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use ibc::clients::tendermint::types::{
    ConsensusState as TendermintConsensusState, Header as TendermintHeader,
};
use ibc::core::primitives::Timestamp;

use crate::impls::utils::array::from_vec_u8_to_be_u32_slice;
use crate::types::cosmos::height::Height;

#[derive(Debug, Clone, HasField)]
pub struct CometUpdateHeader {
    pub trusted_height: Height,
    pub target_height: Height,
    pub time: Timestamp,
    pub root: [u32; 8],
    pub next_validators_hash: Vec<u8>,
}

pub struct EncodeCometUpdateHeader;

delegate_components! {
    EncodeCometUpdateHeader {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("trusted_height"), UseContext>,
                EncodeField<symbol!("target_height"), UseContext>,
                EncodeField<symbol!("time"), UseContext>,
                EncodeField<symbol!("root"), UseContext>,
                EncodeField<symbol!("next_validators_hash"), UseContext>,
            ],
        >,
    }
}

impl From<TendermintHeader> for CometUpdateHeader {
    fn from(header: TendermintHeader) -> Self {
        let trusted_height = Height {
            revision_number: header.trusted_height.revision_number(),
            revision_height: header.trusted_height.revision_height(),
        };

        let target_height = {
            let header_height = header.height();

            Height {
                revision_number: header_height.revision_number(),
                revision_height: header_height.revision_height(),
            }
        };

        let time = header.timestamp().expect("header timestamp is missing");

        let tm_cons_state = TendermintConsensusState::from(header);

        let root = tm_cons_state.root.into_vec();
        let next_validators_hash = tm_cons_state.next_validators_hash.into();

        let root_slice = from_vec_u8_to_be_u32_slice(root).expect("invalid root length");

        Self {
            trusted_height,
            target_height,
            time,
            root: root_slice,
            next_validators_hash,
        }
    }
}
