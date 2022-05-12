use crate::jet;
use crate::jet::{Application, JetNode};

/// Elements application
pub struct Elements {}

impl Application for Elements {
    fn decode_jet(code: u8) -> Result<&'static JetNode, &'static str> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            200 => Ok(&jet::bitcoin::VERSION),
            _ => Err("Unknown Elements jet!"),
        }
    }

    fn encode_jet(jet: &JetNode) -> Result<u8, &'static str> {
        match jet.name {
            "adder32" => Ok(100),
            "version" => Ok(200),
            _ => Err("Unknown Elements jet!"),
        }
    }

    /// Insane code that absolutely requires feature `elements` to compile.
    #[cfg(feature = "elements")]
    fn exec_jet(jet: &JetNode) -> Result<u64, &'static str> {
        match jet.name {
            "adder32" => jet::application::Core::exec_jet(jet),
            "version" => Ok(31337), // rad!
            _ => Err("Unknown Elements jet!"),
        }
    }

    #[cfg(not(feature = "elements"))]
    fn exec_jet(_jet: &JetNode) -> Result<u64, &'static str> {
        Err("Cannot execute Elements jet due to missing Elements feature!")
    }
}
