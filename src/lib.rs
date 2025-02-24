pub mod dialect;
pub mod lexer;
pub mod marker;
pub mod matcher;
pub mod slice;
pub mod templater;
pub mod token;
// include!(concat!(env!("OUT_DIR"), "/dialect_matcher.rs"));

use dialect::matcher::*;
use lexer::{LexInput, SQLLexError};
use pyo3::prelude::*;
use token::Token;

use std::str::FromStr;

#[pyfunction]
#[pyo3(name = "lex")]
#[pyo3(signature = (sql, template_blocks_indent, dialect))]
fn py_lex(
    sql: String,
    template_blocks_indent: bool,
    dialect: String,
) -> PyResult<(Vec<Token>, Vec<SQLLexError>)> {
    let input_str = LexInput::String(sql);
    let dialect = Dialect::from_str(&dialect).expect("Invalid dialect");
    Ok(lexer::lex(input_str, template_blocks_indent, dialect))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rsqlfluff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_lex, m)?)?;
    Ok(())
}
