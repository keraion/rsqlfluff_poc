use std::ops::Range;

use pyo3::pyclass;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Slice {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Slice {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl Slice {
    pub fn slice_is_point(test_slice: &Range<usize>) -> bool {
        test_slice.start == test_slice.end
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
