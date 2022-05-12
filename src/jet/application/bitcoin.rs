use crate::jet;
use crate::jet::{Application, JetNode};

/// Bitcoin application
pub struct Bitcoin {}

impl Application for Bitcoin {
    fn decode_jet(code: u8) -> Result<&'static JetNode, &'static str> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            200 => Ok(&jet::bitcoin::VERSION),
            _ => Err("Unknown Bitcoin jet!"),
        }
    }

    fn encode_jet(jet: &JetNode) -> Result<u8, &'static str> {
        match jet.name {
            "adder32" => Ok(100),
            "version" => Ok(200),
            _ => Err("Unknown Bitcoin jet!"),
        }
    }

    /// Insane code that absolutely requires feature `bitcoin` to compile.
    #[cfg(feature = "bitcoin")]
    fn exec_jet(jet: &JetNode) -> Result<u64, &'static str> {
        match jet.name {
            "adder32" => jet::application::Core::exec_jet(jet),
            "version" => Ok(1337), // wow!
            _ => Err("Unknown Bitcoin jet!"),
        }
    }

    #[cfg(not(feature = "bitcoin"))]
    fn exec_jet(_jet: &JetNode) -> Result<u64, &'static str> {
        Err("Cannot execute Bitcoin jet due to missing Bitcoin feature!")
    }
}
