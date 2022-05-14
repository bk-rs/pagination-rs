use core::num::NonZeroUsize;

use crate::page::Page;

//
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Pages {
    total_count: usize,
    per_page: NonZeroUsize,
    total_pages: usize,
    //
    finished: bool,
    curr_front: Option<NonZeroUsize>,
    curr_back: Option<NonZeroUsize>,
}

impl Pages {
    pub(crate) fn new(total_count: usize, per_page: NonZeroUsize, total_pages: usize) -> Self {
        Self {
            total_count,
            per_page,
            total_pages,
            //
            finished: Default::default(),
            curr_front: Default::default(),
            curr_back: Default::default(),
        }
    }
}

//
impl Iterator for Pages {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        todo!()
    }
}

impl DoubleEndedIterator for Pages {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        todo!()
    }
}
