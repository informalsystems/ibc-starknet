use cgp::prelude::*;
use ibc::clients::tendermint::types::{
    ConsensusState as TendermintConsensusState, Header as TendermintHeader,
};
use ibc::core::primitives::Timestamp;

use crate::impls::utils::array::from_vec_u8_to_be_u32_slice;
use crate::types::cosmos::height::Height;

#[derive(Debug, Clone, HasField, HasFields)]
pub struct CometUpdateHeader {
    pub trusted_height: Height,
    pub target_height: Height,
    pub time: Timestamp,
    pub root: [u32; 8],
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

        let root = TendermintConsensusState::from(header).root.into_vec();

        let root_slice = from_vec_u8_to_be_u32_slice(root).expect("invalid root length");

        Self {
            trusted_height,
            target_height,
            time,
            root: root_slice,
        }
    }
}
