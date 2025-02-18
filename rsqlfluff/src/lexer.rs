use std::{cmp::min, ops::Range, time::Instant};

use fancy_regex::{Regex, RegexBuilder};
use once_cell::sync::Lazy;
use pyo3::{pyclass, pymethods};

use crate::{
    marker::PositionMarker,
    templater::{RawFileSlice, TemplatedFile, TemplatedFileSlice},
    token::Token,
};

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Bound, RangeBounds},
};

use uuid::Uuid;

use itertools::multipeek;

pub struct BlockTracker {
    stack: Vec<Uuid>,
    map: HashMap<Range<usize>, Uuid>,
}

impl BlockTracker {
    /// Create a new `BlockTracker`.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Enter a block given a source slice (start, end).
    pub fn enter(&mut self, src_slice: Range<usize>) {
        let uuid = self
            .map
            .entry(src_slice.clone())
            .or_insert_with(Uuid::new_v4)
            .clone();

        log::debug!(
            "Entering block stack @ {:?}: {} ({})",
            src_slice,
            uuid,
            if self.map.contains_key(&src_slice) {
                "cached"
            } else {
                "fresh"
            }
        );

        self.stack.push(uuid);
    }

    /// Exit the current block, removing it from the stack.
    pub fn exit(&mut self) {
        if let Some(uuid) = self.stack.pop() {
            log::debug!("Exiting block stack: {}", uuid);
        } else {
            log::warn!("Attempted to exit an empty block stack!");
        }
    }

    /// Get the `Uuid` on top of the stack.
    ///
    /// # Panics
    /// This method panics if the stack is empty.
    pub fn top(&self) -> Uuid {
        *self
            .stack
            .last()
            .expect("Block stack is empty. Cannot get the top block.")
    }

    /// Check if the block stack is empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Get the size of the stack.
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Debug)]
pub struct TemplateElement {
    pub raw: String,
    pub template_slice: Range<usize>, // Slice equivalent
    pub matcher: LexMatcher,          // Reference to the lexer that matched this element
}

impl Display for TemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TemplatedElement(raw='{}', template_slice={:?}, matcher={})",
            self.raw, self.template_slice, self.matcher
        )
    }
}

impl TemplateElement {
    // Constructor to create a TemplateElement from a LexedElement and a template slice
    pub fn from_element(element: &LexedElement, template_slice: Range<usize>) -> Self {
        Self {
            raw: element.raw.to_string(),
            template_slice,
            matcher: element.matcher.clone(),
        }
    }

    // Method to convert the TemplateElement to a RawSegment
    pub fn to_token<R>(&self, pos_marker: PositionMarker, subslice: Option<R>) -> Token
    where
        R: RangeBounds<usize>,
    {
        let raw_subslice = match subslice {
            Some(range) => {
                let start = match range.start_bound() {
                    Bound::Included(&start) => start,
                    Bound::Excluded(&start) => start + 1,
                    Bound::Unbounded => 0,
                };

                let end = match range.end_bound() {
                    Bound::Included(&end) => end + 1,
                    Bound::Excluded(&end) => end,
                    Bound::Unbounded => self.raw.len(),
                };

                &self.raw[start..end]
            }
            None => &self.raw,
        };
        self.matcher.construct_token(raw_subslice, pos_marker)
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

pub struct LexMatch<'a> {
    forward_string: String,
    elements: Vec<LexedElement<'a>>,
}

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

pub fn lex_string<'a>(
    mut input: &'a str,
    lexer_matchers: &'a [LexMatcher],
    last_resort_lexer: &'a LexMatcher,
) -> Vec<LexedElement<'a>> {
    let mut element_buffer: Vec<LexedElement> = Vec::with_capacity(input.len());

    while !input.is_empty() {
        if let Some(elements) =
            lex_match(input, lexer_matchers).or_else(|| last_resort_lexer.scan_match(input))
        {
            let match_length = elements.iter().fold(0, |acc, e| acc + e.raw.len());
            element_buffer.extend(elements);
            input = &input[match_length..];
        } else {
            panic!(
                "Fatal. Unable to lex characters: {}",
                &input[..input.chars().take(10).count()]
            );
        };
    }

    element_buffer
}

pub fn lex_match<'a>(
    input: &'a str,
    lexer_matchers: &'a [LexMatcher],
) -> Option<Vec<LexedElement<'a>>> {
    lexer_matchers
        .iter()
        .find_map(|matcher| matcher.scan_match(input))
}

#[pyclass]
#[derive(Debug)]
pub struct SQLLexError {
    msg: String,
    pos_marker: PositionMarker,
}

#[pymethods]
impl SQLLexError {
    #[new]
    fn new(msg: String, pos_marker: PositionMarker) -> Self {
        Self { msg, pos_marker }
    }
}

pub fn map_template_slices(
    elements: &[LexedElement],
    template: &TemplatedFile,
) -> Vec<TemplateElement> {
    let mut idx = 0;
    let mut templated_buff = Vec::new();

    for element in elements {
        let element_len = element.raw.len();
        let template_slice = idx..idx + element_len;
        idx += element_len;

        // Create a TemplateElement from the LexedElement and the template slice
        let templated_element = TemplateElement::from_element(element, template_slice.clone());
        templated_buff.push(templated_element);

        // Validate that the slice matches the element's raw content
        if template.templated_str[template_slice.clone()] != *element.raw {
            panic!(
                "Template and lexed elements do not match. This should never happen  {:?} != {:?}",
                &element.raw, &template.templated_str[template_slice]
            )
        }
    }

    templated_buff
}

fn elements_to_tokens(
    elements: &[TemplateElement],
    templated_file: &TemplatedFile,
    template_blocks_indent: bool,
) -> Vec<Token> {
    log::info!("Elements to Segments.");

    // Convert elements into segments using an iterator
    let mut segment_buffer: Vec<Token> =
        iter_tokens(elements, templated_file, template_blocks_indent);

    // Add an EndOfFile marker
    let eof_marker = if let Some(last_segment) = segment_buffer.last() {
        Token::end_of_file_token(last_segment.pos_marker.end_point_marker(), false, None)
    } else {
        Token::end_of_file_token(
            PositionMarker::from_point(0, 0, templated_file, None, None),
            false,
            None,
        )
    };
    segment_buffer.push(eof_marker);

    segment_buffer
}

fn iter_tokens(
    lexed_elements: &[TemplateElement],
    templated_file: &TemplatedFile,
    add_indents: bool,
) -> Vec<Token> {
    let mut tfs_idx = 0;
    let mut block_stack = BlockTracker::new();
    let mut templated_file_slices = multipeek(templated_file.sliced_file.iter().peekable());

    lexed_elements
        .iter()
        .enumerate()
        .flat_map(|(idx, element)| -> std::vec::IntoIter<Token> {
            log::debug!("  {}: {}. [tfs_idx = {}]", idx, element, tfs_idx);
            let mut consumed_length = 0;
            let mut stashed_source_idx = None;
            let mut segments = Vec::new();

            // TODO: uhh this seems wrong, check when I have an example templated file
            while let Some(tfs) = templated_file_slices.clone().peek() {
                log::debug!("      {}: {:?}", tfs_idx, tfs);

                if tfs.templated_slice.len() == 0 {
                    let next_tfs = templated_file_slices.peek();
                    segments.extend(handle_zero_length_slice(
                        tfs,
                        next_tfs,
                        &mut block_stack,
                        templated_file,
                        add_indents,
                    ));
                    continue;
                }

                match tfs.slice_type.as_str() {
                    "literal" => {
                        let tfs_offset = tfs.source_slice.start - tfs.templated_slice.start;

                        if element.template_slice.end <= tfs.templated_slice.end {
                            // Consume the whole element within this slice
                            let slice_start = stashed_source_idx.unwrap_or_else(|| {
                                element.template_slice.start + consumed_length + tfs_offset
                            });

                            segments.push(element.to_token(
                                PositionMarker::new(
                                    slice_start..(element.template_slice.end + tfs_offset),
                                    element.template_slice.clone(),
                                    &templated_file,
                                    None,
                                    None,
                                ),
                                Some(consumed_length..),
                            ));

                            // Move to the next templated slice if it's an exact match
                            if element.template_slice.end == tfs.templated_slice.end {
                                tfs_idx += 1;
                            }
                            break;
                        } else {
                            // Handle spilling over slices
                            let incremental_length =
                                tfs.templated_slice.end - element.template_slice.start;
                            segments.push(element.to_token(
                                PositionMarker::new(
                                    (element.template_slice.start + consumed_length + tfs_offset)
                                        ..tfs.templated_slice.end + tfs_offset,
                                    element.template_slice.clone(),
                                    &templated_file,
                                    None,
                                    None,
                                ),
                                Some(consumed_length..(consumed_length + incremental_length)),
                            ));
                            consumed_length += incremental_length;
                            templated_file_slices.next();
                        }
                    }
                    "templated" | "block_start" | "escaped" => {
                        if !is_zero_slice(&tfs.templated_slice) {
                            if tfs.slice_type == "block_start" {
                                block_stack.enter(tfs.source_slice.clone());
                            }

                            if element.template_slice.end <= tfs.templated_slice.end {
                                let slice_start =
                                    stashed_source_idx.unwrap_or(tfs.source_slice.start);
                                segments.push(element.to_token(
                                    PositionMarker::new(
                                        slice_start..tfs.source_slice.end,
                                        element.template_slice.clone(),
                                        &templated_file,
                                        None,
                                        None,
                                    ),
                                    Some(consumed_length..),
                                ));

                                if element.template_slice.end == tfs.templated_slice.end {
                                    tfs_idx += 1;
                                }
                                break;
                            } else {
                                stashed_source_idx = Some(tfs.source_slice.start);
                                templated_file_slices.next();
                            }
                        } else {
                            // Handle zero-length slices separately
                            // segments.extend(handle_zero_length_slice(
                            //     tfs,
                            //     templated_file_slices_iter.clone().next(),
                            //     &mut block_stack,
                            //     &templated_file,
                            //     add_indents,
                            // ));
                        }
                    }
                    _ => panic!("Unexpected slice type: {}", tfs.slice_type),
                }
            }

            // TODO: finalize other zero length templated area

            segments.into_iter()
        })
        .collect::<Vec<_>>()
}

fn handle_zero_length_slice(
    tfs: &TemplatedFileSlice,
    next_tfs: Option<&&TemplatedFileSlice>,
    block_stack: &mut BlockTracker,
    templated_file: &TemplatedFile,
    add_indents: bool,
) -> impl Iterator<Item = Token> {
    let mut segments = Vec::new();
    assert!(is_zero_slice(&tfs.templated_slice));

    if next_tfs.is_some() && tfs.slice_type.starts_with("block") {
        let peek = next_tfs.unwrap();
        if peek.source_slice.start < tfs.source_slice.start {
            log::debug!("      Backward jump detected. Inserting Loop Marker");
            let pos_marker = PositionMarker::from_point(
                tfs.source_slice.start,
                tfs.templated_slice.start,
                templated_file,
                None,
                None,
            );

            if add_indents {
                segments.push(Token::dedent_token(pos_marker.clone(), true, None))
            }

            //TODO: TemplateLoop
            segments.push(Token::template_loop_token(
                pos_marker.clone(),
                Some(block_stack.top()),
            ));

            if add_indents {
                segments.push(Token::indent_token(pos_marker.clone(), true, None))
            }
            return segments.into_iter();
        }
    }

    // Then handle blocks (which aren't jumps backwards)
    if tfs.slice_type.starts_with("block") {
        // It's a block. Yield a placeholder with potential indents.

        // Update block stack or add indents
        if tfs.slice_type == "block_start" {
            block_stack.enter(tfs.source_slice.clone());
        } else if add_indents && (tfs.slice_type == "block_end" || tfs.slice_type == "block_mid") {
            let pos_marker = PositionMarker::from_point(
                tfs.source_slice.start,
                tfs.templated_slice.start,
                templated_file,
                None,
                None,
            );
            segments.push(Token::dedent_token(
                pos_marker,
                true,
                Some(block_stack.top()),
            ))
        }

        // TemplateSegment from_slice
        segments.push(Token::template_placeholder_token_from_slice(
            tfs.source_slice.clone(),
            tfs.templated_slice.clone(),
            tfs.slice_type.clone(),
            templated_file,
            Some(block_stack.top()),
        ));

        if tfs.slice_type == "block_end" {
            block_stack.exit();
        } else if add_indents && (tfs.slice_type == "block_start" || tfs.slice_type == "block_mid")
        {
            let pos_marker = PositionMarker::from_point(
                tfs.source_slice.end,
                tfs.templated_slice.end,
                templated_file,
                None,
                None,
            );
            segments.push(Token::indent_token(
                pos_marker,
                true,
                Some(block_stack.top()),
            ));
        }

        // Before we move on, we might have a _forward_ jump to the next
        // element. That element can handle itself, but we'll add a
        // placeholder for it here before we move on.
        if next_tfs.is_some() && next_tfs.unwrap().source_slice.start > tfs.source_slice.end {
            let placeholder_str = templated_file.source_str
                [tfs.source_slice.end..next_tfs.unwrap().source_slice.start]
                .to_string();
            log::debug!("Forward jump detected. Inserting placeholder");
            let pos_marker = PositionMarker::new(
                tfs.source_slice.end..next_tfs.unwrap().source_slice.start,
                tfs.templated_slice.clone(),
                templated_file,
                None,
                None,
            );
            segments.push(Token::template_placeholder_token(
                pos_marker,
                placeholder_str,
                "skipped_source".to_string(),
                None,
            ));
        }

        return segments.into_iter();
    }

    // Always return the slice, even if the source slice was also zero length.  Some
    // templaters might want to pass through totally zero length slices as a way of
    // marking locations in the middle of templated output.
    segments.push(Token::template_placeholder_token_from_slice(
        tfs.source_slice.clone(),
        tfs.templated_slice.clone(),
        tfs.slice_type.clone(),
        templated_file,
        None,
    ));

    segments.into_iter()
}

pub fn is_zero_slice(s: &Range<usize>) -> bool {
    s.start == s.end
}

#[pyclass]
pub enum LexInput {
    String(String),
    TemplatedFile(TemplatedFile),
}

pub fn lex(raw: LexInput, template_blocks_indent: bool) -> (Vec<Token>, Vec<SQLLexError>) {
    let (template, str_buff) = match raw {
        LexInput::String(raw_str) => {
            let template = TemplatedFile::from_string(raw_str.clone());
            (template, raw_str)
        }
        LexInput::TemplatedFile(template_file) => {
            let str_buff = template_file.to_string();
            (template_file, str_buff)
        }
    };

    // TODO: handle more matchers
    let matcher = ansi_lexers();
    let last_resort_lexer = LexMatcher::regex_lexer(
        "<unlexable>",
        r#"[^\t\n\ ]*"#,
        Token::unlexable_token,
        None,
        None,
    );

    let lexed_elements = lex_string(&str_buff, &matcher, &last_resort_lexer);
    let templated_buffer = map_template_slices(&lexed_elements, &template);
    let tokens = elements_to_tokens(&templated_buffer, &template, template_blocks_indent);

    let violations = violations_from_tokens(&tokens);
    (tokens, violations)
}

fn violations_from_tokens(tokens: &[Token]) -> Vec<SQLLexError> {
    tokens
        .iter()
        .filter(|t| t.token_type.as_ref().is_some_and(|tt| tt == "unlexable"))
        .map(|token| {
            SQLLexError::new(
                format!(
                    "Unable to lex characters: {}",
                    token.raw.chars().take(10).collect::<String>()
                ),
                token.pos_marker.clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::Instant;

    fn init() {
        let _ = env_logger::builder()
            .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_parser_lexer_obj() {
        let test_cases = vec![
            // NOTE: The final empty string is the end of file marker
            ("a b", vec!["a", " ", "b", ""]),
            ("b.c", vec!["b", ".", "c", ""]),
            (
                "abc \n \t def  ;blah",
                vec!["abc", " ", "\n", " \t ", "def", "  ", ";", "blah", ""],
            ),
            // Test Quotes
            (
                "abc\'\n \"\t\' \"de`f\"",
                vec!["abc", "'\n \"\t'", " ", "\"de`f\"", ""],
            ),
            // Test Comments
            (
                "abc -- comment \nblah",
                vec!["abc", " ", "-- comment ", "\n", "blah", ""],
            ),
            (
                "abc # comment \nblah",
                vec!["abc", " ", "# comment ", "\n", "blah", ""],
            ),
            // Note the more complicated parsing of block comments.
            // This tests subdivision and trimming (incl the empty case)
            (
                "abc /* comment \nblah*/",
                vec!["abc", " ", "/* comment", " ", "\n", "blah*/", ""],
            ),
            (
                "abc /*\n\t\n*/",
                vec!["abc", " ", "/*", "\n", "\t", "\n", "*/", ""],
            ),
            // Test strings
            ("*-+bd/", vec!["*", "-", "+", "bd", "/", ""]),
            // Test Negatives and Minus
            ("2+4 -5", vec!["2", "+", "4", " ", "-", "5", ""]),
            (
                "when 'Spec\\'s 23' like",
                vec!["when", " ", "'Spec\\'s 23'", " ", "like", ""],
            ),
            (
                "when \"Spec\\\"s 23\" like",
                vec!["when", " ", "\"Spec\\\"s 23\"", " ", "like", ""],
            ),
        ];

        for (raw, res) in test_cases {
            let (tokens, _) = lex(LexInput::String(raw.to_string()), true);
            for token in &tokens {
                println!("{:?}", token);
            }
            assert_eq!(tokens.iter().map(|t| &t.raw).collect::<Vec<_>>(), res)
        }
    }

    #[test]
    fn test_unlexable_lex() {
        let raw = LexInput::String(
            r#"SELECT 1 
        FROM table_2 WHERE a / b = 3   "  ;"#
                .to_string(),
        );
        let (_tokens, violations) = lex(raw, true);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].msg, "Unable to lex characters: \"");
        assert_eq!(violations[0].pos_marker.line_no(), 2);
        assert_eq!(violations[0].pos_marker.line_pos(), 40);
    }

    #[test]
    fn test_scan() {
        let raw = LexInput::String(
            r#"SELECT 1 
        FROM table_2 WHERE a / b = 3 || "hahaha"     ;"#
                .to_string(),
        );
        let (template, str_buff) = match raw {
            LexInput::String(raw_str) => {
                let template = TemplatedFile::from_string(raw_str.clone());
                (template, raw_str)
            }
            LexInput::TemplatedFile(template_file) => {
                let str_buff = template_file.to_string();
                (template_file, str_buff)
            }
        };

        let matcher = ansi_lexers();
        let last_resort_lexer = LexMatcher::regex_lexer(
            "<unlexable>",
            r#"[^\t\n\ ]*"#,
            Token::unlexable_token,
            None,
            None,
        );
        let test_case = lex_string(
            r#"SELECT 1 
        FROM table_2 WHERE a / b = 3 || "hahaha"     ;"#,
            &matcher,
            &last_resort_lexer,
        );
        // for element in test_case {
        //     println!(r#"{} <"{}">"#, element.matcher.name, element.raw);
        // }
        let templated_buffer = map_template_slices(&test_case, &template);
        // for e in x {
        //     println!(
        //         "{} : {} : {} - {}",
        //         e.matcher.name, e.raw, e.template_slice.start, e.template_slice.end
        //     )
        // }
        let tokens = elements_to_tokens(&templated_buffer, &template, false);
        for token in tokens {
            println!(
                "{}: {} : '{}'",
                token.pos_marker.to_source_string(),
                token.token_type.unwrap_or("None".to_string()),
                token.raw.escape_debug(),
            )
        }
    }

    #[test]
    fn test_scan_broken_quotes() {
        env_logger::init();
        let matcher = ansi_lexers();
        let last_resort_lexer = LexMatcher::regex_lexer(
            "<unlexable>",
            r"[^\t\n\ ]*",
            Token::unlexable_token,
            None,
            None,
        );
        let test_case = lex_string(
            r#"SELECT 1 
        FROM table_2 WHERE a / b = 3   "  ;"#,
            &matcher,
            &last_resort_lexer,
        );
        for element in test_case {
            println!(r#"{} <"{}">"#, element.matcher.name, element.raw);
        }
    }

    #[test]
    fn test_scan_utf8() {
        init();

        let raw = LexInput::String(
            r#"SELECT amount+1 AS 'amount' FROM num1;

SELECT höhe+1 AS 'höhe' FROM num1;


SELECT amount*2 AS 'amount' FROM num1;

SELECT höhe*2 AS 'höhe' FROM num1;


SELECT employees.personal.name, neighbors.area FROM neighbors, employees
WHERE employees.personal.address.zipcode=neighbors.area.zipcode AND neighbors.num_neighbors > 1;

SELECT mitarbeiter.persönlicher.name, nachbarn.bereich FROM nachbarn, mitarbeiter
WHERE mitarbeiter.persönlicher.adresse.zipcode=nachbarn.gebiet.zipcode AND nachbarn.nummer_nachbarn > 1;


SELECT itemkey AS key, IMPLODE(itemprice) WITHIN GROUP (ORDER BY itemprice) AS prices
    FROM filtered GROUP BY itemkey ORDER BY itemkey;

SELECT ключтовара AS key, IMPLODE(ценатовара) WITHIN GROUP (ORDER BY ценатовара) AS цены
    FROM отфильтровано GROUP BY ключтовара ORDER BY ключтовара;


SELECT State, APPROXIMATE_PERCENTILE(sales USING PARAMETERS percentiles='0.5') AS median
FROM allsales GROUP BY state;

SELECT Χώρα, APPROXIMATE_PERCENTILE(πωλήσεις USING PARAMETERS percentiles='0.5') AS διάμεσος
FROM όλεςτιςπωλήσεις GROUP BY χώρα;


SELECT customer_state, customer_key, annual_income, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_CONT
   FROM customer_dimension WHERE customer_state IN ('DC','WI') ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY revenu_annuel)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_CONT
   FROM dimension_client WHERE état_du_client IN ('Provence','Сhampagne') ORDER BY état_du_client, clé_client;


SELECT customer_state, customer_key, annual_income,
      PERCENTILE_DISC(.2) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_DISC
   FROM customer_dimension
   WHERE customer_state IN ('DC','WI')
   AND customer_key < 300
   ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel,
      PERCENTILE_DISC(.2) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_DISC
   FROM dimension_client
   WHERE état_du_client IN ('Provence','Сhampagne')
   AND clé_client < 300
   ORDER BY état_du_client, clé_client;

SELECT customer_state, customer_key, annual_income, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_CONT
   FROM customer_dimension WHERE customer_state IN ('DC','WI') AND customer_key < 300
   ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY revenu_annuel)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_CONT
   FROM dimension_client WHERE état_du_client IN ('Provence','Сhampagne') AND clé_client < 300
   ORDER BY état_du_client, clé_client;


SELECT store_region, store_city||', '||store_state location, store_name, number_of_employees FROM store.store_dimension
     LIMIT 2 OVER (PARTITION BY store_region ORDER BY number_of_employees ASC);

SELECT регион_магазина, город_магазина||', '||область_магазина местоположение, имя_магазина, количество_сотрудников FROM магазины.измерение_магазины
     LIMIT 2 OVER (PARTITION BY регион_магазина ORDER BY количество_сотрудников ASC);


SELECT PREDICT_LINEAR_REG(waiting USING PARAMETERS model_name='myLinearRegModel') FROM
faithful ORDER BY id;

SELECT PREDICT_LINEAR_REG(attente USING PARAMETERS model_name='monRegModèleLinéaire') FROM
fidèle ORDER BY id;


SELECT INFER_EXTERNAL_TABLE_DDL('/data/people/*.parquet'
        USING PARAMETERS format = 'parquet', table_name = 'employees');

SELECT INFER_EXTERNAL_TABLE_DDL('/data/άνθρωποι/*.parquet'
        USING PARAMETERS format = 'parquet', table_name = 'εργαζόμενοι');


SELECT PREDICT_ARIMA(temperature USING PARAMETERS model_name='arima_temp', start=100, npredictions=10) OVER(ORDER BY time) FROM temp_data;

SELECT PREDICT_ARIMA(температура USING PARAMETERS model_name='arima_temp', start=100, npredictions=10) OVER(ORDER BY time) FROM временные_данные;

SELECT INFER_TABLE_DDL ('/data/*.json'
    USING PARAMETERS table_name='restaurants', format='json',
max_files=3, max_candidates=3);

SELECT INFER_TABLE_DDL ('/data/*.json'
    USING PARAMETERS table_name='εστιατόρια', format='json',
max_files=3, max_candidates=3);


SELECT PURGE_TABLE('store.store_sales_fact');

SELECT PURGE_TABLE('المتجر.متجر_مبيعات_المتجر');


SELECT MSE(obs, prediction) OVER()
   FROM (SELECT eruptions AS obs,
                PREDICT_LINEAR_REG (waiting USING PARAMETERS model_name='myLinearRegModel') AS prediction
         FROM faithful_testing) AS PredictionOutput;

SELECT MSE(наблюдения, предсказания) OVER()
   FROM (SELECT извержения AS наблюдения,
                PREDICT_LINEAR_REG (ожидания USING PARAMETERS model_name='myLinearRegModel') AS прогноз
         FROM верное_испытание) AS РезультатПрогноза;


SELECT ps[0] as q0, ps[1] as q1, ps[2] as q2, ps[3] as q3, ps[4] as q4
FROM (SELECT APPROXIMATE_PERCENTILE(sales USING PARAMETERS percentiles='0, 0.25, 0.5, 0.75, 1')
AS ps FROM allsales GROUP BY state) as s1;

SELECT pz[0] as q0, pz[1] as q1, pz[2] as q2, pz[3] as q3, pz[4] as q4
FROM (SELECT APPROXIMATE_PERCENTILE(Verkäufe USING PARAMETERS percentiles='0, 0.25, 0.5, 0.75, 1')
AS pz FROM alleVerkäufe GROUP BY Staat) as s1;


SELECT id.name, major, GPA FROM students
   WHERE id = ROW('alice',119, ARRAY['alice@example.com','ap16@cs.example.edu']);

SELECT ид.имя, курс, СРБАЛЛ FROM студенты
   WHERE ид = ROW('алиса',119, ARRAY['alice@example.com','ap16@cs.example.edu']);


SELECT E'first part o'
    'f a long line';

SELECT E'πρώτο μέρος μι'
    'ας μακράς γραμμής';


SELECT STRING_TO_ARRAY(name USING PARAMETERS collection_delimiter=' ') FROM employee;

SELECT STRING_TO_ARRAY(имя USING PARAMETERS collection_delimiter=' ') FROM сотрудники;

-- ALTER SCHEMA block
ALTER SCHEMA ms OWNER TO dbadmin CASCADE;

ALTER SCHEMA επιμελητεία OWNER TO διαχειριστής CASCADE;

ALTER SCHEMA логистика OWNER TO алиса CASCADE;

ALTER SCHEMA s1, s2 RENAME TO s3, s4;

ALTER SCHEMA εμπορικός, s2 RENAME TO продажи, s4;

-- ALTER TABLE block
ALTER TABLE public.store_orders ADD COLUMN expected_ship_date date;

ALTER TABLE public.κατάστημα_παραγγελίες ADD COLUMN αναμενόμενη_ημερομηνία_αποστολής date;

ALTER TABLE public.заказы_магазина ADD COLUMN ожиддаемая_дата_отгрузки date;

ALTER TABLE t33 OWNER TO Alice;

ALTER TABLE επιμελητεία OWNER TO διαχειριστής;

ALTER TABLE заказы OWNER TO алиса;

-- ARRAY block
SELECT (ARRAY['مسؤل', 'διαχειριστής', 'логистика', 'd', 'e'])[1];

-- Cast w/ whitespace
SELECT amount_of_honey :: FLOAT
FROM bear_inventory;

SELECT ποσότητα_μελιού :: FLOAT
FROM αρκούδα_αποθήκη;

SELECT количество_мёда :: FLOAT
FROM медвежий_склад;

-- COMMENT ON block
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'alias of APPROXIMATE_PERCENTILE with 0.5 as its parameter';
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'ψευδώνυμο APPROXIMATE_PERCENTILE με 0,5 ως παράμετρό του';
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'псевдоним APPROXIMATE_PERCENTILE с 0,5 в качестве параметра';

COMMENT ON SCHEMA public  IS 'All users can access this schema';
COMMENT ON SCHEMA public  IS 'Όλοι οι χρήστες έχουν πρόσβαση σε αυτό το σχήμα';
COMMENT ON SCHEMA public  IS 'Все пользователи могут получить доступ к этой схеме';

-- COPY block
COPY public.customer_dimension (
    customer_since FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

COPY παραγγελίες.παραγγελίες_ανά_ημέρα (
    πελάτη_αφού FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

COPY заказы.заказы_на_день (
    клиент_с_даты FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

-- CREATE PROJECTION block
CREATE PROJECTION public.employee_dimension_super
    AS SELECT * FROM public.employee_dimension
    ORDER BY employee_key
    SEGMENTED BY hash(employee_key) ALL NODES;

CREATE PROJECTION εμπορικός.παραγγελίες_ανά_ημέρα
    AS SELECT * FROM εμπορικός.παραγγελίες
    ORDER BY employee_key
    SEGMENTED BY hash(employee_key) ALL NODES;

CREATE PROJECTION продажи.продажи_на_по_клиенту
    AS SELECT * FROM продажи.продажи_на_сегодня
    ORDER BY клиент
    SEGMENTED BY hash(клиент) ALL NODES;

-- CREATE SCHEMA block
CREATE SCHEMA s3 DEFAULT INCLUDE SCHEMA PRIVILEGES;
CREATE SCHEMA εμπορικός DEFAULT INCLUDE SCHEMA PRIVILEGES;
CREATE SCHEMA продажи DEFAULT INCLUDE SCHEMA PRIVILEGES;

-- unqouted identifiers
SELECT * FROM логистика.εμπορικός;

SELECT * FROM логистика.εμπορικός1;
SELECT * FROM логистика.εμπορικός_;
SELECT * FROM логистика.s$ales$;
SELECT * FROM логистика._εμπορικός;
SELECT * FROM логистика._1234εμπορικός;

SELECT * FROM логистика1.εμπορικός;
SELECT * FROM логистика_.εμπορικός;
SELECT * FROM p$ublic$.εμπορικός;
SELECT * FROM _логистика.εμπορικός;
SELECT * FROM _1234логистика.εμπορικός;

SELECT * FROM логистика1.εμπορικός1;
SELECT * FROM логистика1_.εμπορικός1_;
SELECT * FROM p$ublic1_$.s$ales1_$;

-- quoted identifiers
SELECT * FROM "12логистика"."12344εμπορικός";
SELECT * FROM "_1234логистика"."_1234εμπορικός";
"#
                .to_string(),
        );
        let (template, str_buff) = match raw {
            LexInput::String(raw_str) => {
                let template = TemplatedFile::from_string(raw_str.clone());
                (template, raw_str)
            }
            LexInput::TemplatedFile(template_file) => {
                let str_buff = template_file.to_string();
                (template_file, str_buff)
            }
        };

        let matcher = ansi_lexers();
        let last_resort_lexer = LexMatcher::regex_lexer(
            "<unlexable>",
            r#"[^\t\n\ ]*"#,
            Token::unlexable_token,
            None,
            None,
        );
        let t0 = Instant::now();
        let test_case = lex_string(&str_buff, &matcher, &last_resort_lexer);
        let duration = t0.elapsed(); // Calculate elapsed time
        println!("lex_string time: {:?}", duration);
        // for element in test_case {
        //     println!(r#"{} <"{}">"#, element.matcher.name, element.raw);
        // }
        let templated_buffer = map_template_slices(&test_case, &template);
        // for e in x {
        //     println!(
        //         "{} : {} : {} - {}",
        //         e.matcher.name, e.raw, e.template_slice.start, e.template_slice.end
        //     )
        // }
        let duration = t0.elapsed(); // Calculate elapsed time
        println!("map_template_slices time: {:?}", duration);
        let _tokens = elements_to_tokens(&templated_buffer, &template, false);
        // for token in tokens {
        //     println!(
        //         "{}: {} : '{}'",
        //         token.pos_marker.to_source_string(),
        //         token.token_type.unwrap_or("None".to_string()),
        //         token.raw.escape_debug(),
        //     )
        // }
        let duration = t0.elapsed(); // Calculate elapsed time
        println!("Execution time: {:?}", duration);
    }
}
