use core::num::traits::Zero;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct BasePrefix {
    pub prefix: ByteArray,
}

pub impl BasePrefixZero of Zero<BasePrefix> {
    fn zero() -> BasePrefix {
        BasePrefix { prefix: "" }
    }

    fn is_zero(self: @BasePrefix) -> bool {
        self.prefix.len() == 0
    }

    fn is_non_zero(self: @BasePrefix) -> bool {
        !self.is_zero()
    }
}

pub impl BasePrefixIntoByteArray of Into<BasePrefix, ByteArray> {
    fn into(self: BasePrefix) -> ByteArray {
        self.prefix
    }
}

pub fn CLIENTS_PREFIX() -> ByteArray {
    "clients"
}

pub fn CONNECTIONS_PREFIX() -> ByteArray {
    "connections"
}

pub fn CHANNELS_PREFIX() -> ByteArray {
    "channels"
}

pub fn CHANNEL_ENDS_PREFIX() -> ByteArray {
    "channelEnds"
}

pub fn PORTS_PREFIX() -> ByteArray {
    "ports"
}

pub fn SEQUENCES_PREFIX() -> ByteArray {
    "sequences"
}

pub fn COMMITMENTS_PREFIX() -> ByteArray {
    "commitments"
}

pub fn RECEIPTS_PREFIX() -> ByteArray {
    "receipts"
}

pub fn ACKS_PREFIX() -> ByteArray {
    "acks"
}

pub fn NEXT_SEQ_SEND_PREFIX() -> ByteArray {
    "nextSequenceSend"
}

pub fn NEXT_SEQ_RECV_PREFIX() -> ByteArray {
    "nextSequenceRecv"
}

pub fn NEXT_SEQ_ACK_PREFIX() -> ByteArray {
    "nextSequenceAck"
}
