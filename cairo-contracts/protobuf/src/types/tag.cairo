#[derive(Copy, Debug, Clone, Drop, PartialEq)]
pub enum WireType {
    Varint,
    Fixed64,
    LengthDelimited,
    StartGroup,
    EndGroup,
    Fixed32,
}

impl WireTypeToU8 of Into<WireType, u8> {
    fn into(self: WireType) -> u8 {
        match self {
            WireType::Varint => 0,
            WireType::Fixed64 => 1,
            WireType::LengthDelimited => 2,
            WireType::StartGroup => 3,
            WireType::EndGroup => 4,
            WireType::Fixed32 => 5,
        }
    }
}

impl U8ToWireType of Into<u8, WireType> {
    fn into(self: u8) -> WireType {
        match self {
            0 => WireType::Varint,
            1 => WireType::Fixed64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Fixed32,
            _ => panic!("Unsupported wire type"),
        }
    }
}


#[derive(Drop, PartialEq)]
pub struct ProtobufTag {
    pub field_number: u8,
    pub wire_type: WireType,
}

#[generate_trait]
pub impl ProtobufTagImpl of ProtobufTagTrait {
    fn decode(tag: u8) -> ProtobufTag {
        let wire_type: WireType = (tag & 0x07).into();

        let field_number: u8 = tag / 0x08;

        ProtobufTag { field_number, wire_type, }
    }

    fn encode(self: ProtobufTag) -> u8 {
        self.field_number * 0x08 | self.wire_type.into()
    }
}
