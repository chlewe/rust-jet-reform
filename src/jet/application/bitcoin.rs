use crate::error::LibError;
use crate::jet;
use crate::jet::application::core::CoreError;
use crate::jet::bitcoin::BitcoinJetName;
use crate::jet::{AppError, Application, JetNode};

/// Bitcoin application
#[derive(Debug)]
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
    type JetName = BitcoinJetName;

    fn decode_jet(code: u8) -> Result<&'static JetNode<Self>, LibError> {
        match code {
            100 => Ok(&jet::bitcoin::ADDER32),
            200 => Ok(&jet::bitcoin::VERSION),
            _ => Err(LibError::ParseError),
        }
    }

    fn encode_jet(jet: &JetNode<Self>) -> u8 {
        match jet.name {
            BitcoinJetName::Adder => 100,
            BitcoinJetName::Version => 200,
        }
    }

    /// Insane code that absolutely requires feature `bitcoin` to compile.
    #[cfg(feature = "bitcoin")]
    fn exec_jet(jet: &JetNode<Self>) -> Result<u64, Self::Error> {
        match jet.name {
            BitcoinJetName::Adder => Ok(jet::application::Core::exec_jet(&jet::core::ADDER32)?),
            BitcoinJetName::Version => Ok(1337), // wow!
        }
    }

    #[cfg(not(feature = "bitcoin"))]
    fn exec_jet(_jet: &JetNode<Self>) -> Result<u64, Self::Error> {
        Err(BitcoinError::MissingFeature)
    }
}
