use crate::error::LibError;
use crate::jet;
use crate::jet::core::CoreJetName;
use crate::jet::{AppError, Application, JetNode};

/// Core application, i.e., nothing enabled
#[derive(Debug)]
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
    type JetName = CoreJetName;

    fn decode_jet(code: u8) -> Result<&'static JetNode<Self>, LibError> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            _ => Err(LibError::ParseError),
        }
    }

    fn encode_jet(jet: &JetNode<Self>) -> u8 {
        match jet.name {
            CoreJetName::Adder => 100,
        }
    }

    /// Boring code that always compiles.
    fn exec_jet(jet: &JetNode<Self>) -> Result<u64, Self::Error> {
        match jet.name {
            CoreJetName::Adder => Ok(0), // zzz...
        }
    }
}
