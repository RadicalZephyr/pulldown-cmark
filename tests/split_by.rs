extern crate pulldown_cmark;

use pulldown_cmark::split_by::split_by;

#[test]
fn test_it() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let (first_three, last_three) = split_by(nums.into_iter(), |&i| i < 4 );
    assert_eq!(vec![1, 2, 3],
               first_three.collect::<Vec<u8>>());
    assert_eq!(vec![4, 5, 6],
               last_three.collect::<Vec<u8>>());

}
