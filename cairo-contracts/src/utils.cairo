pub trait ComputeKeyTrait<T> {
    fn compute_key(self: @T) -> felt252;
}

pub trait ValidateBasicTrait<T> {
    fn validate_basic(self: @T);
}
