use std::ops::Range;

use lexer::{LexInput, SQLLexError};
use pyo3::prelude::*;
use token::Token;

mod lexer;
mod marker;
mod templater;
mod token;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn rust_lex(
    raw: String,
    template_blocks_indent: bool,
) -> PyResult<(Vec<Token>, Vec<SQLLexError>)> {
    let input_str = LexInput::String(raw);
    Ok(lexer::lex(input_str, template_blocks_indent))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rsqlfluff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
