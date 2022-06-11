#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LibError {
    ParseError,
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibError::ParseError => f.write_str("Parse error"),
        }
    }
}

impl std::error::Error for LibError {}
