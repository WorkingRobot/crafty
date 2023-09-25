pub fn nth_bit_set_64(value: u64, n: u32) -> u32 {
    if n >= value.count_ones() {
        return 64;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("bmi2") {
            nth_bit_set_64_bmi2(value, n)
        } else {
            nth_bit_set_64_scalar(value, n)
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        nth_bit_set_64_scalar(value, n)
    }
}

pub fn nth_bit_set_32(value: u32, n: u32) -> u32 {
    if n >= value.count_ones() {
        return 32;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("bmi2") {
            nth_bit_set_32_bmi2(value, n)
        } else {
            nth_bit_set_32_scalar(value, n)
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        nth_bit_set_32_scalar(value, n)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "bmi2")]
unsafe fn nth_bit_set_64_bmi2(value: u64, n: u32) -> u32 {
    use std::arch::x86_64::_pdep_u64;

    _pdep_u64(1u64 << n, value).trailing_zeros()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "bmi2")]
unsafe fn nth_bit_set_32_bmi2(value: u32, n: u32) -> u32 {
    use std::arch::x86_64::_pdep_u32;

    _pdep_u32(1u32 << n, value).trailing_zeros()
}

fn nth_bit_set_64_scalar(value: u64, mut n: u32) -> u32 {
    let mut mask = 0x00000000FFFFFFFFu64;
    let mut size = 32;
    let mut base = 0;

    if n >= value.count_ones() {
        return 64;
    }

    n += 1;
    while size > 0 {
        let count = (value & mask).count_ones();
        if n > count {
            base += size;
            size >>= 1;
            mask |= mask << size;
        } else {
            size >>= 1;
            mask >>= size;
        }
    }

    base
}

fn nth_bit_set_32_scalar(value: u32, mut n: u32) -> u32 {
    let mut mask = 0x0000FFFFu32;
    let mut size = 16;
    let mut base = 0;

    if n >= value.count_ones() {
        return 32;
    }

    n += 1;
    while size > 0 {
        let count = (value & mask).count_ones();
        if n > count {
            base += size;
            size >>= 1;
            mask |= mask << size;
        } else {
            size >>= 1;
            mask >>= size;
        }
    }

    base
}
