#![cfg_attr(feature = "strict", deny(warnings))]

mod slice;

#[allow(dead_code)]
fn reorder(seq: &mut Vec<i32>, start: usize, end: usize, dest: usize) {
    println!("reorder: {:?}, {}, {}, {}", seq, start, end, dest);
    let selected_seq = slice::get_slice(seq, start, end);
    let remaining_seq = slice::get_slice(seq, end, seq.len());
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
}
