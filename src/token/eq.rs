use std::hash::Hash;

use super::Token;

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
            || (self.token_types == other.token_types
                && self.raw == other.raw
                && self.pos_marker.is_some()
                && other.pos_marker.is_some()
                && self.pos_marker == other.pos_marker)
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token_types.hash(state);
        self.raw.hash(state);
        self.pos_marker.as_ref().map(|p| p.working_loc().hash(state));
    }
}
