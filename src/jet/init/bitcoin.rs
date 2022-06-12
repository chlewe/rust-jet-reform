/* This file has been automatically generated. */

use crate::jet::application::Bitcoin;
use crate::jet::JetNode;

#[derive(Debug)]
pub enum BitcoinJetName {
    Adder,
    Version,
}

pub const ADDER32: JetNode<Bitcoin> = JetNode {
    name: BitcoinJetName::Adder,
    cmr: [0; 32],
    _source_ty: "l",
    _target_ty: "2i",
};

pub const VERSION: JetNode<Bitcoin> = JetNode {
    name: BitcoinJetName::Version,
    cmr: [1; 32],
    _source_ty: "1",
    _target_ty: "i",
};
