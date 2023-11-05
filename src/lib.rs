#![cfg_attr(feature = "strict", deny(warnings))]

fn get_sub_sequence(seq: &Vec<i32>, start: usize, end: usize) -> &[i32] {
    &seq[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn should_return_empty_sub_sequence() {
        let seq: Vec<i32> = vec![1, 2];
        let expected: Vec<i32> = vec![];
        assert_eq!(&expected, get_sub_sequence(&seq, 0, 0));
    }

    #[test]
    fn should_get_slice_from_beginning() {
        let seq: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: Vec<i32> = vec![1];
        assert_eq!(&expected, get_sub_sequence(&seq, 0, 1));
    }

    #[test]
    fn should_get_slice_from_end() {
        let seq: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: Vec<i32> = vec![5];
        assert_eq!(&expected, get_sub_sequence(&seq, 4, 5));
    }

    #[test]
    fn should_get_slice_from_middle() {
        let seq: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: Vec<i32> = vec![2, 3, 4];
        assert_eq!(&expected, get_sub_sequence(&seq, 1, 4));
    }
}
