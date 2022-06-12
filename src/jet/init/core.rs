/* This file has been automatically generated. */

use crate::jet::application::Core;
use crate::jet::JetNode;

#[derive(Debug)]
pub enum CoreJetName {
    Adder,
}

pub const ADDER32: JetNode<Core> = JetNode {
    name: CoreJetName::Adder,
    cmr: [0; 32],
    _source_ty: "l",
    _target_ty: "2i",
};
