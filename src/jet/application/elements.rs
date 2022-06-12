use crate::error::LibError;
use crate::jet;
use crate::jet::application::core::CoreError;
use crate::jet::elements::ElementsJetName;
use crate::jet::{AppError, Application, JetNode};

/// Elements application
#[derive(Debug)]
pub struct Elements {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ElementsError {
    MissingFeature,
}

impl std::fmt::Display for ElementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementsError::MissingFeature => f.write_str("Missing Elements feature"),
        }
    }
}

impl std::error::Error for ElementsError {}
impl AppError for ElementsError {}

impl From<CoreError> for ElementsError {
    fn from(e: CoreError) -> Self {
        match e {}
    }
}

impl Application for Elements {
    type Error = ElementsError;
    type JetName = ElementsJetName;

    fn decode_jet(code: u8) -> Result<&'static JetNode<Self>, LibError> {
        match code {
            100 => Ok(&jet::elements::ADDER32),
            200 => Ok(&jet::elements::VERSION),
            _ => Err(LibError::ParseError),
        }
    }

    fn encode_jet(jet: &JetNode<Self>) -> u8 {
        match jet.name {
            ElementsJetName::Adder => 100,
            ElementsJetName::Version => 200,
        }
    }

    /// Insane code that absolutely requires feature `elements` to compile.
    #[cfg(feature = "elements")]
    fn exec_jet(jet: &JetNode<Self>) -> Result<u64, Self::Error> {
        match jet.name {
            ElementsJetName::Adder => Ok(jet::application::Core::exec_jet(&jet::core::ADDER32)?),
            ElementsJetName::Version => Ok(31337), // rad!
        }
    }

    #[cfg(not(feature = "elements"))]
    fn exec_jet(_jet: &JetNode<Self>) -> Result<u64, Self::Error> {
        Err(ElementsError::MissingFeature)
    }
}
