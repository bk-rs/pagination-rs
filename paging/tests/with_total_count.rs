// https://github.com/kaminari/kaminari/blob/master/kaminari-core/test/models/array_test.rb

use paging::Paginator;

//
const TOTAL_COUNT: usize = 100;

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L98
#[test]
fn test_paginator_total_pages() {
    // per 25 (default)
    assert_eq!(Paginator::new(TOTAL_COUNT, None).total_pages(), 4);

    // per 7
    assert_eq!(Paginator::new(TOTAL_COUNT, 7).total_pages(), 15);

    // per 65536
    assert_eq!(Paginator::new(TOTAL_COUNT, 65536).total_pages(), 1);

    // per 0
    // Eq per 25 (default)
    assert_eq!(Paginator::new(TOTAL_COUNT, 0).total_pages(), 4);
}

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L138
#[test]
fn test_page_next_page() {
    // page 1
    assert_eq!(
        Paginator::new(TOTAL_COUNT, None)
            .page(1)
            .unwrap()
            .next_page()
            .map(|x| x.page_value().get()),
        Some(2)
    );

    assert_eq!(
        Paginator::new(TOTAL_COUNT, None)
            .page(4)
            .unwrap()
            .next_page(),
        None
    );
}

//
// https://github.com/kaminari/kaminari/blob/v1.2.2/kaminari-core/test/models/array_test.rb#L148
#[test]
fn test_page_prev_page() {
    // page 1
    assert_eq!(
        Paginator::new(TOTAL_COUNT, None)
            .page(1)
            .unwrap()
            .prev_page(),
        None
    );

    // page 3
    assert_eq!(
        Paginator::new(TOTAL_COUNT, None)
            .page(3)
            .unwrap()
            .prev_page()
            .map(|x| x.page_value().get()),
        Some(2)
    );

    assert_eq!(
        Paginator::new(TOTAL_COUNT, None)
            .page(4)
            .unwrap()
            .prev_page()
            .map(|x| x.page_value().get()),
        Some(3)
    );
}

#[test]
fn test_pages() {
    let mut pages = Paginator::new(TOTAL_COUNT, None).pages();
    assert_eq!(pages.next().unwrap().page_value().get(), 1);
    assert_eq!(pages.next_back().unwrap().page_value().get(), 4);
    assert_eq!(pages.next_back().unwrap().page_value().get(), 3);
    assert_eq!(pages.next().unwrap().page_value().get(), 2);
    assert!(pages.next().is_none());
    assert!(pages.next_back().is_none());
}
