extern crate pulldown_cmark;

use pulldown_cmark::split_by::{SplitBy};

#[test]
fn test_it() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let (first_three, last_three) = nums.into_iter().split_by(|&i| i > 3 );
    assert_eq!(vec![1, 2, 3],
               first_three.collect::<Vec<u8>>());
    assert_eq!(vec![4, 5, 6],
               last_three.collect::<Vec<u8>>());

}
