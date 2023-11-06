#![cfg_attr(feature = "strict", deny(warnings))]

mod slice;

#[allow(dead_code)]
fn reorder(seq: &mut Vec<i32>, start: usize, end: usize, dest: usize) {
    println!("reorder: {:?}, {}, {}, {}", seq, start, end, dest);
}

#[cfg(test)]
mod tests {
    use super::reorder;
    use std::vec::Vec;

    #[test]
    fn should_reorder_empty_sub_sequence() {
        let mut seq: Vec<i32> = vec![1, 2];
        let expected: Vec<i32> = vec![1, 2];
        reorder(&mut seq, 0, 0, 0);
        assert_eq!(expected, seq);
    }
}
