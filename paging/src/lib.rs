//
pub mod page;
pub mod pages;
pub mod paginator;

pub use page::{Page, SlicePage};
pub use pages::{Pages, SlicePages};
pub use paginator::{Paginator, SlicePaginator};

//
pub fn paginate_slice<T>(
    all_items: &[T],
    per_page: impl Into<Option<usize>>,
) -> SlicePaginator<'_, T> {
    SlicePaginator::new(all_items, per_page)
}
