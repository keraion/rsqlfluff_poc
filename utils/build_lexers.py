"""For autogenerating rust lexers."""

import re

from sqlfluff.core.dialects import dialect_readout, dialect_selector
from sqlfluff.core.parser.lexer import LexerType


def segment_to_token_name(s: str):
    return re.sub("([A-Z])", r"_\1", s).strip("_").lower().replace("segment", "token")


def generate_dialect_enum():
    dialects = ",\n".join([dialect.label.capitalize() for dialect in dialect_readout()])
    dialect_match = ",\n".join(
        [
            f"Dialect::{d.label.capitalize()} => &{d.label.upper()}_LEXERS"
            for d in dialect_readout()
        ]
    )
    dialect_strings = ",\n".join(
        [
            f'"{d.label}" => Ok(Dialect::{d.label.capitalize()})'
            for d in dialect_readout()
        ]
    )
    print(f"""
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Dialect {{
    {dialects}
}}

impl FromStr for Dialect {{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            {dialect_strings},
            _ => Err(())
        }}
    }}
}}

pub fn get_lexers(dialect: Dialect) -> &'static Vec<LexMatcher> {{
    match dialect {{
        {dialect_match}
    }}
}}
""")


def generate_lexers():
    print("use once_cell::sync::Lazy;")
    print("use crate::matcher::LexMatcher;")
    print("use std::str::FromStr;")
    for dialect in dialect_readout():
        loaded_dialect = dialect_selector(dialect.label)
        print(
            f"pub static {dialect.label.upper()}_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| {{"
            " vec!["
        )
        for matcher in loaded_dialect.get_lexer_matchers():
            print(f"{_as_rust_lexer_matcher(matcher)},")
        print("]});")


def _as_rust_lexer_matcher(lexer_matcher: LexerType):
    lexer_class = lexer_matcher.__class__.__name__
    segment_name = segment_to_token_name(lexer_matcher.segment_class.__name__)
    subdivider = (
        f"Some(Box::new({_as_rust_lexer_matcher(lexer_matcher.subdivider)}))"
        if lexer_matcher.subdivider
        else None
    )
    trim_post_subdivide = (
        f"Some(Box::new({_as_rust_lexer_matcher(lexer_matcher.trim_post_subdivide)}))"
        if lexer_matcher.trim_post_subdivide
        else None
    )

    if lexer_class == "StringLexer":
        rust_fn = "string_lexer"
        template = f'"{lexer_matcher.template}"'
    elif lexer_class == "RegexLexer":
        rust_fn = "regex_lexer"
        template = f'r#"{lexer_matcher.template}"#'
    else:
        raise ValueError
    return f"""
    LexMatcher::{rust_fn}(
        "{lexer_matcher.name}",
        {template},
        Token::{segment_name},
        {subdivider},
        {trim_post_subdivide},
    )"""


generate_lexers()
generate_dialect_enum()
