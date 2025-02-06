#[derive(Debug)]
pub struct MembershipVerifierContainer {
    pub state_root: Vec<u8>,
    pub prefix: Vec<u8>,
    pub path: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

impl MembershipVerifierContainer {
    pub fn canonical_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let encode_bytes = |mut data: Vec<u8>, output: &mut Vec<u8>| {
            let len = data.len() as u64;
            output.extend(len.to_be_bytes());
            output.append(&mut data);
        };
        encode_bytes(self.state_root, &mut bytes);
        encode_bytes(self.prefix, &mut bytes);
        encode_bytes(self.path, &mut bytes);
        if let Some(v) = self.value {
            encode_bytes(v, &mut bytes);
        }
        bytes
    }
}
