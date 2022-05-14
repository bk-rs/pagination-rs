// https://github.com/kaminari/kaminari/blob/master/kaminari-core/test/models/array_test.rb

use paging::paginate_slice;

#[test]
fn test_paginate_slice() {
    // array 1..15, page 1, per 10, total_count 15
    let items = (1..=15).collect::<Vec<_>>();
    let page = paginate_slice(&items, Some(10)).page(1).unwrap();
    assert_eq!(page.items(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}