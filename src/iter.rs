use std::cmp::Ord;
use std::cmp::Ordering;
use std::iter::Iterator;

use task::Task;

/// A convenience iterator for tasks, providing some functionality for sorting and filtering for
/// the tasks it iterates over
pub struct TaskIterator<I: Iterator<Item = Task>> {
    i: I
}

impl<I: Iterator<Item = Task>> TaskIterator<I> {

    pub fn new(i: I) -> TaskIterator<I> {
        TaskIterator {
            i: i
        }
    }

    pub fn sort_by_fn<F>(self, f: F) -> TaskIterator<I>
        where F: FnOnce(&Task, &Task) -> Ordering
    {
        unimplemented!()
    }

    pub fn sort_by_prio(self) -> TaskIterator<I> {
        self.sort_by_fn(|a, b| {
            if let Some(a) = a.priority() {
                if let Some(b) = b.priority() {
                    a.clone().cmp(b.clone())
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Greater
            }
        })
    }

    pub fn sort_by_entry(self) -> TaskIterator<I> {
        unimplemented!()
    }

    pub fn sort_by_due(self) -> TaskIterator<I> {
        unimplemented!()
    }

    pub fn sort_by_scheduled(self) -> TaskIterator<I> {
        unimplemented!()
    }

    pub fn sort_by_num_annotations(self) -> TaskIterator<I> {
        unimplemented!()
    }

    pub fn filter_by_fn<F>(self, f: F) -> TaskIterator<I>
        where F: FnOnce(&Task) -> bool
    {
        unimplemented!()
    }

}
