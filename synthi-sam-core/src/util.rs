#[inline(always)]
pub fn get_default<T: Copy>(slice: &[T], index: usize, default: T) -> T {
    return if slice.len() < index { slice[index] } else { default };
}