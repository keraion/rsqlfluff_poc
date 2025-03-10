use std::{
    fmt::{Display, Write},
    sync::{Arc, Weak},
};

use crate::{marker::PositionMarker, slice::Slice, templater::templatefile::TemplatedFile};
use hashbrown::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchGrammar;

#[derive(Debug, Clone, Hash)]
pub struct SourceFix {
    edit: String,
    source_slice: Slice,
    templated_slice: Slice,
}

impl PartialEq for SourceFix {
    fn eq(&self, other: &Self) -> bool {
        self.edit == other.edit && self.source_slice == other.source_slice
    }
}

#[derive(Debug, Clone)]
pub struct PathStep {
    pub segment: Arc<Token>,
    pub idx: usize,
    pub len: usize,
    pub code_idxs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TupleSerialisedSegment {
    Str(String, String),
    Nested(String, Vec<TupleSerialisedSegment>),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Option<String>,
    pub class_types: HashSet<String>,
    pub comment_separate: bool,
    pub is_meta: bool,
    pub allow_empty: bool,
    pub pos_marker: Option<PositionMarker>,
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
    parent_idx: Option<usize>,
    pub segments: Vec<Token>,
    preface_modifier: String,
    suffix: String,
    pub uuid: u128,
    pub source_fixes: Option<Vec<SourceFix>>,
    pub trim_start: Option<Vec<String>>,
    pub trim_chars: Option<Vec<String>>,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Token {
    pub fn base_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        segments: Vec<Token>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("base", class_types);
        Self {
            token_type,
            class_types,
            comment_separate: false,
            is_meta: false,
            allow_empty: false,
            pos_marker: Some(pos_marker),
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
            parent: None.into(),
            parent_idx: None.into(),
            segments,
            preface_modifier: "".to_string(),
            suffix: "".to_string(),
            uuid: Uuid::new_v4().as_u128(),
            source_fixes: None,
            trim_start,
            trim_chars,
        }
    }

    pub fn raw_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("raw", class_types);
        Self {
            token_type,
            // Match python's string
            suffix: format!("'{}'", raw.escape_debug().to_string().trim_matches('"')),
            ..Token::base_token(raw, pos_marker, class_types, vec![], trim_start, trim_chars)
        }
    }

    pub fn code_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        Self {
            ..Self::raw_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn symbol_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("symbol", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn identifier_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("identifier", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn literal_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("literal", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn binary_operator_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("binary_operator", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn comparison_operator_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("comparison_operator", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn word_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("word", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn unlexable_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("unlexable", class_types);
        Self {
            token_type,
            ..Self::code_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn whitespace_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("whitespace", class_types);
        Self {
            token_type,
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: " ".to_string(),
            ..Self::raw_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn newline_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("newline", class_types);
        Self {
            token_type,
            is_whitespace: true,
            is_code: false,
            is_comment: false,
            default_raw: "\n".to_string(),
            ..Self::raw_token(raw, pos_marker, class_types, trim_start, trim_chars)
        }
    }

    pub fn comment_token(
        raw: String,
        pos_marker: PositionMarker,
        class_types: HashSet<String>,
        trim_start: Option<Vec<String>>,
        trim_chars: Option<Vec<String>>,
    ) -> Self {
        let (token_type, class_types) = iter_base_types("comment", class_types);
        Self {
            token_type,
            is_code: false,
            is_comment: true,
            ..Self::raw_token(raw, pos_marker, class_types, trim_start, trim_chars)
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
            suffix: String::new(),
            ..Self::raw_token("".to_string(), pos_marker, class_types, None, None)
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
        match self.segments.len() {
            1.. => self
                .segments
                .iter()
                .flat_map(|s| s.raw_segments())
                .collect::<Vec<_>>(),
            0 => vec![self.clone()],
        }
    }

    pub fn raw_upper(&self) -> String {
        self.raw.to_uppercase()
    }

    pub fn raw_trimmed(&self) -> String {
        let mut raw_buff = self.raw.clone();

        // Trim start sequences
        if let Some(trim_start) = &self.trim_start {
            for seq in trim_start {
                raw_buff = raw_buff.strip_prefix(seq).unwrap_or(&raw_buff).to_string();
            }
        }

        // Trim specified characters from both ends
        if let Some(trim_chars) = &self.trim_chars {
            raw_buff = self.raw.clone(); // Reset raw_buff before trimming chars

            for seq in trim_chars {
                while raw_buff.starts_with(seq) {
                    raw_buff = raw_buff.strip_prefix(seq).unwrap_or(&raw_buff).to_string();
                }
                while raw_buff.ends_with(seq) {
                    raw_buff = raw_buff.strip_suffix(seq).unwrap_or(&raw_buff).to_string();
                }
            }
        }

        raw_buff
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

    pub fn set_parent(mut self, parent: Arc<Token>, idx: usize) {
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

    pub fn class_is_type(&self, seg_types: &[&str]) -> bool {
        let seg_hash: HashSet<String> = seg_types.into_iter().map(|s| s.to_string()).collect();
        self.class_types.intersection(&seg_hash).count() > 0
    }

    pub fn count_segments(&self, raw_only: bool) -> usize {
        if self.is_raw() {
            1
        } else {
            let self_count = if raw_only { 0 } else { 1 };
            self.segments
                .iter()
                .fold(0, |acc, s| acc + s.count_segments(raw_only) + self_count)
        }
    }

    pub fn is_type(&self, seg_types: &[&str]) -> bool {
        self.class_is_type(seg_types)
    }

    pub fn is_raw(&self) -> bool {
        self.segments.len() == 0
    }

    pub fn is_code(&self) -> bool {
        match self.segments.len() {
            0 => self.is_code,
            1.. => self.segments.iter().any(|s| s.is_code()),
        }
    }

    pub fn is_templated(&self) -> bool {
        let pos_marker = self.pos_marker.clone().expect("PositionMarker must be set");
        pos_marker.source_slice.start != pos_marker.source_slice.stop && !pos_marker.is_literal()
    }

    fn code_indices(&self) -> Vec<usize> {
        self.segments
            .iter()
            .enumerate()
            .filter(|(_i, s)| s.is_code())
            .map(|(i, _s)| i)
            .collect()
    }

    pub fn recursive_crawl(
        &self,
        seg_types: &[&str],
        recurse_into: bool,
        no_recursive_seg_type: Option<&[&str]>,
        allow_self: bool,
    ) -> Vec<Token> {
        let seg_type_set: HashSet<String> = seg_types.iter().map(|s| s.to_string()).collect();
        let seg_type_vec: Vec<&str> = seg_types.iter().cloned().collect();
        let no_recursive_set: HashSet<&str> = no_recursive_seg_type
            .unwrap_or(&[])
            .iter()
            .cloned()
            .collect();

        let mut results = Vec::new();

        // Check if self matches the given segment types
        if allow_self && self.is_type(&seg_type_vec) {
            results.push(self.clone());
        }

        // If no matching descendants, terminate early
        if self.descendant_type_set().is_disjoint(&seg_type_set) {
            return results;
        }

        // Recursively process child segments
        for seg in &self.segments {
            if !no_recursive_set.contains(seg.token_type.as_deref().unwrap_or("")) {
                results.extend(seg.recursive_crawl(
                    seg_types,
                    recurse_into,
                    no_recursive_seg_type,
                    true,
                ));
            }
        }

        results
    }

    pub fn path_to(self, other: Self) -> Vec<PathStep> {
        // Return empty if they are the same segment.
        if self == other {
            return vec![];
        }

        // If there are no child segments, return empty.
        if self.segments.is_empty() {
            return vec![];
        }

        // Identifying the highest parent we can using any preset parent values.
        let mut midpoint = other.clone();
        let mut lower_path = Vec::new();

        while let Some(weak_parent) = &midpoint.parent.clone().as_ref() {
            if let Some(parent) = weak_parent.upgrade() {
                let parent_idx = midpoint.parent_idx.expect("Parent index must be set.");

                lower_path.push(PathStep {
                    segment: Arc::clone(&parent),
                    idx: parent_idx,
                    len: parent.segments.len(),
                    code_idxs: parent.code_indices().clone(),
                });

                midpoint = Arc::unwrap_or_clone(parent);
                if midpoint == self {
                    break;
                }
            } else {
                break;
            }
        }

        // Reverse the path so far
        lower_path.reverse();

        // If we have already found the parent, return.
        if midpoint == self {
            return lower_path;
        }
        // If we've gone all the way up to the file segment, return empty.
        if midpoint.class_is_type(&["file"]) {
            return vec![];
        }
        // Check if midpoint is within self's range.
        if !(self.get_start_loc() <= midpoint.get_start_loc()
            && midpoint.get_start_loc() <= self.get_end_loc())
        {
            return vec![];
        }

        // Now, work downward from `self` toward `midpoint`.
        for (idx, seg) in self.segments.clone().iter().enumerate() {
            // Set the parent if it's not already set.
            let seg = seg.clone();
            seg.clone().set_parent(Arc::new(self.clone()), idx);

            let step = PathStep {
                segment: Arc::new(self.clone()),
                idx: idx,
                len: self.segments.clone().len(),
                code_idxs: self.code_indices().clone(),
            };

            // If we found the target
            if seg == midpoint {
                let mut result = vec![step];
                result.extend(lower_path);
                return result;
            }

            // Check recursively if a path exists
            let res = seg.path_to(midpoint.clone());
            if !res.is_empty() {
                let mut result = vec![step];
                result.extend(res);
                result.extend(lower_path);
                return result;
            }
        }

        // Not found.
        vec![]
    }

    pub fn get_start_loc(&self) -> (usize, usize) {
        self.pos_marker
            .clone()
            .expect("PositionMarker unset")
            .working_loc()
    }

    pub fn get_end_loc(&self) -> (usize, usize) {
        self.pos_marker
            .clone()
            .expect("PositionMarker unset")
            .working_loc_after(&self.raw)
    }

    pub fn source_fixes(&self) -> Vec<SourceFix> {
        match self.is_raw() {
            true => self.source_fixes.clone().unwrap_or_default(),
            false => self
                .segments
                .iter()
                .flat_map(|s| s.source_fixes())
                .collect(),
        }
    }

    pub fn recursive_crawl_all(&self, reverse: bool) -> Box<dyn Iterator<Item = &Token> + '_> {
        if reverse {
            Box::new(
                self.segments
                    .iter()
                    .rev()
                    .flat_map(move |seg| seg.recursive_crawl_all(reverse))
                    .chain(std::iter::once(self)),
            )
        } else {
            Box::new(
                std::iter::once(self).chain(
                    self.segments
                        .iter()
                        .flat_map(move |seg| seg.recursive_crawl_all(reverse)),
                ),
            )
        }
    }

    pub fn descendant_type_set(&self) -> HashSet<String> {
        self.segments
            .iter()
            .flat_map(|seg| {
                seg.descendant_type_set()
                    .union(&seg.class_types)
                    .cloned()
                    .collect::<HashSet<String>>()
            })
            .collect::<HashSet<String>>()
    }

    fn comments(&self) -> Vec<Token> {
        self.segments
            .clone()
            .into_iter()
            .filter(|s| s.is_type(&["comment"]))
            .collect::<Vec<_>>()
    }

    fn non_comments(&self) -> Vec<Token> {
        self.segments
            .clone()
            .into_iter()
            .filter(|s| !s.is_type(&["comment"]))
            .collect::<Vec<_>>()
    }

    fn preface(&self, ident: usize, tabsize: usize) -> String {
        let padding = " ".repeat(ident * tabsize);
        let padded_type = format!("{}{}{}:", padding, self.preface_modifier, self.get_type());

        let pos = self.pos_marker.clone();
        let suffix = self.suffix.clone();

        let preface = format!(
            "{:<20}|{:<60}  {}",
            pos.clone()
                .expect("PositionMarker unset")
                .to_source_string(),
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

    pub fn to_tuple(
        &self,
        code_only: Option<bool>,
        show_raw: Option<bool>,
        include_meta: Option<bool>,
    ) -> TupleSerialisedSegment {
        let code_only = code_only.unwrap_or_default();
        let show_raw = show_raw.unwrap_or_default();
        let include_meta = include_meta.unwrap_or_default();
        // If `show_raw` is true and there are no child segments, return (type, raw)
        if show_raw && self.segments.is_empty() {
            return TupleSerialisedSegment::Str(self.get_type(), self.raw.clone());
        }

        // Determine filtering criteria for child segments
        let filtered_segments: Vec<TupleSerialisedSegment> = self
            .segments
            .iter()
            .filter(|seg| {
                if code_only {
                    seg.is_code && !seg.is_meta
                } else {
                    include_meta || !seg.is_meta
                }
            })
            .map(|seg| seg.to_tuple(Some(code_only), Some(show_raw), Some(include_meta)))
            .collect();

        TupleSerialisedSegment::Nested(self.get_type(), filtered_segments)
    }

    pub fn copy(
        &self,
        segments: Option<Vec<Token>>,
        parent: Option<Arc<Token>>,
        parent_idx: Option<usize>,
    ) -> Token {
        let mut new_segment = self.clone();
        new_segment.parent = parent.as_ref().map(Arc::downgrade).into();
        new_segment.parent_idx = parent_idx.into();

        if let Some(ref segs) = segments {
            new_segment.segments = segs.clone();
        } else {
            new_segment.segments = self
                .segments
                .iter()
                .enumerate()
                .map(|(idx, seg)| {
                    seg.copy(
                        None,
                        Some(Arc::new(new_segment.clone())).into(),
                        Some(idx).into(),
                    )
                })
                .collect();
        }

        new_segment
    }

    pub fn edit(&self, raw: Option<String>, source_fixes: Option<Vec<SourceFix>>) -> Self {
        Self {
            raw: raw.unwrap_or(self.raw.clone()),
            source_fixes: Some(source_fixes.unwrap_or(self.source_fixes())),
            ..self.clone()
        }
    }

    pub fn position_segments(segments: &[Token], parent_pos: PositionMarker) -> Vec<Token> {
        assert!(
            !segments.is_empty(),
            "position_segments called on empty sequence."
        );
        let mut line_no = parent_pos.working_line_no;
        let mut line_pos = parent_pos.working_line_pos;

        let mut segment_buffer = Vec::new();

        for (idx, segment) in segments.iter().enumerate() {
            let old_position = segment.pos_marker.clone();
            let mut new_position = segment.pos_marker.clone();

            // If position is missing, try to infer it
            if new_position.is_none() {
                let mut start_point = None;
                if idx > 0 {
                    let prev_seg: &Token = &segment_buffer[idx - 1];
                    if let Some(ref pos_marker) = prev_seg.pos_marker {
                        start_point = Some(pos_marker.end_point_marker());
                    }
                } else {
                    start_point = Some(parent_pos.start_point_marker());
                }

                // Search forward for the end point
                let mut end_point = None;
                for fwd_seg in &segments[idx + 1..] {
                    if let Some(ref pos_marker) = fwd_seg.pos_marker {
                        end_point = Some(pos_marker.start_point_marker());
                        break;
                    }
                }

                new_position = match (start_point, end_point) {
                    (Some(start), Some(end)) if start != end => {
                        Some(PositionMarker::from_points(&start, &end))
                    }
                    (Some(start), _) => Some(start),
                    (_, Some(end)) => Some(end),
                    _ => panic!("Unable to position new segment"),
                };
            }

            let new_position = new_position.expect("Position should be assigned");
            let new_position = new_position.with_working_position(line_no, line_pos);
            let (new_line_no, new_line_pos) =
                new_position.infer_next_position(&segment.raw, line_no, line_pos);
            line_no = new_line_no;
            line_pos = new_line_pos;

            // If position changed, recursively process child segments before copying
            let new_segment =
                if !segment.segments.is_empty() && old_position != Some(new_position.clone()) {
                    let child_segments =
                        Token::position_segments(&segment.segments, new_position.clone());
                    segment.copy(Some(child_segments.into()), None, None)
                } else {
                    segment.copy(None, None, None)
                };

            segment_buffer.push(new_segment);
        }

        segment_buffer
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
            self.pos_marker.clone().expect("PositionMarker unset"),
            self.raw.escape_debug(),
        )
    }
}

pub mod python {
    use std::{
        fmt::{Debug, Display},
        rc::Weak,
        slice::Iter,
        sync::Arc,
    };

    use hashbrown::{HashMap, HashSet};
    use pyo3::{
        prelude::*,
        types::{PyIterator, PyString, PyTuple, PyType},
        BoundObject, IntoPyObjectExt,
    };
    use uuid::Uuid;

    use crate::marker::{
        python::{PyPositionMarker, PySqlFluffPositionMarker},
        PositionMarker,
    };

    use super::{PathStep, SourceFix, Token, TupleSerialisedSegment};

    #[pyclass(name = "SourceFix")]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct PySourceFix(pub SourceFix);

    impl Into<SourceFix> for PySourceFix {
        fn into(self) -> SourceFix {
            self.0
        }
    }

    impl From<SourceFix> for PySourceFix {
        fn from(value: SourceFix) -> Self {
            Self(value)
        }
    }

    #[pyclass(name = "PathStep")]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct PyPathStep(pub PathStep);

    impl Into<PathStep> for PyPathStep {
        fn into(self) -> PathStep {
            self.0
        }
    }

    impl From<PathStep> for PyPathStep {
        fn from(value: PathStep) -> Self {
            Self(value)
        }
    }

    #[pyclass(name = "TupleSerialisedSegment")]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct PyTupleSerialisedSegment(pub TupleSerialisedSegment);

    impl PyTupleSerialisedSegment {
        pub fn to_py_tuple<'py>(&self, py: Python<'py>) -> Result<Bound<'py, PyTuple>, PyErr> {
            match &self.0 {
                TupleSerialisedSegment::Str(segment_type, raw_value) => {
                    PyTuple::new(py, &[segment_type, raw_value])
                }
                TupleSerialisedSegment::Nested(segment_type, segments) => {
                    let py_segment_type = PyString::new(py, segment_type);
                    let py_segments: Vec<_> = segments
                        .iter()
                        .map(|s| {
                            PyTupleSerialisedSegment::to_py_tuple(
                                &PyTupleSerialisedSegment(s.clone()),
                                py,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let pt_segments_tuple = PyTuple::new(py, &py_segments)?;

                    PyTuple::new(
                        py,
                        &[py_segment_type.into_any(), pt_segments_tuple.into_any()],
                    )
                }
            }
        }
    }

    impl Into<TupleSerialisedSegment> for PyTupleSerialisedSegment {
        fn into(self) -> TupleSerialisedSegment {
            self.0
        }
    }

    impl From<TupleSerialisedSegment> for PyTupleSerialisedSegment {
        fn from(value: TupleSerialisedSegment) -> Self {
            Self(value)
        }
    }

    #[pyclass(name = "Token", weakref)]
    #[repr(transparent)]
    #[derive(Debug, Clone)]
    pub struct PyToken(pub Token);

    #[pymethods]
    impl PyToken {
        #[getter]
        pub fn raw(&self) -> String {
            self.0.raw.to_string()
        }

        pub fn raw_trimmed(&self) -> String {
            self.0.raw_trimmed()
        }

        #[getter]
        pub fn pos_marker(&self) -> Option<PyPositionMarker> {
            self.0.pos_marker.clone().map(PyPositionMarker)
        }

        #[setter]
        pub fn set_pos_marker(&mut self, value: Option<PySqlFluffPositionMarker>) {
            self.0.pos_marker = value.map(Into::into);
        }

        pub fn get_type(&self) -> String {
            self.0.get_type()
        }

        #[getter(r#type)]
        pub fn type_(&self) -> String {
            self.0.get_type()
        }

        #[getter]
        pub fn is_templated(&self) -> bool {
            self.0.is_templated()
        }

        #[getter]
        pub fn is_code(&self) -> bool {
            self.0.is_code
        }

        #[getter]
        pub fn is_meta(&self) -> bool {
            self.0.is_meta
        }

        #[pyo3(signature = (raw_only = false))]
        pub fn count_segments(&self, raw_only: Option<bool>) -> usize {
            self.0.count_segments(raw_only.unwrap_or_default())
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
        pub fn is_whitespace(&self) -> bool {
            self.0.is_whitespace
        }

        pub fn is_raw(&self) -> bool {
            self.0.is_raw()
        }

        #[getter]
        pub fn class_types(&self) -> HashSet<String> {
            self.0.class_types.clone()
        }

        #[getter]
        pub fn source_fixes(&self) -> Vec<PySourceFix> {
            self.0.source_fixes().into_iter().map(Into::into).collect()
        }

        #[getter]
        pub fn _source_fixes(&self) -> Option<Vec<PySourceFix>> {
            self.0
                .source_fixes
                .clone()
                .map(|sf| sf.into_iter().map(Into::into).collect())
        }

        #[pyo3(signature = (*seg_type))]
        pub fn class_is_type<'py>(&self, seg_type: &Bound<'py, PyTuple>) -> bool {
            let seg_strs = seg_type
                .extract::<Vec<String>>()
                .expect("args should be all strings");
            self.0
                .class_is_type(&seg_strs.iter().map(String::as_str).collect::<Vec<&str>>())
        }

        #[getter]
        pub fn first_non_whitespace_segment_raw_upper(&self) -> Option<String> {
            self.0.first_non_whitespace_segment_raw_upper()
        }

        #[getter]
        pub fn raw_upper(&self) -> String {
            self.0.raw_upper()
        }

        pub fn invalidate_caches(&self) {}

        #[getter]
        pub fn uuid(&self) -> u128 {
            self.0.uuid
        }

        #[getter]
        pub fn descendant_type_set(&self) -> HashSet<String> {
            self.0.descendant_type_set()
        }

        #[pyo3(signature = (*seg_type, recurse_into = true, no_recursive_seg_type = None, allow_self = true))]
        pub fn recursive_crawl<'py>(
            &self,
            seg_type: &Bound<'py, PyTuple>,
            recurse_into: bool,
            no_recursive_seg_type: Option<Bound<'_, PyAny>>,
            allow_self: bool,
        ) -> Vec<PyToken> {
            let seg_type = seg_type
                .extract::<Vec<String>>()
                .expect("args should be all strings");
            let temp: Option<Vec<String>> = match no_recursive_seg_type {
                Some(py_any) => {
                    if let Ok(single_str) = py_any.extract::<String>() {
                        Some(vec![single_str]) // Convert single string into a Vec<String>
                    } else if let Ok(list_of_str) = py_any.extract::<Vec<String>>() {
                        Some(list_of_str) // Already a Vec<String>, return as is
                    } else {
                        Some(vec![]) // If it's neither, return an empty vector
                    }
                }
                None => None, // If None, return an empty vector
            };
            let no_recursive_seg_type: Option<Vec<&str>> = temp
                .as_ref()
                .map(|vec| vec.iter().map(String::as_str).collect());

            self.0
                .recursive_crawl(
                    &seg_type.iter().map(String::as_str).collect::<Vec<&str>>(),
                    recurse_into,
                    no_recursive_seg_type.as_deref(),
                    allow_self,
                )
                .into_iter()
                .map(Into::into)
                .collect()
        }

        pub fn recursive_crawl_all(&self, reverse: bool) -> Vec<PyToken> {
            self.0
                .recursive_crawl_all(reverse)
                .into_iter()
                .map(|t| t.clone().into())
                .collect()
        }

        #[getter]
        pub fn segments(&self) -> Vec<PyToken> {
            self.0
                .segments
                .clone()
                .into_iter()
                .map(Into::into)
                .collect()
        }

        pub fn path_to(&self, other: PyToken) -> Vec<PyPathStep> {
            self.0
                .clone()
                .path_to(other.into())
                .into_iter()
                .map(Into::into)
                .collect()
        }

        pub fn get_start_loc(&self) -> (usize, usize) {
            self.0.get_start_loc()
        }

        pub fn get_end_loc(&self) -> (usize, usize) {
            self.0.get_end_loc()
        }

        #[getter]
        pub fn raw_segments(&self) -> Vec<PyToken> {
            self.0.raw_segments().into_iter().map(Into::into).collect()
        }

        pub fn _get_raw_segment_kwargs(&self) -> HashMap<String, String> {
            self.0._get_raw_segment_kwargs()
        }

        // pub fn set_parent<'py>(&mut self, parent: Bound<'py, PyAny>, idx: i32) {
        pub fn set_parent<'py>(&self, parent: Bound<'py, PyAny>, idx: usize) {
            // parent
            let token_type = parent.getattr("type").expect("no type");
            let raw = parent.getattr("raw").expect("no raw");
            // println!("{}: {}", token_type, raw);
            let parent_token: Token = parent
                .extract::<PySqlFluffToken>()
                .map(Into::into)
                .expect("bad parent");
            // println!("{:?}", parent_token);
            self.0
                .clone()
                .set_parent(Arc::new(parent_token.clone()), idx);
        }

        pub fn get_parent(&self) -> Option<(PyToken, i32)> {
            None
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

        pub fn to_tuple<'py>(
            &self,
            py: Python<'py>,
            code_only: Option<bool>,
            show_raw: Option<bool>,
            include_meta: Option<bool>,
        ) -> Result<Bound<'py, PyTuple>, PyErr> {
            PyTupleSerialisedSegment(self.0.to_tuple(code_only, show_raw, include_meta))
                .to_py_tuple(py)
        }

        pub fn copy(
            &self,
            segments: Option<Vec<PySqlFluffToken>>,
            parent: Option<PySqlFluffToken>,
            parent_idx: Option<usize>,
        ) -> PyToken {
            PyToken(
                self.0.copy(
                    segments.map(|s| s.into_iter().map(Into::into).collect()),
                    parent
                        .as_ref()
                        .map(|parent_token| Arc::clone(&parent_token.0 .0.clone().into())),
                    parent_idx,
                ),
            )
        }

        #[pyo3(signature = (raw=None, source_fixes=None))]
        pub fn edit(&self, raw: Option<String>, source_fixes: Option<Vec<PySourceFix>>) -> Self {
            Self(self.0.edit(
                raw,
                source_fixes.map(|sf| sf.into_iter().map(Into::into).collect()),
            ))
        }

        #[classmethod]
        pub fn position_segments<'py>(
            _cls: &Bound<'py, PyType>,
            py: Python<'py>,
            segments: Vec<PySqlFluffToken>,
            parent_pos: PySqlFluffPositionMarker,
        ) -> Result<Bound<'py, PyTuple>, PyErr> {
            let tokens = Token::position_segments(
                &segments
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<Token>>(),
                parent_pos.into(),
            );
            PyTuple::new(
                py,
                tokens.into_iter().map(Into::into).collect::<Vec<PyToken>>(),
            )
        }

        pub fn __repr__(&self) -> String {
            format!("{}", self)
        }
    }

    impl Display for PyToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
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

    #[derive(IntoPyObject)]
    pub struct PySqlFluffToken(pub PyToken);

    impl<'py> FromPyObject<'py> for PySqlFluffToken {
        fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
            // println!("{}{}{:?}{:?}", ob, ob.get_type(), ob.str(), ob.repr());
            // println!("{:?}", ob);
            let raw = ob.getattr("raw")?.extract::<String>()?;
            // println!("raw: {:?}", raw);
            let class_types = ob
                .getattr("_class_types")
                .unwrap_or(ob.getattr("class_types")?)
                .extract::<HashSet<String>>()?;
            // println!("class_types: {:?}", class_types);
            // println!("{}{:?}", raw, class_types);
            let segments = ob
                .getattr("segments")?
                .extract::<Vec<PySqlFluffToken>>()
                .map(|s| s.into_iter().map(Into::into).collect::<Vec<Token>>())?;
            // println!("segments: {:#?}", segments);
            let pos_marker = ob
                .getattr("pos_marker")?
                .extract::<PySqlFluffPositionMarker>()?;
            // println!("pos_marker: {:?}", pos_marker);
            Ok(Self(PyToken(Token::base_token(
                raw,
                pos_marker.into(),
                class_types,
                segments,
                None,
                None,
            ))))
        }
    }

    impl Into<Token> for PySqlFluffToken {
        fn into(self) -> Token {
            self.0 .0
        }
    }

    impl From<Token> for PySqlFluffToken {
        fn from(value: Token) -> Self {
            Self(PyToken(value))
        }
    }
}
