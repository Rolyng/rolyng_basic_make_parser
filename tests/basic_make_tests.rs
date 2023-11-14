#[cfg(test)]
mod tests{
use rolyng_parser_c::{Grammar, Rule};
use pest::Parser;
    #[test]
    fn identifier_test() -> anyhow::Result< () > {
        let pair = Grammar::parse(Rule::identifier, "a232_1as");
        assert_eq!(pair?.as_str(), "a232_1as");

        let pair = Grammar::parse(Rule::identifier, "2iden");
        assert!(pair.is_err());

        let pair = Grammar::parse(Rule::identifier, "Я_НЕ_АСКІ");
        assert!(pair.is_err());

        Ok(())
    }
}
