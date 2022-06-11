pub mod error;
pub mod jet;
pub mod term;

#[cfg(test)]
mod test {
    use crate::jet;
    use crate::jet::application::{Bitcoin, Core, Elements};
    use crate::jet::Application;

    #[test]
    fn core_jet() {
        assert_eq!(Ok(0), Core::exec_jet(&jet::core::ADDER32));
    }

    #[cfg(feature = "bitcoin")]
    #[test]
    fn bitcoin_jet() {
        assert_eq!(Ok(1337), Bitcoin::exec_jet(&jet::bitcoin::VERSION));
    }

    #[cfg(not(feature = "bitcoin"))]
    #[test]
    fn bitcoin_jet() {
        assert!(Bitcoin::exec_jet(&jet::bitcoin::VERSION).is_err());
    }

    #[cfg(feature = "elements")]
    #[test]
    fn elements_jet() {
        assert_eq!(Ok(31337), Elements::exec_jet(&jet::elements::VERSION));
    }

    #[cfg(not(feature = "elements"))]
    #[test]
    fn elements_jet() {
        assert!(Elements::exec_jet(&jet::elements::VERSION).is_err());
    }

    #[cfg(feature = "bitcoin")]
    #[test]
    fn bitcoin_term() {
        let term = Term::<u64, Bitcoin>::Jet(&jet::bitcoin::VERSION);
        assert_eq!(Ok(1337), term.exec());

        let term = Term::<u64, Core>::Jet(&jet::bitcoin::VERSION);
        assert!(term.exec().is_err());
    }

    #[cfg(feature = "elements")]
    #[test]
    fn elements_term() {
        let term = Term::<u64, Elements>::Jet(&jet::elements::VERSION);
        assert_eq!(Ok(31337), term.exec());

        let term = Term::<u64, Core>::Jet(&jet::elements::VERSION);
        assert!(term.exec().is_err());
    }
}
