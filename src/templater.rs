use std::{
    cmp::{max, min},
    collections::HashMap,
};

use pyo3::{pyclass, pymethods, FromPyObject, IntoPyObject};

use crate::slice::Slice;

#[derive(Debug, PartialEq, Clone, FromPyObject, IntoPyObject)]
pub struct RawFileSlice {
    pub raw: String, // Source string
    pub slice_type: String,
    pub source_idx: usize, // Offset from beginning of source string
    // Block index, incremented on start or end block tags, e.g. "if", "for".
    // This is used in `BaseRule.discard_unsafe_fixes()` to reject any fixes
    // which span multiple templated blocks.
    pub block_idx: usize,
    // The command of a templated tag, e.g. "if", "for"
    // This is used in template tracing as a kind of cache to identify the kind
    // of template element this is without having to re-extract it each time.
    pub tag: Option<String>,
}

impl RawFileSlice {
    pub fn new(raw: String, slice_type: String, source_idx: usize) -> Self {
        RawFileSlice {
            raw,
            slice_type,
            source_idx,
            block_idx: 0,
            tag: None,
        }
    }

    pub fn end_source_idx(&self) -> usize {
        // Return the closing index of this slice.
        self.source_idx + self.raw.len()
    }

    pub fn source_slice(&self) -> Slice {
        Slice::from(self.source_idx..self.end_source_idx())
    }

    pub fn is_source_only_slice(&self) -> bool {
        // Based on its slice_type, does it only appear in the *source*?
        // There are some slice types which are automatically source only.
        // There are *also* some which are source only because they render
        // to an empty string.
        // TODO: should any new logic go here?
        match self.slice_type.as_str() {
            "comment" | "block_end" | "block_start" | "block_mid" => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, FromPyObject, IntoPyObject)]
pub struct TemplatedFileSlice {
    pub slice_type: String,
    pub source_slice: Slice,
    pub templated_slice: Slice,
}

impl TemplatedFileSlice {
    pub fn new(slice_type: String, source_slice: Slice, templated_slice: Slice) -> Self {
        TemplatedFileSlice {
            slice_type,
            source_slice,
            templated_slice,
        }
    }
}

#[pyclass]
#[derive(Debug, PartialEq, FromPyObject)]
pub struct TemplatedFile {
    #[pyo3(get)]
    pub source_str: String,
    #[pyo3(get)]
    pub fname: String,
    #[pyo3(get)]
    pub templated_str: String,
    #[pyo3(get)]
    pub sliced_file: Vec<TemplatedFileSlice>,
    #[pyo3(get)]
    pub raw_sliced: Vec<RawFileSlice>,
    #[pyo3(get)]
    _source_newlines: Vec<usize>,
    #[pyo3(get)]
    _templated_newlines: Vec<usize>,
}

#[pymethods]
impl TemplatedFile {
    #[new]
    #[pyo3(signature = (source_str, fname, templated_str=None, sliced_file=None, raw_sliced=None))]
    pub fn new(
        source_str: String,
        fname: String,
        templated_str: Option<String>,
        sliced_file: Option<Vec<TemplatedFileSlice>>,
        raw_sliced: Option<Vec<RawFileSlice>>,
    ) -> Self {
        let templated_str_in = templated_str.clone().unwrap_or(source_str.clone());

        let (sliced_file, raw_sliced): (Vec<TemplatedFileSlice>, Vec<RawFileSlice>) =
            if let Some(s_file) = sliced_file.clone() {
                // If sliced_file is provided, ensure raw_sliced is also provided.
                match raw_sliced.clone() {
                    Some(r_file) => (s_file, r_file),
                    None => panic!("Templated file was sliced, but no raw slices provided."),
                }
            } else {
                // If sliced_file is None, ensure the file is not templated.
                if templated_str_in != source_str.clone() {
                    panic!("Cannot instantiate a templated file without slices!");
                }
                // Create "literal" slices for both sliced_file and raw_sliced.
                (
                    vec![TemplatedFileSlice {
                        slice_type: "literal".to_string(),
                        source_slice: Slice::from(0..source_str.len()),
                        templated_slice: Slice::from(0..source_str.len()),
                    }],
                    vec![RawFileSlice {
                        raw: source_str.clone(),
                        slice_type: "literal".to_string(),
                        source_idx: 0,
                        block_idx: 0,
                        tag: None,
                    }],
                )
            };

        let _source_newlines = iter_indices_of_newlines(&source_str).collect();
        let _templated_newlines = iter_indices_of_newlines(&templated_str_in).collect();

        // Consistency check raw string and slices.
        let mut pos = 0;
        for rfs in &raw_sliced {
            assert_eq!(
                rfs.source_idx, pos,
                "TemplatedFile. Consistency fail on running source length: {} != {}",
                pos, rfs.source_idx
            );
            pos += rfs.raw.len();
        }
        assert_eq!(
            pos,
            source_str.len(),
            "TemplatedFile. Consistency fail on total source length: {} != {}",
            pos,
            source_str.len()
        );

        // Consistency check templated string and slices.
        let mut previous_slice: Option<&TemplatedFileSlice> = None;
        for tfs in sliced_file.iter() {
            if let Some(prev_slice) = previous_slice {
                if tfs.templated_slice.start != prev_slice.templated_slice.stop {
                    panic!(
                        "Templated slices found to be non-contiguous. \
                        {:?} (starting {}) does not follow {:?} (starting {})",
                        tfs.templated_slice,
                        &templated_str_in[tfs.templated_slice.start..tfs.templated_slice.stop],
                        prev_slice.templated_slice,
                        &templated_str_in
                            [prev_slice.templated_slice.start..prev_slice.templated_slice.stop],
                    );
                }
            } else if tfs.templated_slice.start != 0 {
                panic!(
                    "First Templated slice not started at index 0 (found slice {:?})",
                    tfs.templated_slice
                );
            }
            previous_slice = Some(tfs);
        }

        if let Some(tfs) = sliced_file.last() {
            if let Some(templated) = templated_str.as_ref() {
                if tfs.templated_slice.stop != templated.len() {
                    panic!(
                        "Length of templated file mismatch with final slice: {} != {}.",
                        templated.len(),
                        tfs.templated_slice.stop
                    );
                }
            }
        }

        TemplatedFile {
            source_str,
            fname,
            templated_str: templated_str_in.to_string(),
            sliced_file,
            raw_sliced,
            _source_newlines,
            _templated_newlines,
        }
    }

    pub fn raw_slices_spanning_source_slice(&self, source_slice: Slice) -> Vec<RawFileSlice> {
        // Special case: The source_slice is at the end of the file.
        let last_raw_slice = self.raw_sliced.last().unwrap();
        if source_slice.start >= last_raw_slice.source_idx + last_raw_slice.raw.len() {
            return vec![];
        }
        // First find the start index
        let mut raw_slice_idx = 0;
        // Move the raw pointer forward to the start of this patch
        while raw_slice_idx + 1 < self.raw_sliced.len()
            && self.raw_sliced[raw_slice_idx + 1].source_idx <= source_slice.start
        {
            raw_slice_idx += 1;
        }
        // Find slice index of the end of this patch.
        let mut slice_span = 1;
        while raw_slice_idx + slice_span < self.raw_sliced.len()
            && self.raw_sliced[raw_slice_idx + slice_span].source_idx < source_slice.stop
        {
            slice_span += 1;
        }
        // Return the raw slices:
        self.raw_sliced[raw_slice_idx..raw_slice_idx + slice_span].to_vec()
    }

    /// Convert a template
    pub fn templated_slice_to_source_slice(&self, template_slice: Slice) -> Slice {
        // If there are no sliced files, return the template slice
        if self.sliced_file.is_empty() {
            return template_slice;
        }

        // Find the indices of sliced files touching the template slice start position
        let (ts_start_sf_start, ts_start_sf_stop) =
            self.find_slice_indices_of_templated_pos(template_slice.start, true, None);

        // Get the sliced files within the found indices
        let ts_start_subsliced_file = &self.sliced_file[ts_start_sf_start..ts_start_sf_stop];

        // Work out the insertion point
        let insertion_point =
            self.get_insertion_point(ts_start_subsliced_file, template_slice.start);

        // Zero length slice
        if template_slice.start == template_slice.stop {
            // Is it on a join?
            if let Some(insertion_point) = insertion_point {
                return Slice::from(insertion_point..insertion_point);
            }
            // It's within a segment
            else if !ts_start_subsliced_file.is_empty()
                && ts_start_subsliced_file[0].slice_type == "literal"
            {
                let offset =
                    template_slice.start - ts_start_subsliced_file[0].templated_slice.start;
                return Slice::from(
                    ts_start_subsliced_file[0].source_slice.start + offset
                        ..(ts_start_subsliced_file[0].source_slice.start + offset),
                );
            } else {
                panic!("Attempting a single length slice within a templated section!");
            }
        }

        // Otherwise it's a slice with length.
        // Use a non inclusive match to get the end point.
        // Find the indices of sliced files touching the template slice end position
        let (ts_stop_sf_start, ts_stop_sf_stop) =
            self.find_slice_indices_of_templated_pos(template_slice.stop, false, None);

        // Update starting position based on insertion point
        let mut ts_start_sf_start = ts_start_sf_start;
        if insertion_point.is_some() {
            for elem in self.sliced_file.iter().skip(ts_start_sf_start) {
                if elem.source_slice.start != insertion_point.unwrap() {
                    ts_start_sf_start += 1;
                } else {
                    break;
                }
            }
        }

        // Collect relevant subslices
        let subslices = &self.sliced_file
            [min(ts_start_sf_start, ts_stop_sf_start)..max(ts_start_sf_stop, ts_stop_sf_stop)];

        if ts_start_sf_start == ts_start_sf_stop {
            if ts_start_sf_start > self.sliced_file.len() {
                panic!("Starting position higher than sliced file position");
            }
            if ts_start_sf_start < self.sliced_file.len() {
                return self.sliced_file[1].source_slice.clone();
            } else {
                return self.sliced_file.last().unwrap().source_slice.clone();
            }
        }

        // Define start and stop slices
        let start_slices = &self.sliced_file[ts_start_sf_start..ts_start_sf_stop];
        let stop_slices = if ts_stop_sf_start == ts_stop_sf_stop {
            vec![self.sliced_file[ts_stop_sf_start].clone()]
        } else {
            self.sliced_file[ts_stop_sf_start..ts_stop_sf_stop].to_vec()
        };

        // If it's a literal segment then we can get the exact position
        // otherwise we're greedy.

        // Start.
        let source_start = if insertion_point.is_some() {
            insertion_point.unwrap()
        } else if start_slices[0].slice_type == "literal" {
            let offset = template_slice.start - start_slices[0].templated_slice.start;
            start_slices[0].source_slice.start + offset
        } else {
            start_slices[0].source_slice.start
        };

        // Stop.
        let source_stop = if stop_slices.last().unwrap().slice_type == "literal" {
            let offset = stop_slices.last().unwrap().templated_slice.stop - template_slice.stop;
            stop_slices.last().unwrap().source_slice.stop - offset
        } else {
            stop_slices.last().unwrap().source_slice.stop
        };

        // Does this slice go backward?
        let (source_start, source_stop) = if source_start > source_stop {
            // If this happens, it's because one was templated and
            // the other isn't, or because a loop means that the segments
            // are in a different order.

            // Take the widest possible span in this case.
            (
                subslices
                    .iter()
                    .map(|elem| elem.source_slice.start)
                    .min()
                    .unwrap(),
                subslices
                    .iter()
                    .map(|elem| elem.source_slice.stop)
                    .max()
                    .unwrap(),
            )
        } else {
            (source_start, source_stop)
        };

        Slice::from(source_start..source_stop)
    }
}

impl From<String> for TemplatedFile {
    fn from(raw: String) -> TemplatedFile {
        TemplatedFile::new(raw, String::from("<string>"), None, None, None)
    }
}

impl TemplatedFile {
    pub fn get_line_pos_of_char_pos(&self, char_pos: usize, source: bool) -> (usize, usize) {
        let ref_str = if source {
            &self._source_newlines
        } else {
            &self._templated_newlines
        };

        let nl_idx = ref_str.binary_search(&char_pos).unwrap_or_else(|x| x);

        if nl_idx > 0 {
            (nl_idx + 1, char_pos - ref_str[nl_idx - 1])
        } else {
            (1, char_pos + 1)
        }
    }

    /// Find a subset of the sliced file which touch this point.
    ///
    /// NB: the last_idx is exclusive, as the intent is to use this as a slice.
    fn find_slice_indices_of_templated_pos(
        &self,
        templated_pos: usize,
        inclusive: bool,
        start_idx: Option<usize>,
    ) -> (usize, usize) {
        let start_idx = start_idx.unwrap_or(0);
        let mut first_idx = None;
        let mut last_idx = start_idx;
        let mut found = false;

        // Work through the sliced file, starting at the start_idx if given
        // as an optimisation hint. The sliced_file is a list of TemplatedFileSlice
        // which reference parts of the templated file and where they exist in the
        // source.
        for (idx, elem) in self.sliced_file.iter().enumerate().skip(start_idx) {
            last_idx = idx + start_idx;
            if elem.templated_slice.stop >= templated_pos {
                if first_idx.is_none() {
                    first_idx = Some(idx + start_idx);
                }
                if elem.templated_slice.start > templated_pos
                    || (!inclusive && elem.templated_slice.start >= templated_pos)
                {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            last_idx += 1;
        }

        if first_idx.is_none() {
            panic!("Position Not Found");
        }

        (first_idx.unwrap(), last_idx)
    }

    pub fn get_insertion_point(
        &self,
        subsliced_file: &[TemplatedFileSlice],
        template_slice_start: usize,
    ) -> Option<usize> {
        let mut point = None;
        let mut insertion_point = None;
        for elem in subsliced_file {
            if elem.templated_slice.start == template_slice_start {
                point = Some(elem.source_slice.start);
                if insertion_point.is_none() || point.unwrap() < insertion_point.unwrap() {
                    insertion_point = point;
                }
            }
            if elem.templated_slice.stop == template_slice_start {
                point = Some(elem.source_slice.stop);
                if insertion_point.is_none() || point.unwrap() < insertion_point.unwrap() {
                    insertion_point = point;
                }
            }
        }
        insertion_point
    }

    pub fn is_source_slice_literal(&self, source_slice: &Slice) -> bool {
        // No sliced file? Everything is literal
        if self.raw_sliced.is_empty() {
            return true;
        }

        // Zero length slice. It's a literal, because it's definitely not templated.
        if source_slice.start == source_slice.stop {
            return true;
        }

        let mut is_literal = true;
        for raw_slice in &self.raw_sliced {
            // Reset if we find a literal and we're up to the start
            // otherwise set false.
            if raw_slice.source_idx <= source_slice.start {
                is_literal = raw_slice.slice_type == "literal";
            } else if raw_slice.source_idx >= source_slice.stop {
                // We've gone past the end. Break and Return.
                break;
            } else {
                // We're in the middle. Check type
                if raw_slice.slice_type != "literal" {
                    is_literal = false;
                }
            }
        }

        is_literal
    }

    pub fn source_only_slices(&self) -> Vec<RawFileSlice> {
        self.raw_sliced
            .iter()
            .filter(|elem| elem.is_source_only_slice())
            .cloned()
            .collect()
    }

    pub fn source_position_dict_from_slice(&self, source_slice: &Slice) -> HashMap<String, usize> {
        let start = self.get_line_pos_of_char_pos(source_slice.start, true);
        let stop = self.get_line_pos_of_char_pos(source_slice.stop, true);
        let mut dict = HashMap::new();
        dict.insert("start_line_no".to_string(), start.0);
        dict.insert("start_line_pos".to_string(), start.1);
        dict.insert("start_file_pos".to_string(), source_slice.start);
        dict.insert("end_line_no".to_string(), stop.0);
        dict.insert("end_line_pos".to_string(), stop.1);
        dict.insert("end_file_pos".to_string(), source_slice.stop);
        dict
    }
}

impl std::fmt::Display for TemplatedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.templated_str)
    }
}

fn iter_indices_of_newlines(raw_str: &str) -> impl Iterator<Item = usize> + '_ {
    raw_str.match_indices('\n').map(|(idx, _)| idx)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    #[derive(Clone)]
    struct TestFileArgs {
        fname: String,
        sliced_file: Vec<TemplatedFileSlice>,
        templated_str: Option<String>,
        raw_sliced: Vec<RawFileSlice>,
        source_str: String,
    }

    static ref SIMPLE_FILE_KWARGS: Lazy<TestFileArgs> = Lazy::new(|| {
        TestFileArgs {
            fname: "test.sql".to_string(),
            source_str: "01234\n6789{{foo}}fo\nbarss".to_string(),
            templated_str: Some("01234\n6789x\nfo\nbarss".to_string()),
            sliced_file: vec![
                TemplatedFileSlice::new("literal".to_string(), 0..10, 0..10),
                TemplatedFileSlice::new("templated".to_string(), 10..17, 10..12),
                TemplatedFileSlice::new("literal".to_string(), 17..25, 12..20),
            ],
            raw_sliced: vec![
                RawFileSlice::new("x".repeat(10), "literal".to_string(), 0),
                RawFileSlice::new("x".repeat(7), "templated".to_string(), 10),
                RawFileSlice::new("x".repeat(8), "literal".to_string(), 17)
            ],
        }
    });
        static ref COMPLEX_RAW_SLICED: Vec<RawFileSlice> = vec![
            RawFileSlice::new("x".repeat(13), "literal".to_string(), 0),
            RawFileSlice::new("x".repeat(16), "comment".to_string(), 13),
            RawFileSlice::new("x".repeat(15), "literal".to_string(), 29),
            RawFileSlice::new("x".repeat(24), "block_start".to_string(), 44),
            RawFileSlice::new("x".repeat(13), "literal".to_string(), 68),
            RawFileSlice::new("x".repeat(5), "templated".to_string(), 81),
            RawFileSlice::new("x".repeat(24), "literal".to_string(), 86),
            RawFileSlice::new("x".repeat(13), "templated".to_string(), 110),
            RawFileSlice::new("x".repeat(9), "literal".to_string(), 123),
            RawFileSlice::new("x".repeat(12), "block_end".to_string(), 132),
            RawFileSlice::new("x".repeat(11), "literal".to_string(), 144),
            RawFileSlice::new("x".repeat(24), "block_start".to_string(), 155),
            RawFileSlice::new("x".repeat(10), "literal".to_string(), 179),
            RawFileSlice::new("x".repeat(5), "templated".to_string(), 189),
            RawFileSlice::new("x".repeat(9), "literal".to_string(), 194),
            RawFileSlice::new("x".repeat(12), "block_end".to_string(), 203),
            RawFileSlice::new("x".repeat(15), "literal".to_string(), 215),
        ];
        static ref COMPLEX_FILE_KWARGS: TestFileArgs = {
            let raw_sliced = &*COMPLEX_RAW_SLICED;

            TestFileArgs {
                fname: "test.sql".to_string(),
                sliced_file: vec![
                    TemplatedFileSlice::new("literal".to_string(), 0..13, 0..13),
                    TemplatedFileSlice::new("comment".to_string(), 13..29, 13..13),
                    TemplatedFileSlice::new("literal".to_string(), 29..44, 13..28),
                    TemplatedFileSlice::new("block_start".to_string(), 44..68, 28..28),
                    TemplatedFileSlice::new("literal".to_string(), 68..81, 28..41),
                    TemplatedFileSlice::new("templated".to_string(), 81..86, 41..42),
                    TemplatedFileSlice::new("literal".to_string(), 86..110, 42..66),
                    TemplatedFileSlice::new("templated".to_string(), 68..86, 66..76),
                    TemplatedFileSlice::new("literal".to_string(), 68..81, 76..89),
                    TemplatedFileSlice::new("templated".to_string(), 81..86, 89..90),
                    TemplatedFileSlice::new("literal".to_string(), 86..110, 90..114),
                    TemplatedFileSlice::new("templated".to_string(), 68..86, 114..125),
                    TemplatedFileSlice::new("literal".to_string(), 68..81, 125..138),
                    TemplatedFileSlice::new("templated".to_string(), 81..86, 138..139),
                    TemplatedFileSlice::new("literal".to_string(), 86..110, 139..163),
                    TemplatedFileSlice::new("templated".to_string(), 110..123, 163..166),
                    TemplatedFileSlice::new("literal".to_string(), 123..132, 166..175),
                    TemplatedFileSlice::new("block_end".to_string(), 132..144, 175..175),
                    TemplatedFileSlice::new("literal".to_string(), 144..155, 175..186),
                    TemplatedFileSlice::new("block_start".to_string(), 155..179, 186..186),
                    TemplatedFileSlice::new("literal".to_string(), 179..189, 186..196),
                    TemplatedFileSlice::new("templated".to_string(), 189..194, 196..197),
                    TemplatedFileSlice::new("literal".to_string(), 194..203, 197..206),
                    TemplatedFileSlice::new("literal".to_string(), 179..189, 206..216),
                    TemplatedFileSlice::new("templated".to_string(), 189..194, 216..217),
                    TemplatedFileSlice::new("literal".to_string(), 194..203, 217..226),
                    TemplatedFileSlice::new("literal".to_string(), 179..189, 226..236),
                    TemplatedFileSlice::new("templated".to_string(), 189..194, 236..237),
                    TemplatedFileSlice::new("literal".to_string(), 194..203, 237..246),
                    TemplatedFileSlice::new("block_end".to_string(), 203..215, 246..246),
                    TemplatedFileSlice::new("literal".to_string(), 215..230, 246..261),
                ],
                raw_sliced: raw_sliced.clone(),
                source_str: raw_sliced
                    .iter()
                    .map(|slice| &*slice.raw)
                    .collect::<String>(),
                templated_str: None,
            }
        };
    }

    #[test]
    fn test_indices_of_newlines() {
        let test_cases = vec![
            ("", vec![]),
            ("foo", vec![]),
            ("foo\nbar", vec![3]),
            ("\nfoo\n\nbar\nfoo\n\nbar\n", vec![0, 4, 5, 9, 13, 14, 18]),
        ];

        for (raw_str, positions) in test_cases {
            assert_eq!(
                iter_indices_of_newlines(raw_str).collect::<Vec<_>>(),
                positions
            );
        }
    }

    #[test]
    fn test_templated_file_find_slice_indices_of_templated_pos() {
        let complex_file_kwargs = &*COMPLEX_FILE_KWARGS;
        let simple_file_kwargs = &*SIMPLE_FILE_KWARGS;
        let test_cases: Vec<(usize, bool, TestFileArgs, usize, usize)> = vec![
            (100, true, complex_file_kwargs.clone(), 10, 11),
            (13, true, complex_file_kwargs.clone(), 0, 3),
            (28, true, complex_file_kwargs.clone(), 2, 5),
            // Check end slicing.
            (12, true, simple_file_kwargs.clone(), 1, 3),
            (20, true, simple_file_kwargs.clone(), 2, 3),
            // Check inclusivity
            (13, false, complex_file_kwargs.clone(), 0, 1),
        ];

        for (templated_position, inclusive, tf_kwargs, sliced_idx_start, sliced_idx_stop) in
            test_cases
        {
            let file = TemplatedFile::new(
                tf_kwargs.source_str,
                tf_kwargs.fname,
                tf_kwargs.templated_str,
                Some(tf_kwargs.sliced_file),
                Some(tf_kwargs.raw_sliced),
            );
            let (res_start, res_stop) =
                file.find_slice_indices_of_templated_pos(templated_position, None, inclusive);
            assert_eq!(res_start, sliced_idx_start);
            assert_eq!(res_stop, sliced_idx_stop);
        }
    }

    #[test]
    fn test_templated_file_templated_slice_to_source_slice() {
        let complex_file_kwargs = &*COMPLEX_FILE_KWARGS;
        let simple_file_kwargs = &*SIMPLE_FILE_KWARGS;
        let test_cases: Vec<(
            String,
            String,
            Option<String>,
            Slice,
            Slice,
            bool,
            Vec<TemplatedFileSlice>,
            Vec<RawFileSlice>,
        )> = vec![
            // Simple example
            (
                "foo.sql".to_string(),
                "x".repeat(20).to_string(),
                None,
                5..10,
                5..10,
                true,
                vec![TemplatedFileSlice::new("literal".to_string(), 0..20, 0..20)],
                vec![RawFileSlice::new("x".repeat(20), "literal".to_string(), 0)],
            ),
            // Trimming the end of a literal (with things that follow).
            (
                "test.sql".to_string(),
                complex_file_kwargs.source_str.clone(),
                None,
                10..13,
                10..13,
                true,
                complex_file_kwargs.sliced_file.clone(),
                complex_file_kwargs.raw_sliced.clone(),
            ),
            // Unrealistic, but should still work
            (
                "foo.sql".to_string(),
                "x".repeat(70).to_string(),
                None,
                5..10,
                55..60,
                true,
                vec![TemplatedFileSlice::new(
                    "literal".to_string(),
                    50..70,
                    0..20,
                )],
                vec![
                    RawFileSlice::new("x".repeat(50), "literal".to_string(), 0),
                    RawFileSlice::new("x".repeat(20), "literal".to_string(), 50),
                ],
            ),
            // Spanning a template
            (
                "test.sql".to_string(),
                "01234\n6789{{foo}}fo\nbarss".to_string(),
                Some("01234\n6789x\nfo\nbarss".to_string()),
                5..15,
                5..20,
                false,
                vec![
                    TemplatedFileSlice::new("literal".to_string(), 0..10, 0..10),
                    TemplatedFileSlice::new("templated".to_string(), 10..17, 10..12),
                    TemplatedFileSlice::new("literal".to_string(), 17..25, 12..20),
                ],
                vec![
                    RawFileSlice::new("x".repeat(10), "literal".to_string(), 0),
                    RawFileSlice::new("x".repeat(7), "templated".to_string(), 10),
                    RawFileSlice::new("x".repeat(8), "literal".to_string(), 17),
                ],
            ),
            // Handling templated
            (
                simple_file_kwargs.clone().fname,
                simple_file_kwargs.clone().source_str,
                simple_file_kwargs.clone().templated_str,
                5..15,
                0..25,
                false,
                simple_file_kwargs
                    .sliced_file
                    .iter()
                    .map(|slc| {
                        TemplatedFileSlice::new(
                            "templated".to_string(),
                            slc.source_slice.clone(),
                            slc.templated_slice.clone(),
                        )
                    })
                    .collect(),
                simple_file_kwargs
                    .raw_sliced
                    .iter()
                    .map(|slc| {
                        RawFileSlice::new(
                            slc.raw.clone(),
                            "templated".to_string(),
                            slc.source_idx.clone(),
                        )
                    })
                    .collect(),
            ),
            // Handling single length slices
            (
                simple_file_kwargs.fname.clone(),
                simple_file_kwargs.source_str.clone(),
                simple_file_kwargs.templated_str.clone(),
                10..10,
                10..10,
                true,
                simple_file_kwargs.sliced_file.clone(),
                simple_file_kwargs.raw_sliced.clone(),
            ),
            (
                simple_file_kwargs.fname.clone(),
                simple_file_kwargs.source_str.clone(),
                simple_file_kwargs.templated_str.clone(),
                12..12,
                17..17,
                true,
                simple_file_kwargs.sliced_file.clone(),
                simple_file_kwargs.raw_sliced.clone(),
            ),
            // Dealing with single length elements
            {
                let extended_source_str = simple_file_kwargs.source_str.clone() + &"x".repeat(10);
                let extended_sliced_file: Vec<TemplatedFileSlice> = {
                    let mut sliced_file = simple_file_kwargs.sliced_file.clone();
                    sliced_file.push(TemplatedFileSlice::new(
                        "comment".to_string(),
                        25..35,
                        20..20,
                    ));
                    sliced_file
                };
                let extended_raw_sliced: Vec<RawFileSlice> = {
                    let mut raw_sliced = simple_file_kwargs.raw_sliced.clone();
                    raw_sliced.push(RawFileSlice::new("x".repeat(10), "comment".to_string(), 25));
                    raw_sliced
                };
                (
                    "foo.sql".to_string(),
                    extended_source_str,
                    None,
                    20..20,
                    25..25,
                    true,
                    extended_sliced_file,
                    extended_raw_sliced,
                )
            },
            // Just more test coverage
            (
                complex_file_kwargs.fname.clone(),
                complex_file_kwargs.source_str.clone(),
                complex_file_kwargs.templated_str.clone(),
                43..43,
                87..87,
                true,
                complex_file_kwargs.sliced_file.clone(),
                complex_file_kwargs.raw_sliced.clone(),
            ),
            (
                complex_file_kwargs.fname.clone(),
                complex_file_kwargs.source_str.clone(),
                complex_file_kwargs.templated_str.clone(),
                13..13,
                13..13,
                true,
                complex_file_kwargs.sliced_file.clone(),
                complex_file_kwargs.raw_sliced.clone(),
            ),
            (
                complex_file_kwargs.fname.clone(),
                complex_file_kwargs.source_str.clone(),
                complex_file_kwargs.templated_str.clone(),
                186..186,
                155..155,
                true,
                complex_file_kwargs.sliced_file.clone(),
                complex_file_kwargs.raw_sliced.clone(),
            ),
            // // Backward slicing.
            (
                complex_file_kwargs.fname.clone(),
                complex_file_kwargs.source_str.clone(),
                complex_file_kwargs.templated_str.clone(),
                100..130,
                68..110,
                false,
                complex_file_kwargs.sliced_file.clone(),
                complex_file_kwargs.raw_sliced.clone(),
            ),
        ];

        for (
            fname,
            source_str,
            templated_str,
            in_slice,
            out_slice,
            is_literal,
            sliced_file,
            raw_sliced,
        ) in test_cases
        {
            let file = TemplatedFile::new(
                source_str,
                fname,
                templated_str,
                Some(sliced_file),
                Some(raw_sliced),
            );
            let source_slice = file.templated_slice_to_source_slice(in_slice.clone());
            let literal_test = file.is_source_slice_literal(source_slice.clone());
            assert_eq!(
                (is_literal, source_slice.clone()),
                (literal_test, out_slice)
            );
        }
    }

    // #[test]
    // fn test_raw_templater() {
    //     let mut t = RawTemplater::new();
    //     let instr = "SELECT * FROM {{blah}}";
    //     let tf = t.process(instr, "test");
    //     assert_eq!(instr, &tf.source_str);
    // }
}
 */
