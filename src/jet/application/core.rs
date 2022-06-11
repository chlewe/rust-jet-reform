use crate::error::LibError;
use crate::jet;
use crate::jet::{AppError, Application, JetNode};

/// Core application, i.e., nothing enabled
pub struct Core {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum CoreError {}

impl std::fmt::Display for CoreError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for CoreError {}
impl AppError for CoreError {}

impl Application for Core {
    type Error = CoreError;

    fn decode_jet(code: u8) -> Result<&'static JetNode, LibError> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            _ => Err(LibError::ParseError),
        }
    }

    fn encode_jet(jet: &JetNode) -> u8 {
        match jet.name {
            "adder32" => 100,
            _ => panic!("Unknown Core jet!"),
        }
    }

    /// Boring code that always compiles.
    fn exec_jet(jet: &JetNode) -> Result<u64, Self::Error> {
        match jet.name {
            "adder32" => Ok(0), // zzz...
            _ => panic!("Unknown Core jet!"),
        }
    }
}
