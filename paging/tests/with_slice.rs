// https://github.com/kaminari/kaminari/blob/master/kaminari-core/test/models/array_test.rb

use paging::paginate_slice;

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L172
#[test]
fn test_items() {
    // array 1..15, page 1, per 10, total_count 15
    let all_items = (1..=15).collect::<Vec<_>>();
    let page = paginate_slice(&all_items, 10).page(1);
    assert_eq!(page.unwrap().items(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // array 1..25, page 2, per 10, total_count 15
    let all_items = (1..=25).collect::<Vec<_>>();
    let page = paginate_slice(&all_items[..15], 10).page(2);
    assert_eq!(page.unwrap().items(), &[11, 12, 13, 14, 15]);

    let page = paginate_slice(&all_items[..15], 10).page(3);
    assert!(page.is_none());
}
