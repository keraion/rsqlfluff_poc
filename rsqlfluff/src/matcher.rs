use std::fmt::Display;

use fancy_regex::{Regex, RegexBuilder};
use once_cell::sync::Lazy;

use crate::{marker::PositionMarker, token::Token};

#[derive(Debug, Clone)]
pub enum LexerMode {
    String(String), // Match a literal string
    Regex(Regex),   // Match using a regex
}

impl Display for LexerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LexerMode::Regex(_) => write!(f, "RegexMatcher"),
            LexerMode::String(_) => write!(f, "StringMatcher"),
        }
    }
}

pub struct LexedElement<'a> {
    pub raw: &'a str,
    pub matcher: &'a LexMatcher,
}

impl<'a> LexedElement<'a> {
    pub fn new(raw: &'a str, matcher: &'a LexMatcher) -> Self {
        Self { raw, matcher }
    }
}

#[derive(Debug, Clone)]
pub struct LexMatcher {
    pub name: String,
    pub mode: LexerMode,
    pub token_class_func: fn(String, PositionMarker) -> Token,
    pub subdivider: Option<Box<LexMatcher>>,
    pub trim_post_subdivide: Option<Box<LexMatcher>>,
}

impl Display for LexMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}: {}>", self.mode, self.name)
    }
}

impl LexMatcher {
    pub fn string_lexer(
        name: &str,
        template: &str,
        token_class_func: fn(String, PositionMarker) -> Token,
        subdivider: Option<Box<LexMatcher>>,
        trim_post_subdivide: Option<Box<LexMatcher>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            mode: LexerMode::String(template.to_string()),
            token_class_func,
            subdivider,
            trim_post_subdivide,
        }
    }

    pub fn regex_lexer(
        name: &str,
        template: &str,
        token_class_func: fn(String, PositionMarker) -> Token,
        subdivider: Option<Box<LexMatcher>>,
        trim_post_subdivide: Option<Box<LexMatcher>>,
    ) -> Self {
        let pattern = format!(r"(?s)\A(?:{})", template);
        // let pattern = format!(r"(?s){}", template);
        Self {
            name: name.to_string(),
            mode: LexerMode::Regex(
                RegexBuilder::new(&pattern)
                    .build()
                    .expect("Failed to compile regex"),
            ),
            token_class_func,
            subdivider,
            trim_post_subdivide,
        }
    }

    pub fn regex_subdivider(
        name: &str,
        template: &str,
        token_class_func: fn(String, PositionMarker) -> Token,
        subdivider: Option<Box<LexMatcher>>,
        trim_post_subdivide: Option<Box<LexMatcher>>,
    ) -> Self {
        let pattern = format!(r"(?:{})", template);
        // let pattern = format!(r"(?s){}", template);
        Self {
            name: name.to_string(),
            mode: LexerMode::Regex(
                RegexBuilder::new(&pattern)
                    .build()
                    .expect("Failed to compile regex"),
            ),
            token_class_func,
            subdivider,
            trim_post_subdivide,
        }
    }

    pub fn scan_match<'a>(&'a self, input: &'a str) -> Option<Vec<LexedElement<'a>>> {
        // let start = Instant::now();
        if input.is_empty() {
            panic!("Unexpected empty string!");
        }

        // Match based on the mode
        let matched = match &self.mode {
            LexerMode::String(template) => input
                .starts_with(template)
                .then(|| LexedElement::new(template, self)),
            LexerMode::Regex(regex) => regex
                .find(input)
                .ok()
                .flatten()
                .map(|mat| LexedElement::new(mat.as_str(), self)),
        };

        // Handle subdivision and trimming
        if let Some(matched) = matched {
            let elements = self.subdivide(matched);
            Some(elements)
        } else {
            None
        }
    }

    fn search(&self, input: &str) -> Option<(usize, usize)> {
        match &self.mode {
            LexerMode::String(template) => input.find(template).map(|start| {
                let end = start + template.len();
                (start, end)
            }),
            LexerMode::Regex(regex) => regex
                .find(input)
                .ok()
                .flatten()
                .map(|mat| (mat.start(), mat.end())),
        }
    }

    fn subdivide<'a>(&'a self, matched: LexedElement<'a>) -> Vec<LexedElement<'a>> {
        if let Some(subdivider) = &self.subdivider {
            let mut elements = Vec::new();
            let mut buffer = matched.raw;
            while !buffer.is_empty() {
                if let Some((start, end)) = subdivider.search(buffer) {
                    let trimmed_elems = self.trim_match(&buffer[..start]);
                    elements.extend(trimmed_elems);
                    elements.push(LexedElement {
                        raw: &buffer[start..end],
                        matcher: subdivider,
                    });
                    buffer = &buffer[end..];
                } else {
                    let trimmed_elems = self.trim_match(&buffer);
                    elements.extend(trimmed_elems);
                    break;
                }
            }
            elements
        } else {
            vec![matched]
        }
    }

    fn trim_match<'a>(&'a self, raw: &'a str) -> Vec<LexedElement<'a>> {
        if let Some(trim_post_subdivide) = &self.trim_post_subdivide {
            let mut elements = Vec::new();
            let mut buffer = raw;
            let mut content_buffer = 0..0;
            while !buffer.is_empty() {
                if let Some((start, end)) = trim_post_subdivide.search(buffer) {
                    if start == 0 {
                        // Starting match
                        elements.push(LexedElement {
                            raw: &buffer[..end],
                            matcher: trim_post_subdivide,
                        });
                        buffer = &buffer[end..];
                        content_buffer.start = end;
                    } else if end == buffer.len() {
                        elements.push(LexedElement {
                            raw: &raw[content_buffer.start..content_buffer.end + start],
                            matcher: self,
                        });
                        elements.push(LexedElement {
                            raw: &buffer[start..end],
                            matcher: trim_post_subdivide,
                        });
                        return elements;
                    } else {
                        content_buffer.end += end;
                        buffer = &buffer[end..];
                    }
                } else {
                    break;
                }
            }
            if !content_buffer.is_empty() || !buffer.is_empty() {
                elements.push(LexedElement {
                    raw: &raw[content_buffer.start..],
                    matcher: self,
                });
            }
            elements
        } else {
            vec![LexedElement {
                raw: raw,
                matcher: self,
            }]
        }
    }

    /*

    def construct_segment(self, raw: str, pos_marker: PositionMarker) -> RawSegment:
        """Construct a segment using the given class a properties.

        Unless an override `type` is provided in the `segment_kwargs`,
        it is assumed that the `name` of the lexer is designated as the
        intended `type` of the segment.
        """
        # NOTE: Using a private attribute here feels a bit wrong.
        _segment_class_types = self.segment_class._class_types
        _kwargs = self.segment_kwargs
        assert not (
            "type" in _kwargs and "instance_types" in _kwargs
        ), f"Cannot set both `type` and `instance_types` in segment kwargs: {_kwargs}"
        if "type" in _kwargs:
            # TODO: At some point we should probably deprecate this API and only
            # allow setting `instance_types`.
            assert _kwargs["type"]
            _kwargs["instance_types"] = (_kwargs.pop("type"),)
        elif "instance_types" not in _kwargs and self.name not in _segment_class_types:
            _kwargs["instance_types"] = (self.name,)
        return self.segment_class(raw=raw, pos_marker=pos_marker, **_kwargs)

     */

    pub fn construct_token(&self, raw: &str, pos_marker: PositionMarker) -> Token {
        // let mut segment_class_types = self.token_class_func.instance_types.clone();
        // let mut segment_class_types = self.token_class_func.clone();
        // let matcher_instance_types = self.instance_types.clone();

        // let instance_types = if matcher_instance_types.is_some() {
        //     matcher_instance_types.unwrap()
        // } else if !segment_class_types.contains(&self.name.to_string()) {
        //     let seg_name = self.name.to_string();
        //     segment_class_types.push(seg_name);
        //     segment_class_types
        // } else {
        //     segment_class_types
        // };

        (self.token_class_func)(raw.to_string(), pos_marker)
    }
}

pub static LEXERS: Lazy<Vec<LexMatcher>> = Lazy::new(|| {
    vec![
        LexMatcher::regex_lexer(
            "whitespace",
            r"[^\S\r\n]+",
            Token::whitespace_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "inline_comment",
            r"(?:--|#)[^\n]*",
            Token::comment_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "block_comment",
            r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/",
            Token::comment_token,
            Some(Box::new(LexMatcher::regex_subdivider(
                "newline",
                r"\r?\n",
                Token::newline_token,
                None,
                None,
            ))),
            Some(Box::new(LexMatcher::regex_subdivider(
                "whitespace",
                r"[^\S\r\n]+",
                Token::whitespace_token,
                None,
                None,
            ))),
        ),
        LexMatcher::regex_lexer(
            "single_quote",
            r"'(?:[^'\\]|\\.|'')*'",
            Token::code_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "double_quote",
            r#""(?:""|[^"\\]|\\.)*""#,
            Token::code_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "back_quote",
            r"`(?:[^`\\]|\\.)*`",
            Token::code_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "dollar_quote",
            r"\$(\w*)\$.*?\$\1\$",
            Token::code_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "numeric_literal",
            // r"(?>\d+\.\d+|\d+\.(?![\.\w])|\.\d+|\d+)(\.?[eE][+-]?\d+)?((?<=\.)|(?=\b))",
            r"\d+\.\d+|\d+\.(?!\.)|\.\d+|\d+(?:[eE][+-]?\d+)?\b",
            Token::literal_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer(
            "like_operator",
            r"!?~~?\*?",
            Token::comparison_operator_token,
            None,
            None,
        ),
        LexMatcher::regex_lexer("newline", r"\r?\n", Token::newline_token, None, None),
        LexMatcher::string_lexer("casting_operator", "::", Token::code_token, None, None),
        LexMatcher::string_lexer("equals", "=", Token::code_token, None, None),
        LexMatcher::string_lexer("greater_than", ">", Token::code_token, None, None),
        LexMatcher::string_lexer("less_than", "<", Token::code_token, None, None),
        LexMatcher::string_lexer("not", "!", Token::code_token, None, None),
        LexMatcher::string_lexer("dot", ".", Token::code_token, None, None),
        LexMatcher::string_lexer("comma", ",", Token::code_token, None, None),
        LexMatcher::string_lexer("plus", "+", Token::code_token, None, None),
        LexMatcher::string_lexer("minus", "-", Token::code_token, None, None),
        LexMatcher::string_lexer("divide", "/", Token::code_token, None, None),
        LexMatcher::string_lexer("percent", "%", Token::code_token, None, None),
        LexMatcher::string_lexer("question", "?", Token::code_token, None, None),
        LexMatcher::string_lexer("ampersand", "&", Token::code_token, None, None),
        LexMatcher::string_lexer("vertical_bar", "|", Token::code_token, None, None),
        LexMatcher::string_lexer("caret", "^", Token::code_token, None, None),
        LexMatcher::string_lexer("star", "*", Token::code_token, None, None),
        LexMatcher::string_lexer("start_bracket", "(", Token::code_token, None, None),
        LexMatcher::string_lexer("end_bracket", ")", Token::code_token, None, None),
        LexMatcher::string_lexer("start_square_bracket", "[", Token::code_token, None, None),
        LexMatcher::string_lexer("end_square_bracket", "]", Token::code_token, None, None),
        LexMatcher::string_lexer("start_curly_bracket", "{", Token::code_token, None, None),
        LexMatcher::string_lexer("end_curly_bracket", "}", Token::code_token, None, None),
        LexMatcher::string_lexer("colon", ":", Token::code_token, None, None),
        LexMatcher::string_lexer("semicolon", ";", Token::code_token, None, None),
        // This is the "fallback" lexer for anything else which looks like SQL.
        // LexMatcher::regex_lexer("word", r"[0-9a-zA-Z_]+", Token::word_token),
        LexMatcher::regex_lexer(
            "word",
            r"[\p{L}_][\p{L}\p{N}_$]*\b",
            Token::word_token,
            None,
            None,
        ),
    ]
});

pub fn ansi_lexers() -> &'static Vec<LexMatcher> {
    &LEXERS
}
