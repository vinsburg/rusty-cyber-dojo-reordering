#![cfg_attr(feature = "strict", deny(warnings))]

mod slice;

#[allow(dead_code)]
fn reorder(seq: &mut Vec<i32>, start: usize, end: usize, dest: usize) {
    let selected_seq: Vec<i32> = slice::get_slice(seq, start, end);
    let before_seq: Vec<i32> = slice::get_slice(seq, 0, start);
    let after_seq: Vec<i32> = slice::get_slice(seq, end, seq.len());
    let remaining_seq: Vec<i32> = [before_seq, after_seq].concat();
    let new_dest: usize = adjust_destination_index(dest, start, end);
    let new_before_seq: Vec<i32> = slice::get_slice(&remaining_seq, 0, new_dest);
    let new_after_seq: Vec<i32> = slice::get_slice(&remaining_seq, new_dest, remaining_seq.len());
    let result: Vec<i32> = [new_before_seq, selected_seq, new_after_seq].concat();
    seq.clear();
    seq.extend(result);
}

fn adjust_destination_index(dest: usize, start: usize, end: usize) -> usize {
    if dest > end {
        dest - end + start
    } else {
        dest
    }
}

#[cfg(test)]
mod tests {
    use super::reorder;

    #[test]
    fn should_reorder_empty_sub_sequence() {
        let mut seq: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![0, 1];
        reorder(&mut seq, 0, 0, 0);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_first_member_to_second_place() {
        let mut seq: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![1, 0];
        reorder(&mut seq, 0, 1, 1);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_third_place() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![0, 2, 1];
        reorder(&mut seq, 1, 2, 2);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_first_place() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![1, 0, 2];
        reorder(&mut seq, 1, 2, 0);
        assert_eq!(expected, seq);
    }
}
