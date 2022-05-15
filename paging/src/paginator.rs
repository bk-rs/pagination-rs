use core::{num::NonZeroUsize, ops::Deref};

use crate::{
    page::{Page, SlicePage, FIRST_PAGE},
    pages::{Pages, SlicePages},
};

//
pub const DEFAULT_PER_PAGE: usize = 25;

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Paginator {
    pub total_count: usize,
    per_page: Option<NonZeroUsize>,
}

impl Paginator {
    pub fn new(total_count: usize, per_page: impl Into<Option<usize>>) -> Self {
        let per_page = per_page.into();

        Self {
            total_count,
            per_page: per_page.and_then(NonZeroUsize::new),
        }
    }

    pub fn per_page(&self) -> NonZeroUsize {
        self.per_page
            .unwrap_or_else(|| unsafe { NonZeroUsize::new_unchecked(DEFAULT_PER_PAGE) })
    }

    pub fn total_pages(&self) -> usize {
        (self.total_count as f64 / self.per_page().get() as f64).ceil() as usize
    }

    pub fn page(&self, value: usize) -> Option<Page> {
        let page_value = NonZeroUsize::new(value)
            .unwrap_or_else(|| unsafe { NonZeroUsize::new_unchecked(FIRST_PAGE) });

        // out_of_range
        if page_value.get() > self.total_pages() {
            return None;
        }

        Some(Page::new(*self, page_value))
    }

    pub fn pages(self) -> Pages {
        Pages::new(self)
    }
}

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SlicePaginator<'i, T> {
    all_items: &'i [T],
    paginator: Paginator,
}

impl<'i, T> Deref for SlicePaginator<'i, T> {
    type Target = Paginator;

    fn deref(&self) -> &Self::Target {
        &self.paginator
    }
}

impl<'i, T> SlicePaginator<'i, T> {
    pub fn new(all_items: &'i [T], per_page: impl Into<Option<usize>>) -> Self {
        let per_page = per_page.into();

        Self {
            all_items,
            paginator: Paginator::new(all_items.len(), per_page),
        }
    }

    pub fn page(&self, n: usize) -> Option<SlicePage<'i, T>> {
        self.paginator
            .page(n)
            .map(|x| SlicePage::new(self.all_items, x))
    }

    pub fn pages(self) -> SlicePages<'i, T> {
        let pages = self.paginator.pages();
        SlicePages::new(self.all_items, pages)
    }
}
