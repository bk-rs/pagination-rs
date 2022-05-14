// https://github.com/kaminari/kaminari/blob/master/kaminari-core/test/models/array_test.rb

use paging::paginate;

//
const TOTAL_COUNT: usize = 100;

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L98
#[test]
fn test_total_pages() {
    // per 25 (default)
    assert_eq!(paginate(TOTAL_COUNT, None).total_pages(), 4);

    // per 7
    assert_eq!(paginate(TOTAL_COUNT, 7).total_pages(), 15);

    // per 65536
    assert_eq!(paginate(TOTAL_COUNT, 65536).total_pages(), 1);

    // per 0
    // Eq per 25 (default)
    assert_eq!(paginate(TOTAL_COUNT, 0).total_pages(), 4);
}

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L138
#[test]
fn test_next_page() {
    // page 1
    assert_eq!(
        paginate(TOTAL_COUNT, None)
            .page(1)
            .unwrap()
            .next_page()
            .map(|x| x.get()),
        Some(2)
    );

    assert_eq!(
        paginate(TOTAL_COUNT, None).page(4).unwrap().next_page(),
        None
    );
}

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L148
#[test]
fn test_prev_page() {
    // page 1
    assert_eq!(
        paginate(TOTAL_COUNT, None).page(1).unwrap().prev_page(),
        None
    );

    // page 3
    assert_eq!(
        paginate(TOTAL_COUNT, None)
            .page(3)
            .unwrap()
            .prev_page()
            .map(|x| x.get()),
        Some(2)
    );

    assert_eq!(
        paginate(TOTAL_COUNT, None)
            .page(4)
            .unwrap()
            .prev_page()
            .map(|x| x.get()),
        Some(3)
    );
}
