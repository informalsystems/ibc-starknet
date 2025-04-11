use cgp::prelude::*;

#[derive(Debug, HasField, HasFields)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: [u32; 8],
    pub next_validators_hash: Vec<u8>,
}

pub struct EncodeCometConsensusState;

delegate_components! {
    EncodeCometConsensusState {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("timestamp"), UseContext>,
                EncodeField<symbol!("root"), UseContext>,
                EncodeField<symbol!("next_validators_hash"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometConsensusState {
    type From = Product![u64, [u32; 8], Vec<u8>];
    type To = CometConsensusState;

    fn transform(
        product![timestamp, root, next_validators_hash]: Self::From,
    ) -> CometConsensusState {
        CometConsensusState {
            timestamp,
            root,
            next_validators_hash,
        }
    }
}
