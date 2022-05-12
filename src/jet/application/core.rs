use crate::jet;
use crate::jet::{Application, JetNode};

/// Core application, i.e., nothing enabled
pub struct Core {}

impl Application for Core {
    fn decode_jet(code: u8) -> Result<&'static JetNode, &'static str> {
        match code {
            100 => Ok(&jet::core::ADDER32),
            _ => Err("Unknown Core jet!"),
        }
    }

    fn encode_jet(jet: &JetNode) -> Result<u8, &'static str> {
        match jet.name {
            "adder32" => Ok(100),
            _ => Err("Unknown Core jet!"),
        }
    }

    /// Boring code that always compiles.
    fn exec_jet(jet: &JetNode) -> Result<u64, &'static str> {
        match jet.name {
            "adder32" => Ok(0), // zzz...
            _ => Err("Unknown Core jet!"),
        }
    }
}
