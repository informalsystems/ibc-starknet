use core::num::traits::OverflowingMul;

pub const POW_2_32: u64 = 4_294_967_296;

pub trait BitShift<T> {
    fn shl(x: T, n: T) -> T;
    fn shr(x: T, n: T) -> T;
}

impl U256BitShift of BitShift<u256> {
    fn shl(x: u256, n: u256) -> u256 {
        let (r, _) = OverflowingMul::overflowing_mul(x, pow(2, n));
        r
    }

    fn shr(x: u256, n: u256) -> u256 {
        x / pow(2, n)
    }
}

pub fn pow<T, +Copy<T>, +Drop<T>, +Mul<T>, +Div<T>, +Rem<T>, +PartialEq<T>, +Into<u8, T>>(
    base: T, exp: T
) -> T {
    let zero: T = 0_u8.into();
    let one: T = 1_u8.into();
    let two: T = 2_u8.into();

    if exp == zero {
        return one;
    }

    if exp == one {
        return base;
    }

    let recur = pow(base * base, exp / two);

    if exp % two == zero {
        recur
    } else {
        base * recur
    }
}

#[derive(Drop)]
pub struct U32Collector {
    pub value: Array<u32>,
}

#[generate_trait]
pub impl U32CollectorImpl of U32CollectorTrait {
    fn init() -> U32Collector {
        U32Collector { value: ArrayTrait::new() }
    }

    fn append(ref self: U32Collector, value: u32) {
        self.value.append(value);
    }

    fn extend<T, +IntoArrayU32<T>>(ref self: U32Collector, value: T) {
        let array = value.into_array_u32();
        self.value.append_span(array.span());
    }

    fn extend_from_chunk(ref self: U32Collector, slice: [u32; 8]) {
        self.value.append_span(slice.span());
    }

    fn value(self: U32Collector) -> Array<u32> {
        self.value
    }
}

pub trait IntoArrayU32<T> {
    fn into_array_u32(self: T) -> Array<u32>;
}

pub impl U64IntoArrayU32 of IntoArrayU32<u64> {
    fn into_array_u32(self: u64) -> Array<u32> {
        u64_into_array_u32(self)
    }
}

pub fn u64_into_array_u32(value: u64) -> Array<u32> {
    let mut array: Array<u32> = ArrayTrait::new();
    let upper = (value / POW_2_32).try_into().unwrap();
    let lower = (value % POW_2_32).try_into().unwrap();
    array.append(upper);
    array.append(lower);
    array
}

pub trait IntoDigest<T> {
    fn into_digest(self: T) -> [u32; 8];
}

pub impl U256IntoDigest of IntoDigest<u256> {
    fn into_digest(self: u256) -> [u32; 8] {
        [0, 0, 0, 0, 0, 0, 0, 0]
    }
}

pub trait IntoU256<T> {
    fn into_u256(self: T) -> u256;
}

pub impl Sha256DigestIntoU256 of IntoU256<[u32; 8]> {
    fn into_u256(self: [u32; 8]) -> u256 {
        array_u32_into_u256(self)
    }
}

pub fn array_u32_into_u256(u32_array: [u32; 8]) -> u256 {
    let mut value: u256 = 0;

    let [l0, l1, l2, l3, l4, l5, l6, l7] = u32_array;

    value = BitShift::shl(l0.into(), 224)
        | BitShift::shl(l1.into(), 192)
        | BitShift::shl(l2.into(), 160)
        | BitShift::shl(l3.into(), 128)
        | BitShift::shl(l4.into(), 96)
        | BitShift::shl(l5.into(), 64)
        | BitShift::shl(l6.into(), 32)
        | l7.into();

    value
}

pub fn array_u8_into_array_u32(array: Array<u8>) -> Array<u32> {
    array![0]
}
