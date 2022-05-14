use crate::{
    page::{Page, FIRST_PAGE},
    paginator::Paginator,
};

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Pages<'a> {
    paginator: &'a Paginator,
    //
    finished: bool,
    curr_front: Option<Page<'a>>,
    curr_back: Option<Page<'a>>,
}

impl<'a> Pages<'a> {
    pub(crate) fn new(paginator: &'a Paginator) -> Self {
        Self {
            paginator,
            //
            finished: Default::default(),
            curr_front: Default::default(),
            curr_back: Default::default(),
        }
    }
}

impl<'a> Iterator for Pages<'a> {
    type Item = Page<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = if let Some(page) = &self.curr_front {
            page.next_page()
        } else {
            self.paginator.page(FIRST_PAGE)
        };
        if next == self.curr_back {
            self.finished = true;
            return None;
        }
        self.curr_front = next.to_owned();
        next
    }
}

impl<'a> DoubleEndedIterator for Pages<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let prev = if let Some(page) = &self.curr_back {
            page.prev_page()
        } else {
            self.paginator.page(self.paginator.total_pages())
        };
        if prev == self.curr_front {
            self.finished = true;
            return None;
        }
        self.curr_back = prev.to_owned();
        prev
    }
}
