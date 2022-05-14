//
pub mod page;
pub mod pages;
pub mod paginator;

pub use page::{Page, SlicePage};
pub use pages::Pages;
pub use paginator::{Paginator, SlicePaginator};

//
pub fn paginate(total_count: usize, per_page: impl Into<Option<usize>>) -> Paginator {
    Paginator::new(total_count, per_page)
}

pub fn paginate_slice<T>(
    all_items: &[T],
    per_page: impl Into<Option<usize>>,
) -> SlicePaginator<'_, T> {
    SlicePaginator::new(all_items, per_page)
}
