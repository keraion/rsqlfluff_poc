use super::Token;

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}