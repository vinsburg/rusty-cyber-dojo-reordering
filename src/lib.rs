#![cfg_attr(feature = "strict", deny(warnings))]

#[allow(dead_code)]
fn reorder<T>(seq: &mut Vec<T>, start: usize, end: usize, dest: usize) {
    let removed: Vec<T> = seq.drain(start..end).collect();
    let dest = adjust_destination_index(dest, start, end);
    seq.splice(dest..dest, removed);
}

fn adjust_destination_index(dest: usize, start: usize, end: usize) -> usize {
    if dest > end {
        dest - (end - start)
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
    fn should_move_first_member_to_second_position() {
        let mut seq: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![1, 0];
        reorder(&mut seq, 0, 1, 1);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_third_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![0, 2, 1];
        reorder(&mut seq, 1, 2, 2);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_first_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![1, 0, 2];
        reorder(&mut seq, 1, 2, 0);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_1to3_to_8th_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected: Vec<i32> = vec![0, 4, 5, 6, 7, 1, 2, 3, 8, 9];
        reorder(&mut seq, 1, 4, 8);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_1to3_to_2nd_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected: Vec<i32> = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9];
        reorder(&mut seq, 1, 4, 2);
        assert_eq!(expected, seq);
    }
}
