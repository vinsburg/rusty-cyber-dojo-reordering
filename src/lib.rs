#![cfg_attr(feature = "strict", deny(warnings))]

pub fn get_sub_sequence(sequence: &[usize], start: usize, end:usize) -> &[usize] {
    return &sequence[end as usize..start as usize];
}
