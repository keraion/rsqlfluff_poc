use std::ops::Range;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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

pub mod python {
    use super::Slice;
    use pyo3::{prelude::*, types::PySlice};

    impl<'py> FromPyObject<'py> for Slice {
        fn extract_bound(obj: &pyo3::Bound<'py, pyo3::PyAny>) -> PyResult<Self> {
            let start = obj.getattr("start")?.extract::<usize>()?;
            let stop = obj.getattr("stop")?.extract::<usize>()?;
            Ok(Slice { start, stop })
        }
    }

    impl<'py> IntoPyObject<'py> for Slice {
        type Target = PySlice; // the Python type
        type Output = Bound<'py, Self::Target>; // in most cases this will be `Bound`
        type Error = PyErr; // the conversion error type, has to be convertable to `PyErr`

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            Ok(PySlice::new(
                py,
                isize::try_from(self.start)?,
                isize::try_from(self.stop)?,
                isize::try_from(1)?,
            ))
        }
    }
}
