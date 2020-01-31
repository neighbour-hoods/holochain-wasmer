pub const U16_MAX: u32 = u16::max_value() as u32;
pub const U32_MAX: u64 = u32::max_value() as u64;

/// returns the u16 high bits from a u32
pub fn u32_high_bits(i: u32) -> u16 {
    (i >> 16) as u16
}

/// returns the u16 low bits from a u32 by doing a lossy cast
pub fn u32_low_bits(i: u32) -> u16 {
    (i as u16)
}

/// splits the high and low bits of u32 into a tuple of u16, for destructuring convenience
pub fn u32_split_bits(i: u32) -> (u16, u16) {
    (u32_high_bits(i), u32_low_bits(i))
}

/// merges 2x u16 into a single u32
pub fn u32_merge_bits(high: u16, low: u16) -> u32 {
    (u32::from(high) << 16) | u32::from(low)
}

pub fn u64_high_bits(i: u64) -> u32 {
    (i >> 32) as u32
}

pub fn u64_low_bits(i: u64) -> u32 {
    (i as u32)
}

pub fn u64_split_bits(i: u64) -> (u32, u32) {
    (u64_high_bits(i), u64_low_bits(i))
}

pub fn u64_merge_bits(high: u32, low: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low)
}