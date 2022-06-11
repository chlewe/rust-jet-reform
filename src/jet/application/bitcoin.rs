use crate::error::LibError;
use crate::jet;
use crate::jet::application::core::CoreError;
use crate::jet::{AppError, Application, JetNode};

/// Bitcoin application
pub struct Bitcoin {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum BitcoinError {
    MissingFeature,
}

impl std::fmt::Display for BitcoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitcoinError::MissingFeature => f.write_str("Missing Bitcoin feature"),
        }
    }
}

impl std::error::Error for BitcoinError {}
impl AppError for BitcoinError {}

impl From<CoreError> for BitcoinError {
    fn from(e: CoreError) -> Self {
        match e {}
    }
}

impl Application for Bitcoin {
    type Error = BitcoinError;

    fn decode_jet(code: u8) -> Result<&'static JetNode, LibError> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            200 => Ok(&jet::bitcoin::VERSION),
            _ => Err(LibError::ParseError),
        }
    }

    fn encode_jet(jet: &JetNode) -> u8 {
        match jet.name {
            "adder32" => 100,
            "version" => 200,
            _ => panic!("Unknown Bitcoin jet!"),
        }
    }

    /// Insane code that absolutely requires feature `bitcoin` to compile.
    #[cfg(feature = "bitcoin")]
    fn exec_jet(jet: &JetNode) -> Result<u64, Self::Error> {
        match jet.name {
            "adder32" => Ok(jet::application::Core::exec_jet(jet)?),
            "version" => Ok(1337), // wow!
            _ => panic!("Unknown Bitcoin jet!"),
        }
    }

    #[cfg(not(feature = "bitcoin"))]
    fn exec_jet(_jet: &JetNode) -> Result<u64, Self::Error> {
        Err(BitcoinError::MissingFeature)
    }
}
