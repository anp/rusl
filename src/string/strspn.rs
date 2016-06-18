use core::usize;

use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn strspn(string: *const c_schar, chars: *const c_schar) -> size_t {
    if *chars.offset(0) == 0 {
        return 0;
    }
    if *chars.offset(1) == 0 {
        let mut i = 0;
        loop {
            if *string.offset(i) != *chars {
                return i as usize;
            }
            i += 1;
        }
    }

    let mut bit_array = AsciiBitArray::new();
    let mut i = 0;
    loop {
        let c = *chars.offset(i);
        if c == 0 {
            break;
        }
        bit_array.set_bit(c as usize);
        i += 1;
    }

    let mut j = 0;
    loop {
        let s = *string.offset(j);
        if s == 0 || !bit_array.get_bit(s as usize) {
            break;
        }
        j += 1;
    }

    return j as usize;
}

const ASCII_MAX_VALUE: usize = 127;
const U32_N_BITS: usize = 8 * 4;
const BIT_ARRAY_LEN: usize = 1 + ASCII_MAX_VALUE / U32_N_BITS;

struct AsciiBitArray {
    bit_array: [u32; BIT_ARRAY_LEN],
}

impl AsciiBitArray {
    fn new() -> AsciiBitArray { AsciiBitArray { bit_array: [0; BIT_ARRAY_LEN] } }

    fn set_bit(&mut self, index: usize) {
        let value = (1u32 << (index % U32_N_BITS)) as u32;
        let byte_index = index % BIT_ARRAY_LEN / U32_N_BITS;

        self.bit_array[byte_index] |= value;
    }

    fn get_bit(&self, index: usize) -> bool {
        let value = (1u32 << (index % U32_N_BITS)) as u32;
        let byte_index = index % BIT_ARRAY_LEN / U32_N_BITS;

        self.bit_array[byte_index] & value > 0
    }
}
