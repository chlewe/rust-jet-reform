use crate::jet::{Application, JetNode};
use std::marker::PhantomData;

pub enum Term<Witness, App: Application> {
    Unit,
    Witness(Witness),
    Jet(&'static JetNode),
    _Marker(PhantomData<App>),
}

impl<App: Application> Term<u64, App> {
    pub fn decode(code: u8) -> Result<Self, &'static str> {
        match code {
            0 => Ok(Term::Unit),
            1 => Ok(Term::Witness(u64::default())),
            x if x > 100 => Ok(Term::Jet(App::decode_jet(code)?)),
            _ => Err("Unknown term!"),
        }
    }

    pub fn encode(&self) -> Result<u8, &'static str> {
        match self {
            Term::Jet(jet) => App::encode_jet(jet),
            Term::Unit => Ok(0),
            Term::Witness(_witness) => Ok(1),
            _ => Err("Illegal term!"),
        }
    }

    pub fn exec(&self) -> Result<u64, &'static str> {
        match self {
            Term::Jet(jet) => App::exec_jet(jet),
            Term::Unit => Ok(0),
            Term::Witness(witness) => Ok(*witness),
            _ => Err("Illegal term!"),
        }
    }
}

pub struct Program<Witness, App: Application>(pub(crate) Vec<Term<Witness, App>>);
