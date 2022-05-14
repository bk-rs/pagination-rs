use core::{cmp::min, num::NonZeroUsize};

//
pub const FIRST_PAGE: usize = 1;

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Page {
    curr_page: NonZeroUsize,
    //
    total_count: usize,
    per_page: NonZeroUsize,
    total_pages: usize,
}
impl Page {
    pub(crate) fn new(
        curr_page: NonZeroUsize,
        //
        total_count: usize,
        per_page: NonZeroUsize,
        total_pages: usize,
    ) -> Self {
        Self {
            curr_page,
            //
            total_count,
            per_page,
            total_pages,
        }
    }
}

impl Page {
    pub fn is_first_page(&self) -> bool {
        self.curr_page.get() == FIRST_PAGE
    }

    pub fn is_last_page(&self) -> bool {
        self.curr_page.get() == self.total_pages
    }

    pub fn next_page(&self) -> Option<NonZeroUsize> {
        if self.is_last_page() {
            None
        } else {
            Some(unsafe { NonZeroUsize::new_unchecked(self.curr_page.get() + 1) })
        }
    }

    pub fn prev_page(&self) -> Option<NonZeroUsize> {
        if self.is_first_page() {
            None
        } else {
            Some(unsafe { NonZeroUsize::new_unchecked(self.curr_page.get() - 1) })
        }
    }

    pub fn curr_page(&self) -> NonZeroUsize {
        self.curr_page
    }

    pub fn limit_value(&self) -> NonZeroUsize {
        self.per_page
    }

    pub fn offset_begin(&self) -> usize {
        (self.curr_page().get() - 1) * self.limit_value().get()
    }

    pub fn offset_end(&self) -> usize {
        min(
            self.curr_page().get() * self.limit_value().get(),
            self.total_count,
        )
    }
}
