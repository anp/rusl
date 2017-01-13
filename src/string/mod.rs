pub mod stpcpy;
pub mod strcpy;
pub mod strlen;
pub mod strcmp;
pub mod strspn;

const ONES: usize  = 0x0101010101010101_usize;
const HIGHS: usize = 0x8080808080808080_usize;

#[inline(always)]
fn has_zero(x: usize) -> bool {
    (x.wrapping_sub(ONES) & !x & HIGHS) != 0
}
