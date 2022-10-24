//! This module implements the `par_iter()` method on iterators to call `iter()` instead.
//!
//! This only exists to reduce config attributes when we call `par_iter()`
//! but we're compiling for Wasm.

use std::slice::Iter;

pub trait VecParIter {
    type Item;

    fn par_iter(&self) -> Iter<Self::Item>;
}

impl<T> VecParIter for Vec<T> {
    type Item = T;

    #[inline(always)]
    fn par_iter(&self) -> Iter<T> {
        self.iter()
    }
}
