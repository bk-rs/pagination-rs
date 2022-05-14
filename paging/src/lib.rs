//
pub mod page;
pub mod paginator;

pub use page::{Page, SlicePage};
pub use paginator::{Paginator, SlicePaginator};

//
pub fn paginate_slice<T>(items: &[T], per_page: Option<usize>) -> SlicePaginator<'_, T> {
    SlicePaginator::new(items, per_page)
}
