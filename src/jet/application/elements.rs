use crate::error::LibError;
use crate::jet;
use crate::jet::application::core::CoreError;
use crate::jet::{AppError, Application, JetNode};

/// Elements application
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
            _ => panic!("Unknown Elements jet!"),
        }
    }

    /// Insane code that absolutely requires feature `elements` to compile.
    #[cfg(feature = "elements")]
    fn exec_jet(jet: &JetNode) -> Result<u64, Self::Error> {
        match jet.name {
            "adder32" => Ok(jet::application::Core::exec_jet(jet)?),
            "version" => Ok(31337), // rad!
            _ => panic!("Unknown Elements jet!"),
        }
    }

    #[cfg(not(feature = "elements"))]
    fn exec_jet(_jet: &JetNode) -> Result<u64, Self::Error> {
        Err(ElementsError::MissingFeature)
    }
}
