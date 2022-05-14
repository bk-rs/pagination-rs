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
pub struct Page<'a> {
    paginator: &'a Paginator,
    //
    pub curr_page: NonZeroUsize,
}

impl<'a> Page<'a> {
    pub(crate) fn new(
        paginator: &'a Paginator,
        //
        curr_page: NonZeroUsize,
    ) -> Self {
        Self {
            paginator,
            //
            curr_page,
        }
    }

    pub fn is_first_page(&self) -> bool {
        self.curr_page.get() == FIRST_PAGE
    }

    pub fn is_last_page(&self) -> bool {
        self.curr_page.get() == self.paginator.total_pages()
    }

    pub fn next_page(&self) -> Option<Self> {
        if self.is_last_page() {
            None
        } else {
            self.paginator.page(self.curr_page.get() + 1)
        }
    }

    pub fn prev_page(&self) -> Option<Self> {
        if self.is_first_page() {
            None
        } else {
            self.paginator.page(self.curr_page.get() - 1)
        }
    }

    pub fn limit_value(&self) -> NonZeroUsize {
        self.paginator.per_page()
    }

    pub fn offset_value(&self) -> usize {
        self.offset_range().start
    }

    pub fn offset_range(&self) -> Range<usize> {
        let start = (self.curr_page.get() - 1) * self.limit_value().get();
        let end = min(
            self.curr_page.get() * self.limit_value().get(),
            self.paginator.total_count,
        );
        Range { start, end }
    }
}

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SlicePage<'a, 'i, T> {
    all_items: &'i [T],
    pub page: Page<'a>,
}

impl<'a, 'i, T> Deref for SlicePage<'a, 'i, T> {
    type Target = Page<'a>;

    fn deref(&self) -> &Self::Target {
        &self.page
    }
}

impl<'a, 'i, T> SlicePage<'a, 'i, T> {
    pub(crate) fn new(all_items: &'i [T], page: Page<'a>) -> Self {
        Self { all_items, page }
    }

    pub fn items(&self) -> &'i [T] {
        &self.all_items[self.offset_range()]
    }
}
