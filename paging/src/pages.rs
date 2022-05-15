use core::ops::Deref;

use crate::{
    page::{Page, SlicePage, FIRST_PAGE},
    paginator::Paginator,
};

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Pages {
    paginator: Paginator,
    //
    finished: bool,
    curr_front: Option<Page>,
    curr_back: Option<Page>,
}

impl Deref for Pages {
    type Target = Paginator;

    fn deref(&self) -> &Self::Target {
        &self.paginator
    }
}

impl Pages {
    pub(crate) fn new(paginator: Paginator) -> Self {
        Self {
            paginator,
            //
            finished: Default::default(),
            curr_front: Default::default(),
            curr_back: Default::default(),
        }
    }
}

impl Iterator for Pages {
    type Item = Page;

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

impl DoubleEndedIterator for Pages {
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

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SlicePages<'i, T> {
    all_items: &'i [T],
    pages: Pages,
}

impl<'i, T> Deref for SlicePages<'i, T> {
    type Target = Pages;

    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}

impl<'i, T> SlicePages<'i, T> {
    pub(crate) fn new(all_items: &'i [T], pages: Pages) -> Self {
        Self { all_items, pages }
    }
}

impl<'i, T> Iterator for SlicePages<'i, T> {
    type Item = SlicePage<'i, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pages
            .next()
            .map(|page| SlicePage::new(self.all_items, page))
    }
}

impl<'i, T> DoubleEndedIterator for SlicePages<'i, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pages
            .next_back()
            .map(|page| SlicePage::new(self.all_items, page))
    }
}
