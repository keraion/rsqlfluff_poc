use std::ops::Range;

use pyo3::{pyclass, FromPyObject, IntoPyObject};

#[derive(Debug, PartialEq, Hash, Eq, Clone, FromPyObject, IntoPyObject)]
pub struct Slice {
    pub start: usize,
    pub stop: usize,
}

impl From<Range<usize>> for Slice {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            stop: value.end,
        }
    }
}

impl Slice {
    pub fn slice_is_point(test_slice: &Range<usize>) -> bool {
        test_slice.start == test_slice.end
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..self.stop
    }

    pub fn len(&self) -> usize {
        self.stop - self.start
    }
}
