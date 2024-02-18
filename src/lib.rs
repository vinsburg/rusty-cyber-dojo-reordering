#![cfg_attr(feature = "strict", deny(warnings))]

pub fn reorder<T>(seq: &mut Vec<T>, inds: ReorderIndices) {
    let removed: Vec<T> = seq.drain(inds.start..inds.end).collect();
    let dest = adjust_destination_index(inds);
    seq.splice(dest..dest, removed);
}

pub struct ReorderIndices {
    pub start: usize,
    pub end: usize,
    pub dest: usize,
}

fn adjust_destination_index(inds: ReorderIndices) -> usize {
    if inds.dest > inds.end {
        inds.dest - (inds.end - inds.start)
    } else {
        inds.dest
    }
}

#[cfg(test)]
mod tests {
    use super::reorder;
    use super::ReorderIndices;

    #[test]
    fn should_reorder_empty_sub_sequence() {
        let mut seq: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![0, 1];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 0,
            end: 0,
            dest: 0,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_first_member_to_second_position() {
        let mut seq: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![1, 0];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 0,
            end: 1,
            dest: 1,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_third_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![0, 2, 1];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 1,
            end: 2,
            dest: 2,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_second_member_to_first_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2];
        let expected: Vec<i32> = vec![1, 0, 2];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 1,
            end: 2,
            dest: 0,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_1to3_to_8th_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected: Vec<i32> = vec![0, 4, 5, 6, 7, 1, 2, 3, 8, 9];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 1,
            end: 4,
            dest: 8,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }

    #[test]
    fn should_move_1to3_to_2nd_position() {
        let mut seq: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected: Vec<i32> = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9];
        let inds: ReorderIndices = super::ReorderIndices {
            start: 1,
            end: 4,
            dest: 2,
        };
        reorder(&mut seq, inds);
        assert_eq!(expected, seq);
    }
}
