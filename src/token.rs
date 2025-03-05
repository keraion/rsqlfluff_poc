use std::{
    fmt::{Display, Write},
    sync::{Arc, Weak},
};

use crate::{marker::PositionMarker, slice::Slice, templater::templatefile::TemplatedFile};
use hashbrown::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchGrammar;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Option<String>,
    pub class_types: HashSet<String>,
    pub comment_separate: bool,
    pub is_meta: bool,
    pub allow_empty: bool,
    pub pos_marker: PositionMarker,
    pub raw: String,
    is_whitespace: bool,
    is_code: bool,
    is_comment: bool,
    default_raw: String,
    pub indent_value: i32,
    pub is_templated: bool,
    pub block_uuid: Option<Uuid>,
    pub source_str: Option<String>,
    pub block_type: Option<String>,
    parent: Option<Weak<Token>>,
    parent_idx: Option<i32>,
    pub segments: Vec<Token>,
    preface_modifier: String,
    suffix: String,
}

impl Token {
    pub fn base_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        segments: Vec<Token>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("base", class_types);
        Self {
            token_type,
            class_types,
            comment_separate: false,
            is_meta: false,
            allow_empty: false,
            pos_marker,
            raw,
            is_whitespace: false,
            is_code: true,
            is_comment: false,
            default_raw: "".to_string(),
            indent_value: 0,
            is_templated: false,
            block_uuid: None,
            source_str: None,
            block_type: None,
            parent: None,
            parent_idx: None,
            segments,
            preface_modifier: "".to_string(),
            suffix: "".to_string(),
        }
    }

    pub fn raw_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("raw", class_types);
        Self {
            token_type,
            suffix: raw.escape_default().to_string(),
            ..Token::base_token(raw, pos_marker, class_types, vec![])
        }
    }

    pub fn code_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        Self {
            ..Self::raw_token(raw, pos_marker, class_types)
        }
    }

    pub fn symbol_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("symbol", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn identifier_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("identifier", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn literal_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("literal", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn binary_operator_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("binary_operator", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn comparison_operator_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("comparison_operator", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn word_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("word", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn unlexable_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("unlexable", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types)
        }
    }

    pub fn whitespace_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("whitespace", class_types);
        Self {
            token_type,
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: " ".to_string(),
            ..Self::raw_token(raw, pos_marker, class_types)
        }
    }

    pub fn newline_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("newline", class_types);
        Self {
            token_type,
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: "\n".to_string(),
            ..Self::raw_token(raw, pos_marker, class_types)
        }
    }

    pub fn comment_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("comment", class_types);
        Self {
            token_type,
            is_code: false,
            is_comment: true,
            ..Self::raw_token(raw, pos_marker, class_types)
        }
    }

    pub fn meta_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("meta", class_types);
        Self {
            token_type,
            is_code: false,
            is_meta: true,
            is_templated,
            block_uuid,
            preface_modifier: "[META] ".to_string(),
            ..Self::raw_token("".to_string(), pos_marker, class_types)
        }
    }

    pub fn end_of_file_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("end_of_file", class_types);
        Self {
            token_type,
            ..Self::meta_token(pos_marker, is_templated, block_uuid, class_types)
        }
    }

    pub fn indent_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("indent", class_types);
        Self {
            token_type,
            indent_value: 1,
            suffix: block_uuid
                .map(|u| u.as_hyphenated().to_string())
                .unwrap_or_default(),
            ..Self::meta_token(pos_marker, is_templated, block_uuid, class_types)
        }
    }

    pub fn dedent_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("dedent", class_types);
        Self {
            token_type,
            indent_value: -1,
            ..Self::indent_token(pos_marker, is_templated, block_uuid, class_types)
        }
    }

    pub fn template_loop_token(
        pos_marker: PositionMarker,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("template_loop", class_types);
        Self {
            token_type,
            ..Self::meta_token(pos_marker, false, block_uuid, class_types)
        }
    }

    pub fn template_placeholder_token(
        pos_marker: PositionMarker,
        source_string: String,
        block_type: String,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("placeholder", class_types);
        Self {
            token_type,
            block_type: Some(block_type),
            source_str: Some(source_string),
            ..Self::meta_token(pos_marker, false, block_uuid, class_types)
        }
    }

    pub fn template_placeholder_token_from_slice(
        source_slice: Slice,
        templated_slice: Slice,
        block_type: String,
        templated_file: &Arc<TemplatedFile>,
        block_uuid: Option<Uuid>,
        class_types: HashSet<String>,
    ) -> Self {
        let pos_marker = PositionMarker::new(
            source_slice.clone(),
            templated_slice,
            templated_file,
            None,
            None,
        );
        Self {
            ..Self::template_placeholder_token(
                pos_marker,
                templated_file.source_str[source_slice.as_range()].to_string(),
                block_type,
                block_uuid,
                class_types,
            )
        }
    }

    pub fn raw_segments(&self) -> Vec<Token> {
        vec![self.clone()]
    }

    pub fn raw_upper(&self) -> String {
        self.raw.to_uppercase()
    }

    pub fn get_type(&self) -> String {
        self.token_type.clone().expect("Token has no type")
    }

    pub fn _get_raw_segment_kwargs(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    pub fn iter_unparseables(&self) -> Vec<Token> {
        self.segments
            .iter()
            .flat_map(|s| s.iter_unparseables())
            .collect()
    }

    pub fn set_parent(&mut self, parent: Token, idx: i32) {
        let parent = Arc::new(parent);
        self.parent = Some(Arc::downgrade(&parent));
        self.parent_idx = Some(idx);
    }

    pub fn first_non_whitespace_segment_raw_upper(&self) -> Option<String> {
        self.raw_segments()
            .iter()
            .filter(|x| !x.raw_upper().trim().is_empty())
            .next()
            .map(|y| y.raw_upper())
    }

    pub fn class_is_type(&self, seg_types: Vec<String>) -> bool {
        let seg_hash: HashSet<String> = seg_types.into_iter().collect();
        self.class_types.intersection(&seg_hash).count() > 0
    }

    pub fn is_type(&self, seg_types: Vec<String>) -> bool {
        self.class_is_type(seg_types)
    }

    fn comments(&self) -> Vec<Token> {
        self.segments
            .clone()
            .into_iter()
            .filter(|s| s.is_type(vec!["comment".to_string()]))
            .collect::<Vec<_>>()
    }

    fn non_comments(&self) -> Vec<Token> {
        self.segments
            .clone()
            .into_iter()
            .filter(|s| !s.is_type(vec!["comment".to_string()]))
            .collect::<Vec<_>>()
    }

    fn preface(&self, ident: usize, tabsize: usize) -> String {
        let padding = " ".repeat(ident * tabsize);
        let padded_type = format!("{}{}{}:", padding, self.preface_modifier, self.get_type());

        let pos = self.pos_marker.clone();
        let suffix = self.suffix.clone();

        let preface = format!(
            "{:<20}|{:<60}  {}",
            pos.to_source_string(),
            padded_type,
            suffix
        );

        preface.trim_end().to_string()
    }

    pub fn stringify(&self, ident: usize, tabsize: usize, code_only: bool) -> String {
        let mut buff = String::new();
        let preface = self.preface(ident, tabsize);
        writeln!(buff, "{}", preface).unwrap();

        if !code_only && self.comment_separate && !self.comments().is_empty() {
            if !self.comments().is_empty() {
                writeln!(buff, "{}Comments:", " ".repeat((ident + 1) * tabsize)).unwrap();
                for seg in &self.comments() {
                    let segment_string = seg.stringify(ident + 2, tabsize, code_only);
                    buff.push_str(&segment_string);
                }
            }

            if !self.non_comments().is_empty() {
                writeln!(buff, "{}Code:", " ".repeat((ident + 1) * tabsize)).unwrap();
                for seg in &self.non_comments() {
                    let segment_string = seg.stringify(ident + 2, tabsize, code_only);
                    buff.push_str(&segment_string);
                }
            }
        } else {
            for seg in &self.segments {
                if !code_only || seg.is_code {
                    let segment_string = seg.stringify(ident + 1, tabsize, code_only);
                    buff.push_str(&segment_string);
                }
            }
        }

        buff
    }
}

fn iter_base_types(
    token_type: &str,
    class_types: HashSet<String>,
) -> (Option<String>, HashSet<String>) {
    let mut class_types = class_types;
    let token_type = Some(token_type.to_string());
    if let Some(token_type) = token_type.clone() {
        class_types.insert(token_type);
    }
    (token_type, class_types)
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}: ({}) '{}'>",
            self.token_type.clone().unwrap_or("unknown".to_string()),
            self.pos_marker,
            self.raw.escape_debug(),
        )
    }
}

pub mod python {
    use std::fmt::Display;

    use hashbrown::{HashMap, HashSet};
    use pyo3::{prelude::*, types::PyTuple};

    use crate::marker::python::PyPositionMarker;

    use super::Token;

    #[pyclass(name = "Token", weakref)]
    #[derive(Debug, Clone)]
    pub struct PyToken(pub Token);

    #[pymethods]
    impl PyToken {
        #[getter]
        pub fn raw(&self) -> String {
            self.0.raw.to_string()
        }

        #[getter]
        pub fn pos_marker(&self) -> PyPositionMarker {
            PyPositionMarker(self.0.pos_marker.clone())
        }

        // #[getter(get_type)]
        pub fn get_type(&self) -> String {
            self.0.get_type()
        }

        #[getter]
        pub fn is_code(&self) -> bool {
            self.0.is_code
        }

        #[getter]
        pub fn is_meta(&self) -> bool {
            self.0.is_meta
        }

        #[pyo3(signature = (*seg_type))]
        pub fn is_type<'py>(&self, seg_type: &Bound<'py, PyTuple>) -> bool {
            let seg_strs = seg_type
                .extract::<Vec<String>>()
                .expect("args should be all strings");
            let mut seg_set = HashSet::new();
            seg_strs.clone().iter().for_each(|s| {
                seg_set.insert(s.clone());
            });
            self.0
                .token_type
                .clone()
                .map(|t| seg_strs.contains(&t))
                .unwrap_or_default()
                || self.0.class_types.intersection(&seg_set).count() > 0
        }

        #[getter]
        pub fn indent_val(&self) -> i32 {
            self.0.indent_value
        }

        #[getter]
        pub fn class_types(&self) -> HashSet<String> {
            self.0.class_types.clone()
        }

        #[getter]
        pub fn first_non_whitespace_segment_raw_upper(&self) -> Option<String> {
            self.0.first_non_whitespace_segment_raw_upper()
        }

        #[getter]
        pub fn raw_upper(&self) -> String {
            self.0.raw_upper()
        }

        pub fn _get_raw_segment_kwargs(&self) -> HashMap<String, String> {
            self.0._get_raw_segment_kwargs()
        }

        // pub fn set_parent<'py>(&mut self, parent: Bound<'py, PyAny>, idx: i32) {
        pub fn set_parent<'py>(&self, parent: Bound<'py, PyAny>, idx: i32) {
            // parent
            let token_type = parent.getattr("type").expect("no type");
            let raw = parent.getattr("raw").expect("no raw");
            // println!("{}: {}", token_type, raw);
            let parent_token = parent.extract::<PyToken>();
            // println!("{:?}", parent_token);
            // self.0.set_parent(parent.into(), idx);
        }

        pub fn iter_unparsables(&self) -> Vec<PyToken> {
            self.0
                .iter_unparseables()
                .into_iter()
                .map(Into::into)
                .collect()
        }

        #[pyo3(signature = (ident=0, tabsize=4, code_only=false))]
        pub fn stringify(
            &self,
            ident: Option<usize>,
            tabsize: Option<usize>,
            code_only: Option<bool>,
        ) -> String {
            self.0.stringify(
                ident.unwrap_or(0),
                tabsize.unwrap_or(4),
                code_only.unwrap_or_default(),
            )
        }

        pub fn __repr__(&self) -> String {
            format!("{}", self)
        }
    }

    impl Display for PyToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "<{}: ({}) '{}'>",
                self.0.token_type.clone().unwrap_or("unknown".to_string()),
                self.0.pos_marker,
                self.0.raw.escape_debug(),
            )
        }
    }

    impl Into<Token> for PyToken {
        fn into(self) -> Token {
            self.0
        }
    }

    impl From<Token> for PyToken {
        fn from(value: Token) -> Self {
            Self(value)
        }
    }
}
