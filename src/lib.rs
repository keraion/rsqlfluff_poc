pub mod config;
pub mod dialect;
pub mod lexer;
pub mod marker;
pub mod matcher;
pub mod slice;
pub mod templater;
pub mod token;
// include!(concat!(env!("OUT_DIR"), "/dialect_matcher.rs"));

use dialect::matcher::*;
use lexer::python::{PyLexer, PySQLLexError};
use marker::python::PyPositionMarker;
use pyo3::prelude::*;
use templater::{
    fileslice::python::{PyRawFileSlice, PyTemplatedFileSlice},
    templatefile::python::PyTemplatedFile,
};
use token::python::PyToken;

/// A Python module implemented in Rust.
#[pymodule]
fn rsqlfluff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyToken>()?;
    m.add_class::<PyTemplatedFile>()?;
    m.add_class::<PyTemplatedFileSlice>()?;
    m.add_class::<PyRawFileSlice>()?;
    m.add_class::<PySQLLexError>()?;
    m.add_class::<PyLexer>()?;
    m.add_class::<PyPositionMarker>()?;
    Ok(())
}
