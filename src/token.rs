use std::{fmt::Display, sync::Arc};

use crate::{marker::PositionMarker, slice::Slice, templater::TemplatedFile};
use pyo3::{pyclass, pymethods};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchGrammar;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Option<String>,
    pub comment_separate: bool,
    pub is_meta: bool,
    pub allow_empty: bool,
    pub pos_marker: PositionMarker,
    #[pyo3(get)]
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
}

impl Token {
    pub fn raw_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("raw".to_string()),
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
        }
    }

    pub fn code_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            ..Self::raw_token(raw, pos_marker)
        }
    }

    pub fn symbol_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("symbol".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn identifier_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("identifier".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn literal_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("literal".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn binary_operator_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("binary_operator".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn comparison_operator_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("comparison_operator".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn word_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("word".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn unlexable_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("unlexable".to_string()),
            ..Self::code_token(raw, pos_marker)
        }
    }

    pub fn whitespace_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("whitespace".to_string()),
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: " ".to_string(),
            ..Self::raw_token(raw, pos_marker)
        }
    }

    pub fn newline_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("newline".to_string()),
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: "\n".to_string(),
            ..Self::raw_token(raw, pos_marker)
        }
    }

    pub fn comment_token(raw: String, pos_marker: PositionMarker) -> Self {
        Self {
            token_type: Some("comment".to_string()),
            is_code: false,
            is_comment: true,
            ..Self::raw_token(raw, pos_marker)
        }
    }

    pub fn meta_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            token_type: Some("meta".to_string()),
            is_code: false,
            is_meta: true,
            is_templated,
            block_uuid,
            ..Self::raw_token("".to_string(), pos_marker)
        }
    }

    pub fn end_of_file_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            token_type: Some("end_of_file".to_string()),
            ..Self::meta_token(pos_marker, is_templated, block_uuid)
        }
    }

    pub fn indent_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            token_type: Some("indent".to_string()),
            indent_value: 1,
            ..Self::meta_token(pos_marker, is_templated, block_uuid)
        }
    }

    pub fn dedent_token(
        pos_marker: PositionMarker,
        is_templated: bool,
        block_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            token_type: Some("dedent".to_string()),
            indent_value: -1,
            ..Self::meta_token(pos_marker, is_templated, block_uuid)
        }
    }

    pub fn template_loop_token(pos_marker: PositionMarker, block_uuid: Option<Uuid>) -> Self {
        Self {
            token_type: Some("template_loop".to_string()),
            ..Self::meta_token(pos_marker, false, block_uuid)
        }
    }

    pub fn template_placeholder_token(
        pos_marker: PositionMarker,
        source_string: String,
        block_type: String,
        block_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            token_type: Some("placeholder".to_string()),
            block_type: Some(block_type),
            source_str: Some(source_string),
            ..Self::meta_token(pos_marker, false, block_uuid)
        }
    }

    pub fn template_placeholder_token_from_slice(
        source_slice: Slice,
        templated_slice: Slice,
        block_type: String,
        templated_file: &Arc<TemplatedFile>,
        block_uuid: Option<Uuid>,
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
            )
        }
    }
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

#[pymethods]
impl Token {
    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}
