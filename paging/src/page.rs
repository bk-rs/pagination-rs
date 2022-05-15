use core::{
    cmp::min,
    num::NonZeroUsize,
    ops::{Deref, Range},
};

use crate::paginator::Paginator;

//
pub const FIRST_PAGE: usize = 1;

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Page {
    paginator: Paginator,
    page_value: NonZeroUsize,
}

impl Deref for Page {
    type Target = Paginator;

    fn deref(&self) -> &Self::Target {
        &self.paginator
    }
}

impl Page {
    pub(crate) fn new(paginator: Paginator, page_value: NonZeroUsize) -> Self {
        Self {
            paginator,
            page_value,
        }
    }

    pub fn is_first_page(&self) -> bool {
        self.page_value.get() == FIRST_PAGE
    }

    pub fn is_last_page(&self) -> bool {
        self.page_value.get() == self.paginator.total_pages()
    }

    pub fn next_page(&self) -> Option<Self> {
        if self.is_last_page() {
            None
        } else {
            self.paginator.page(self.page_value.get() + 1)
        }
    }

    pub fn prev_page(&self) -> Option<Self> {
        if self.is_first_page() {
            None
        } else {
            self.paginator.page(self.page_value.get() - 1)
        }
    }

    pub fn page_value(&self) -> NonZeroUsize {
        self.page_value
    }

    pub fn limit_value(&self) -> NonZeroUsize {
        self.paginator.per_page()
    }

    pub fn offset_value(&self) -> usize {
        self.offset_range().start
    }

    pub fn offset_range(&self) -> Range<usize> {
        let start = (self.page_value.get() - 1) * self.limit_value().get();
        let end = min(
            self.page_value.get() * self.limit_value().get(),
            self.paginator.total_count,
        );
        Range { start, end }
    }
}

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SlicePage<'i, T> {
    all_items: &'i [T],
    page: Page,
}

impl<'i, T> Deref for SlicePage<'i, T> {
    type Target = Page;

    fn deref(&self) -> &Self::Target {
        &self.page
    }
}

impl<'i, T> SlicePage<'i, T> {
    pub(crate) fn new(all_items: &'i [T], page: Page) -> Self {
        Self { all_items, page }
    }

    pub fn items(&self) -> &'i [T] {
        &self.all_items[self.offset_range()]
    }
}
