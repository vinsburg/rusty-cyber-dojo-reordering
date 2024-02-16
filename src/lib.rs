#![cfg_attr(feature = "strict", deny(warnings))]

mod slice;

#[allow(dead_code)]
fn reorder(seq: &mut Vec<i32>, start: usize, end: usize, dest: usize) {
    println!("reorder: {:?}, {}, {}, {}", seq, start, end, dest);
    let selected_seq: Vec<i32> = slice::get_slice(seq, start, end).to_vec();
    let before_seq: Vec<i32> = slice::get_slice(seq, 0, start).to_vec();
    let after_seq: Vec<i32> = slice::get_slice(seq, end, seq.len()).to_vec();
    let remaining_seq: Vec<i32> = [before_seq, after_seq].concat();
    let result: Vec<i32> = [remaining_seq, selected_seq].concat();
    seq.clear();
    seq.extend(result);
}

#[cfg(test)]
mod tests {
    use super::reorder;

    #[test]
    fn should_reorder_empty_sub_sequence() {
        let mut seq: Vec<i32> = vec![1, 2];
        let expected: Vec<i32> = vec![1, 2];
        reorder(&mut seq, 0, 0, 0);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_first_member_to_second_place() {
        let mut seq: Vec<i32> = vec![1, 2];
        let expected: Vec<i32> = vec![2, 1];
        reorder(&mut seq, 0, 1, 1);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_third_place() {
        let mut seq: Vec<i32> = vec![1, 2, 3];
        let expected: Vec<i32> = vec![1, 3, 2];
        reorder(&mut seq, 1, 2, 2);
        assert_eq!(expected, seq);
    }
}
