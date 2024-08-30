#[derive(Drop, Serde)]
pub struct Packet {
    pub src_channel_id: felt252,
    pub src_port_id: felt252,
    pub dst_channel_id: felt252,
    pub dst_port_id: felt252,
    pub sequence: u64,
    pub packet_data: ByteArray,
}
