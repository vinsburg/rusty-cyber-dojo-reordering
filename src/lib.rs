// #![cfg_attr(feature = "strict", deny(warnings))]

pub fn get_sub_sequence(seq: &Vec<i32>, start: usize, end:usize) -> &[i32] {
    &seq[start..end]
}
