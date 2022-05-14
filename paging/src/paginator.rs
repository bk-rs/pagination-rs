use core::num::NonZeroUsize;

//
pub const DEFAULT_PER_PAGE: usize = 25;
pub const FIRST_PAGE: usize = 1;

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
            per_page: per_page.and_then(|x| NonZeroUsize::new(x)),
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

        // out_of_range
        if curr_page.get() > self.total_pages() {
            return None;
        }

        Some(Page {
            total_count: self.total_count,
            per_page: self.per_page(),
            total_pages: self.total_pages(),
            curr_page,
        })
    }
}

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Page {
    pub total_count: usize,
    per_page: NonZeroUsize,
    total_pages: usize,
    pub curr_page: NonZeroUsize,
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

    pub fn limit_value(&self) -> usize {
        self.per_page.get()
    }
}
