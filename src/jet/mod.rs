pub mod application;
mod init;

pub use init::bitcoin;
pub use init::core;
pub use init::elements;

pub trait Application {
    /// Every jet is uniquely identified by its code _in context of its application_.
    /// Two jets of _different_ applications may share the same code.
    fn decode_jet(code: u8) -> Result<&'static JetNode, &'static str>;

    /// Encoding is the reverse of decoding, and vice versa.
    fn encode_jet(jet: &JetNode) -> Result<u8, &'static str>;

    // TODO: associated type for custom jet errors?
    /// Jets execute remote code and produce some result (here: `u64`).
    /// Without the necessary features, jet execution will result in an error.
    /// Moreover, jets may throw custom errors.
    fn exec_jet(jet: &JetNode) -> Result<u64, &'static str>;
}

pub struct JetNode {
    pub(crate) name: &'static str,
    pub(crate) cmr: [u8; 32],
    pub(crate) source_ty: &'static str,
    pub(crate) target_ty: &'static str,
}

impl JetNode {
    /// Every jet has a CMR that is _usually_ unique.
    /// Primitives commit to their application name and to their jet name in their CMR.
    /// Jets that encode sub-DAGs commit to their DAG's root CMR _(usually unique)_ and to their cost.
    pub fn cmr(&self) -> [u8; 32] {
        self.cmr
    }

    /// A jet's CMR is also its IMR.
    /// Jets may not include `witness` or `disconnect` nodes as a result.
    pub fn imr(&self) -> [u8; 32] {
        self.cmr
    }
}
