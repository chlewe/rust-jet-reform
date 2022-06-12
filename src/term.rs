use crate::error::LibError;
use crate::jet::{Application, JetNode};

pub enum Term<Witness, App: Application> {
    Unit,
    Witness(Witness),
    Jet(&'static JetNode<App>),
}

impl<App: Application> Term<u64, App> {
    pub fn decode(code: u8) -> Result<Self, LibError> {
        match code {
            0 => Ok(Term::Unit),
            1 => Ok(Term::Witness(u64::default())),
            x if x > 100 => Ok(Term::Jet(App::decode_jet(code)?)),
            _ => Err(LibError::ParseError),
        }
    }

    pub fn encode(&self) -> u8 {
        match self {
            Term::Jet(jet) => App::encode_jet(jet),
            Term::Unit => 0,
            Term::Witness(_witness) => 1,
        }
    }

    pub fn exec(&self) -> Result<u64, App::Error> {
        match self {
            Term::Jet(jet) => App::exec_jet(jet),
            Term::Unit => Ok(0),
            Term::Witness(witness) => Ok(*witness),
        }
    }
}

pub struct Program<Witness, App: Application>(pub(crate) Vec<Term<Witness, App>>);
