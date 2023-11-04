#![cfg_attr(feature = "strict", deny(warnings))]

use reordering;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, reordering::answer());
}
