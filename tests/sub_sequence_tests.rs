// #![cfg_attr(feature = "strict", deny(warnings))]

use std::vec::Vec;
use reordering;

#[test]
fn should_return_empty_sub_sequence() {
    let seq: Vec<i32> = vec![1,2];
    let expected: Vec<i32> = vec![];
    assert_eq!(&expected, reordering::get_sub_sequence(&seq, 0, 0));
}
