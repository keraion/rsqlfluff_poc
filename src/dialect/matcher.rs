/* This is a generated file! */
use once_cell::sync::Lazy;
use crate::matcher::{LexMatcher, extract_nested_block_comment};
use std::str::FromStr;
use crate::token::Token;

pub static ANSI_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static ATHENA_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static BIGQUERY_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"([rR]?[bB]?|[bB]?[rR]?)?('''((?<!\\)(\\{2})*\\'|'{,2}(?!')|[^'])*(?<!\\)(\\{2})*'''|'((?<!\\)(\\{2})*\\'|[^'])*(?<!\\)(\\{2})*')"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#"([rR]?[bB]?|[bB]?[rR]?)?(\"\"\"((?<!\\)(\\{2})*\\\"|\"{,2}(?!\")|[^\"])*(?<!\\)(\\{2})*\"\"\"|"((?<!\\)(\\{2})*\\"|[^"])*(?<!\\)(\\{2})*")"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question_mark",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign_literal",
        r#"@[a-zA-Z_][\w]*"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_at_sign_literal",
        r#"@@[a-zA-Z_][\w\.]*"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static CLICKHOUSE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"\\]|""|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|``|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "lambda",
        "->",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static DATABRICKS_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "notebook_start",
        r#"-- Databricks notebook source(\r?\n){1}"#,
        Token::comment_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "magic_line",
        r#"(-- MAGIC)( [^%]{1})([^\n]*)"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "magic_start",
        r#"(-- MAGIC %)([^\n]{2,})(\r?\n)"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::string_lexer(
        "start_hint",
        "/*+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "raw_single_quote",
        r#"[rR]'([^'\\]|\\.)*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "raw_double_quote",
        r#"[rR]"([^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bytes_single_quote",
        r#"X'([^'\\]|\\.)*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bytes_double_quote",
        r#"X"([^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "end_hint",
        "*/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`([^`]|``)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>(?>\d+\.\d+|\d+\.|\.\d+)([eE][+-]?\d+)?([dDfF]|BD|bd)?|\d+[eE][+-]?\d+([dDfF]|BD|bd)?|\d+([dDfFlLsSyY]|BD|bd)?)((?<=\.)|(?=\b))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "file_literal",
        r#"[a-zA-Z0-9]+:([a-zA-Z0-9\-_\.]*(\/|\\)){2,}((([a-zA-Z0-9\-_\.]*(:|\?|=|&)[a-zA-Z0-9\-_\.]*)+)|([a-zA-Z0-9\-_\.]*\.[a-z]+))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "command",
        r#"(\r?\n){2}-- COMMAND ----------(\r?\n)"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "equals",
        r#"==|<=>|="#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign_literal",
        r#"@\w*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static DB2_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'((?:[^']|'')*)'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""((?:[^"]|"")*)""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_#]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static DUCKDB_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*(?=\n|$)"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "unicode_single_quote",
        r#"(?si)U&'([^']|'')*'(\s*UESCAPE\s*'[^0-9A-Fa-f'+\-\s)]')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?si)E(('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "unicode_double_quote",
        r#"(?si)U&".+?"(\s*UESCAPE\s*\'[^0-9A-Fa-f\'+\-\s)]\')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "json_operator",
        r#"->>?|#>>?|@[>@?]|<@|\?[|&]?|#-"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "postgis_operator",
        r#"\&\&\&|\&<\||<<\||@|\|\&>|\|>>|\~=|<\->|\|=\||<\#>|<<\->>|<<\#>>"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "bit_string_literal",
        r#"[bBxX]'[0-9a-fA-F]*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "full_text_search_operator",
        "!!",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_divide",
        "//",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "meta_command",
        r#"\\(?!gset|gexec)([^\\\r\n])+((\\\\)|(?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "dollar_numeric_literal",
        r#"\$\d+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "meta_command_query_buffer",
        r#"\\([^\\\r\n])+((\\g(set|exec))|(?=\n)|(?=\r\n))?"#,
        Token::symbol_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[a-zA-Z_][0-9a-zA-Z_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static EXASOL_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"--[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "lua_nested_quotes",
        r#"\[={1,3}\[.*\]={1,3}\]"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "lua_multiline_quotes",
        r#"\[{2}([^[\\]|\\.)*\]{2}"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_identifier",
        r#"\[\w+\]"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "udf_param_dot_syntax",
        r#"\.{3}"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "range_operator",
        r#"\.{2}"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "hash",
        "#",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "function_script_terminator",
        r#"\n/\n|\n/$"#,
        Token::symbol_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"(\n|\r\n)+"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "at_sign_literal",
        r#"@[a-zA-Z_][\w]*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_literal",
        r#"[$][a-zA-Z0-9_.]*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static GREENPLUM_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*(?=\n|$)"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "unicode_single_quote",
        r#"(?si)U&'([^']|'')*'(\s*UESCAPE\s*'[^0-9A-Fa-f'+\-\s)]')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?si)E(('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "unicode_double_quote",
        r#"(?si)U&".+?"(\s*UESCAPE\s*\'[^0-9A-Fa-f\'+\-\s)]\')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "json_operator",
        r#"->>?|#>>?|@[>@?]|<@|\?[|&]?|#-"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "postgis_operator",
        r#"\&\&\&|\&<\||<<\||@|\|\&>|\|>>|\~=|<\->|\|=\||<\#>|<<\->>|<<\#>>"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "bit_string_literal",
        r#"[bBxX]'[0-9a-fA-F]*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "full_text_search_operator",
        "!!",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "meta_command",
        r#"\\(?!gset|gexec)([^\\\r\n])+((\\\\)|(?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "dollar_numeric_literal",
        r#"\$\d+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "meta_command_query_buffer",
        r#"\\([^\\\r\n])+((\\g(set|exec))|(?=\n)|(?=\r\n))?"#,
        Token::symbol_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[a-zA-Z_][0-9a-zA-Z_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static HIVE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static IMPALA_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static MARIADB_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(^--|-- |#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"(?s)('(?:\\'|''|\\\\|[^'])*'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#"(?s)("(?:\\"|""|\\\\|[^"])*"(?!"))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "hexadecimal_literal",
        r#"([xX]'([\da-fA-F][\da-fA-F])+'|0x[\da-fA-F]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bit_value_literal",
        r#"([bB]'[01]+'|0b[01]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "inline_path_operator",
        "->>",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "column_path_operator",
        "->",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_ampersand",
        "&&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_vertical_bar",
        "||",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign",
        r#"@@?[a-zA-Z0-9_$]*(\.[a-zA-Z0-9_$]+)?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static MATERIALIZE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*(?=\n|$)"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "unicode_single_quote",
        r#"(?si)U&'([^']|'')*'(\s*UESCAPE\s*'[^0-9A-Fa-f'+\-\s)]')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?si)E(('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "unicode_double_quote",
        r#"(?si)U&".+?"(\s*UESCAPE\s*\'[^0-9A-Fa-f\'+\-\s)]\')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "json_operator",
        r#"->>?|#>>?|@[>@?]|<@|\?[|&]?|#-"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "postgis_operator",
        r#"\&\&\&|\&<\||<<\||@|\|\&>|\|>>|\~=|<\->|\|=\||<\#>|<<\->>|<<\#>>"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "bit_string_literal",
        r#"[bBxX]'[0-9a-fA-F]*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "full_text_search_operator",
        "!!",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "meta_command",
        r#"\\(?!gset|gexec)([^\\\r\n])+((\\\\)|(?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "dollar_numeric_literal",
        r#"\$\d+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "meta_command_query_buffer",
        r#"\\([^\\\r\n])+((\\g(set|exec))|(?=\n)|(?=\r\n))?"#,
        Token::symbol_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[a-zA-Z_][0-9a-zA-Z_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static MYSQL_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(^--|-- |#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"(?s)('(?:\\'|''|\\\\|[^'])*'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#"(?s)("(?:\\"|""|\\\\|[^"])*"(?!"))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "hexadecimal_literal",
        r#"([xX]'([\da-fA-F][\da-fA-F])+'|0x[\da-fA-F]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bit_value_literal",
        r#"([bB]'[01]+'|0b[01]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "inline_path_operator",
        "->>",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "column_path_operator",
        "->",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_ampersand",
        "&&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_vertical_bar",
        "||",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign",
        r#"@@?[a-zA-Z0-9_$]*(\.[a-zA-Z0-9_$]+)?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static ORACLE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "prompt_command",
        r#"PROMPT([^(\r\n)])*((?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at_sign",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[a-zA-Z][0-9a-zA-Z_$#]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static POSTGRES_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*(?=\n|$)"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "unicode_single_quote",
        r#"(?si)U&'([^']|'')*'(\s*UESCAPE\s*'[^0-9A-Fa-f'+\-\s)]')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?si)E(('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "unicode_double_quote",
        r#"(?si)U&".+?"(\s*UESCAPE\s*\'[^0-9A-Fa-f\'+\-\s)]\')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "json_operator",
        r#"->>?|#>>?|@[>@?]|<@|\?[|&]?|#-"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "postgis_operator",
        r#"\&\&\&|\&<\||<<\||@|\|\&>|\|>>|\~=|<\->|\|=\||<\#>|<<\->>|<<\#>>"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "bit_string_literal",
        r#"[bBxX]'[0-9a-fA-F]*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "full_text_search_operator",
        "!!",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "meta_command",
        r#"\\(?!gset|gexec)([^\\\r\n])+((\\\\)|(?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "dollar_numeric_literal",
        r#"\$\d+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "meta_command_query_buffer",
        r#"\\([^\\\r\n])+((\\g(set|exec))|(?=\n)|(?=\r\n))?"#,
        Token::symbol_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[a-zA-Z_][0-9a-zA-Z_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static REDSHIFT_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*(?=\n|$)"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "unicode_single_quote",
        r#"(?si)U&'([^']|'')*'(\s*UESCAPE\s*'[^0-9A-Fa-f'+\-\s)]')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?si)E(('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "unicode_double_quote",
        r#"(?si)U&".+?"(\s*UESCAPE\s*\'[^0-9A-Fa-f\'+\-\s)]\')?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "json_operator",
        r#"->>?|#>>?|@[>@?]|<@|\?[|&]?|#-"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "postgis_operator",
        r#"\&\&\&|\&<\||<<\||@|\|\&>|\|>>|\~=|<\->|\|=\||<\#>|<<\->>|<<\#>>"#,
        Token::symbol_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "at",
        "@",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "bit_string_literal",
        r#"[bBxX]'[0-9a-fA-F]*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "full_text_search_operator",
        "!!",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "meta_command",
        r#"\\(?!gset|gexec)([^\\\r\n])+((\\\\)|(?=\n)|(?=\r\n))?"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "dollar_numeric_literal",
        r#"\$\d+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "meta_command_query_buffer",
        r#"\\([^\\\r\n])+((\\g(set|exec))|(?=\n)|(?=\r\n))?"#,
        Token::symbol_token,
        None,
        None,
        None,
        |input| input.starts_with(['\\']),
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"#?[0-9a-zA-Z_]+[0-9a-zA-Z_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static SNOWFLAKE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#|//)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::string_lexer(
        "parameter_assigner",
        "=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "function_assigner",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "stage_path",
        r#"(?:@[^\s;)]+|'@[^']+')"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "column_selector",
        r#"\$[0-9]+"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$\$.*\$\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "dollar_literal",
        r#"[$][a-zA-Z0-9_.]*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_dollar_sign",
        r#"[a-zA-Z_][a-zA-Z0-9_$]*\$[a-zA-Z0-9_$]*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "unquoted_file_path",
        r#"file://(?:[a-zA-Z]+:|/)+(?:[0-9a-zA-Z\\/_*?-]+)(?:\.[0-9a-zA-Z]+)?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "question_mark",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "exclude_bracket_open",
        "{-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "exclude_bracket_close",
        "-}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static SOQL_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "datetime_literal",
        r#"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|(\+|\-)[0-9]{2}:[0-9]{2})"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "date_literal",
        r#"[0-9]{4}-[0-9]{2}-[0-9]{2}"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static SPARKSQL_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::string_lexer(
        "start_hint",
        "/*+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "raw_single_quote",
        r#"[rR]'([^'\\]|\\.)*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "raw_double_quote",
        r#"[rR]"([^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bytes_single_quote",
        r#"X'([^'\\]|\\.)*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bytes_double_quote",
        r#"X"([^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "end_hint",
        "*/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`([^`]|``)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>(?>\d+\.\d+|\d+\.|\.\d+)([eE][+-]?\d+)?([dDfF]|BD|bd)?|\d+[eE][+-]?\d+([dDfF]|BD|bd)?|\d+([dDfFlLsSyY]|BD|bd)?)((?<=\.)|(?=\b))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "file_literal",
        r#"[a-zA-Z0-9]+:([a-zA-Z0-9\-_\.]*(\/|\\)){2,}((([a-zA-Z0-9\-_\.]*(:|\?|=|&)[a-zA-Z0-9\-_\.]*)+)|([a-zA-Z0-9\-_\.]*\.[a-z]+))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "equals",
        r#"==|<=>|="#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign_literal",
        r#"@\w*"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static SQLITE_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*(\*\/|\Z)"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`([^`]|``)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "inline_path_operator",
        "->>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "column_path_operator",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign_literal",
        r#"@[a-zA-Z0-9_]+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "colon_literal",
        r#":[a-zA-Z0-9_]+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "question_literal",
        r#"\?[0-9]+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_literal",
        r#"\$[a-zA-Z0-9_]+"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static STARROCKS_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(^--|-- |#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"(?s)('(?:\\'|''|\\\\|[^'])*'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#"(?s)("(?:\\"|""|\\\\|[^"])*"(?!"))"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "hexadecimal_literal",
        r#"([xX]'([\da-fA-F][\da-fA-F])+'|0x[\da-fA-F]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "bit_value_literal",
        r#"([bB]'[01]+'|0b[01]+)"#,
        Token::literal_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "walrus_operator",
        ":=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "inline_path_operator",
        "->>",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "column_path_operator",
        "->",
        Token::symbol_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_ampersand",
        "&&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "double_vertical_bar",
        "||",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "at_sign",
        r#"@@?[a-zA-Z0-9_$]*(\.[a-zA-Z0-9_$]+)?"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static TERADATA_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"([0-9]+(\.[0-9]*)?)"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static TRINO_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::string_lexer(
        "right_arrow",
        "->",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static TSQL_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"/\*(?>[^*/]+|\*(?!\/)|/[^*])*(?>(?R)(?>[^*/]+|\*(?!\/)|/[^*])*)*\*/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""(""|[^"\\]|\\.)*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "atsign",
        r#"[@][a-zA-Z0-9_]+"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "var_prefix",
        r#"[$][a-zA-Z0-9_]+"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "square_quote",
        r#"\[([^\[\]]*)*\]"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "single_quote_with_n",
        r#"N'([^']|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "hash_prefix",
        r#"[#][#]?[a-zA-Z0-9_]+"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "unquoted_relative_sql_file_path",
        r#"[.\w\\/#-]+\.[sS][qQ][lL]\b"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[0-9a-zA-Z_#@]+"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});
pub static VERTICA_LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| { vec![

    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "inline_comment",
        r#"(--|#)[^\n]*"#,
        Token::comment_token,
        None,
        None,
        None,
        |input| input.starts_with(['#','-']),
    ),

    LexMatcher::regex_lexer(
        "block_comment",
        r#"\/\*([^\*]|\*(?!\/))*\*\/"#,
        Token::comment_token,
        Some(Box::new(
    LexMatcher::regex_lexer(
        "newline",
        r#"\r\n|\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(Box::new(
    LexMatcher::regex_lexer(
        "whitespace",
        r#"[^\S\r\n]+"#,
        Token::whitespace_token,
        None,
        None,
        None,
        |_| true,
    ))),
        Some(extract_nested_block_comment),
        |input| input.starts_with("/"),
    ),

    LexMatcher::regex_lexer(
        "single_quote",
        r#"'([^'\\]|\\.|'')*'"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "double_quote",
        r#""([^"]|"")*""#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "back_quote",
        r#"`(?:[^`\\]|\\.)*`"#,
        Token::code_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "dollar_quote",
        r#"\$(\w*)\$(.*?)\$\1\$"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with("$"),
    ),

    LexMatcher::regex_lexer(
        "numeric_literal",
        r#"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))"#,
        Token::literal_token,
        None,
        None,
        None,
        |input| input.starts_with(['.','0','1','2','3','4','5','6','7','8','9']),
    ),

    LexMatcher::regex_lexer(
        "escaped_single_quote",
        r#"(?s)[eE](('')+?(?!')|'.*?((?<!\\)(?:\\\\)*(?<!')(?:'')*|(?<!\\)(?:\\\\)*\\(?<!')(?:'')*')'(?!'))"#,
        Token::code_token,
        None,
        None,
        None,
        |input| input.starts_with(['E']),
    ),

    LexMatcher::regex_lexer(
        "like_operator",
        r#"!?~~?\*?"#,
        Token::comparison_operator_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::regex_lexer(
        "newline",
        r#"\r?\n"#,
        Token::newline_token,
        None,
        None,
        None,
        |_| true,
    ),

    LexMatcher::string_lexer(
        "null_casting_operator",
        "::!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "casting_operator",
        "::",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "equals",
        "=",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "greater_than",
        ">",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "null_equals_operator",
        "<=>",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "less_than",
        "<",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "not",
        "!",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "dot",
        ".",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "comma",
        ",",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "plus",
        "+",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "minus",
        "-",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "integer_division",
        "//",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "divide",
        "/",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "percent",
        "%",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "question",
        "?",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "ampersand",
        "&",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "vertical_bar",
        "|",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "caret",
        "^",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "star",
        "*",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_bracket",
        "(",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_bracket",
        ")",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_square_bracket",
        "[",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_square_bracket",
        "]",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "start_curly_bracket",
        "{",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "end_curly_bracket",
        "}",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "colon",
        ":",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::string_lexer(
        "semicolon",
        ";",
        Token::code_token,
        None,
        None,
        
        
    ),

    LexMatcher::regex_lexer(
        "word",
        r#"[\p{L}_][\p{L}\p{N}_$]*"#,
        Token::word_token,
        None,
        None,
        None,
        |_| true,
    ),
]});

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Dialect {
    Ansi,
Athena,
Bigquery,
Clickhouse,
Databricks,
Db2,
Duckdb,
Exasol,
Greenplum,
Hive,
Impala,
Mariadb,
Materialize,
Mysql,
Oracle,
Postgres,
Redshift,
Snowflake,
Soql,
Sparksql,
Sqlite,
Starrocks,
Teradata,
Trino,
Tsql,
Vertica
}

impl FromStr for Dialect {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ansi" => Ok(Dialect::Ansi),
"athena" => Ok(Dialect::Athena),
"bigquery" => Ok(Dialect::Bigquery),
"clickhouse" => Ok(Dialect::Clickhouse),
"databricks" => Ok(Dialect::Databricks),
"db2" => Ok(Dialect::Db2),
"duckdb" => Ok(Dialect::Duckdb),
"exasol" => Ok(Dialect::Exasol),
"greenplum" => Ok(Dialect::Greenplum),
"hive" => Ok(Dialect::Hive),
"impala" => Ok(Dialect::Impala),
"mariadb" => Ok(Dialect::Mariadb),
"materialize" => Ok(Dialect::Materialize),
"mysql" => Ok(Dialect::Mysql),
"oracle" => Ok(Dialect::Oracle),
"postgres" => Ok(Dialect::Postgres),
"redshift" => Ok(Dialect::Redshift),
"snowflake" => Ok(Dialect::Snowflake),
"soql" => Ok(Dialect::Soql),
"sparksql" => Ok(Dialect::Sparksql),
"sqlite" => Ok(Dialect::Sqlite),
"starrocks" => Ok(Dialect::Starrocks),
"teradata" => Ok(Dialect::Teradata),
"trino" => Ok(Dialect::Trino),
"tsql" => Ok(Dialect::Tsql),
"vertica" => Ok(Dialect::Vertica),
            _ => Err(())
        }
    }
}

pub fn get_lexers(dialect: Dialect) -> &'static Vec<LexMatcher> {
    match dialect {
        Dialect::Ansi => &ANSI_LEXERS,
Dialect::Athena => &ATHENA_LEXERS,
Dialect::Bigquery => &BIGQUERY_LEXERS,
Dialect::Clickhouse => &CLICKHOUSE_LEXERS,
Dialect::Databricks => &DATABRICKS_LEXERS,
Dialect::Db2 => &DB2_LEXERS,
Dialect::Duckdb => &DUCKDB_LEXERS,
Dialect::Exasol => &EXASOL_LEXERS,
Dialect::Greenplum => &GREENPLUM_LEXERS,
Dialect::Hive => &HIVE_LEXERS,
Dialect::Impala => &IMPALA_LEXERS,
Dialect::Mariadb => &MARIADB_LEXERS,
Dialect::Materialize => &MATERIALIZE_LEXERS,
Dialect::Mysql => &MYSQL_LEXERS,
Dialect::Oracle => &ORACLE_LEXERS,
Dialect::Postgres => &POSTGRES_LEXERS,
Dialect::Redshift => &REDSHIFT_LEXERS,
Dialect::Snowflake => &SNOWFLAKE_LEXERS,
Dialect::Soql => &SOQL_LEXERS,
Dialect::Sparksql => &SPARKSQL_LEXERS,
Dialect::Sqlite => &SQLITE_LEXERS,
Dialect::Starrocks => &STARROCKS_LEXERS,
Dialect::Teradata => &TERADATA_LEXERS,
Dialect::Trino => &TRINO_LEXERS,
Dialect::Tsql => &TSQL_LEXERS,
Dialect::Vertica => &VERTICA_LEXERS
    }
}

