/* This file has been automatically generated. */

use crate::jet::application::Elements;
use crate::jet::JetNode;

#[derive(Debug)]
pub enum ElementsJetName {
    Adder,
    Version,
}

pub const ADDER32: JetNode<Elements> = JetNode {
    name: ElementsJetName::Adder,
    cmr: [0; 32],
    _source_ty: "l",
    _target_ty: "2i",
};

pub const VERSION: JetNode<Elements> = JetNode {
    name: ElementsJetName::Version,
    cmr: [2; 32],
    _source_ty: "1",
    _target_ty: "i",
};
