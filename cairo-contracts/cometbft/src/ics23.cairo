#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub enum HashOp {
    #[default]
    NoOp,
    Sha256,
    Sha512,
    Keccak256,
    Ripemd160,
    Bitcoin,
    Sha512_256,
    Blake2b_512,
    Blake2b_256,
    Blake3,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct InnerSpec {
    pub child_order: Array<i32>,
    pub child_size: i32,
    pub min_prefix_length: i32,
    pub max_prefix_length: i32,
    pub empty_child: ByteArray,
    pub hash: HashOp,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub enum LengthOp {
    #[default]
    NoPrefix,
    VarProto,
    VarRlp,
    Fixed32Big,
    Fixed32Little,
    Fixed64Big,
    Fixed64Little,
    Require32Bytes,
    Require64Bytes,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct LeafOp {
    pub hash: HashOp,
    pub prehash_key: HashOp,
    pub prehash_value: HashOp,
    pub length: LengthOp,
    pub prefix: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ProofSpec {
    pub leaf_spec: LeafOp,
    pub inner_spec: InnerSpec,
    pub max_depth: i32,
    pub min_depth: i32,
    pub prehash_key_before_comparison: bool,
}
