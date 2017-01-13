pub mod stpcpy;
pub mod strcpy;
pub mod strlen;
pub mod strcmp;
pub mod strspn;

#[cfg_attr(rustfmt, rustfmt_skip)]
const ONES:  usize = 0x0101010101010101_usize;
const HIGHS: usize = 0x8080808080808080_usize;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[inline(always)]
fn has_zero(x: usize) -> bool {
    (x.wrapping_sub(ONES) & !x & HIGHS) != 0
}
