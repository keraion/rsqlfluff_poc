pub mod dialect;
pub mod lexer;
pub mod marker;
pub mod matcher;
pub mod slice;
pub mod templater;
pub mod token;
// include!(concat!(env!("OUT_DIR"), "/dialect_matcher.rs"));

use dialect::matcher::*;
use lexer::{python::PyLexInput, SQLLexError};
use pyo3::prelude::*;
use templater::{
    fileslice::python::{PyRawFileSlice, PyTemplatedFileSlice},
    templatefile::python::PyTemplatedFile,
};
use token::Token;

use std::str::FromStr;

#[pyfunction]
#[pyo3(name = "lex")]
#[pyo3(signature = (input, template_blocks_indent, dialect))]
fn py_lex(
    input: PyLexInput,
    template_blocks_indent: bool,
    dialect: String,
) -> PyResult<(Vec<Token>, Vec<SQLLexError>)> {
    let dialect = Dialect::from_str(&dialect).expect("Invalid dialect");
    Ok(lexer::lex(input.into(), template_blocks_indent, dialect))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rsqlfluff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Token>()?;
    m.add_class::<PyTemplatedFile>()?;
    m.add_class::<PyTemplatedFileSlice>()?;
    m.add_class::<PyRawFileSlice>()?;
    m.add_function(wrap_pyfunction!(py_lex, m)?)?;
    Ok(())
}
