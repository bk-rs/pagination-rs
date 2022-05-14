use core::{num::NonZeroUsize, ops::Deref};

use crate::page::{Page, SlicePage, FIRST_PAGE};

//
pub const DEFAULT_PER_PAGE: usize = 25;

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Paginator {
    pub total_count: usize,
    per_page: Option<NonZeroUsize>,
}

impl Paginator {
    pub fn new(total_count: usize, per_page: Option<usize>) -> Self {
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

    pub fn page(&self, n: usize) -> Option<Page> {
        let curr_page = NonZeroUsize::new(n)
            .unwrap_or_else(|| unsafe { NonZeroUsize::new_unchecked(FIRST_PAGE) });

        let total_pages = self.total_pages();

        // out_of_range
        if curr_page.get() > total_pages {
            return None;
        }

        Some(Page::new(
            curr_page,
            self.total_count,
            self.per_page(),
            total_pages,
        ))
    }
}

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SlicePaginator<'a, T> {
    items: &'a [T],
    pub paginator: Paginator,
}

impl<'a, T> Deref for SlicePaginator<'a, T> {
    type Target = Paginator;

    fn deref(&self) -> &Self::Target {
        &self.paginator
    }
}

impl<'a, T> SlicePaginator<'a, T> {
    pub fn new(items: &'a [T], per_page: Option<usize>) -> Self {
        Self {
            items,
            paginator: Paginator::new(items.len(), per_page),
        }
    }

    pub fn page(&self, n: usize) -> Option<SlicePage<'a, T>> {
        self.paginator
            .page(n)
            .map(|x| SlicePage::new(self.items, x))
    }
}
