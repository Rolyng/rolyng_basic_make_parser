#[cfg(test)]
mod tests {
    use pest::Parser;
    use rolyng_basic_make_parser::{MakeParser, Rule};

    const RULE_PREFIX: &str = "\t";

    #[test]
    fn identifier_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::identifier, "a232_1as");
        assert_eq!(pair?.as_str(), "a232_1as");

        let pair = MakeParser::parse(Rule::identifier, "2iden");
        assert!(pair.is_err());

        let pair = MakeParser::parse(Rule::identifier, "Я_НЕ_АСКІ");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn shell_command_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::shell_command, "anything");
        assert_eq!(pair?.as_str(), "anything");

        let pair = MakeParser::parse(Rule::shell_command, "\n");
        assert!(pair.is_err());

        let pair = MakeParser::parse(Rule::shell_command, "abc\n");
        assert!(pair?.as_str().len() == "abc".len());

        Ok(())
    }

    #[test]
    fn recipe_command_test() -> anyhow::Result<()> {
        let test_str = format!("{}anything\n", RULE_PREFIX);
        let pair = MakeParser::parse(Rule::recipe_command, &test_str);
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::recipe_command, "no prefix\n");
        assert!(pair.is_err());

        let pair = MakeParser::parse(Rule::recipe_command, &test_str[0..test_str.len()-1]);
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn recipe_test() -> anyhow::Result<()> {
        let test_str = format!("{}anything\n", RULE_PREFIX);
        let pair = MakeParser::parse(Rule::recipe, &test_str);
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::recipe, "");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn rule_name_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::rule_name, "a232_1as");
        assert_eq!(pair?.as_str(), "a232_1as");

        let pair = MakeParser::parse(Rule::rule_name, "2iden");
        assert!(pair.is_err());

        let pair = MakeParser::parse(Rule::rule_name, "Я_НЕ_АСКІ");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn dependencies_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::dependencies, "a232_1as");
        assert_eq!(pair?.as_str(), "a232_1as");

        let pair = MakeParser::parse(Rule::dependencies, "a232_1as, ab");
        assert_eq!(pair?.as_str(), "a232_1as, ab");

        let pair = MakeParser::parse(Rule::dependencies, "");
        assert!(pair.is_err());

        Ok(())
    
    }

    #[test]
    fn rule_header_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::rule_header, "aa: bb\n");
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::rule_header, "aa:\n");
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::rule_header, "aa  bb\n");
        assert!(pair.is_err());

        let pair = MakeParser::parse(Rule::rule_header, "aa\n");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn rule_test() -> anyhow::Result<()> {
        let test_str = format!("aa:bb\n{}anything\n", RULE_PREFIX);
        let pair = MakeParser::parse(Rule::rule, &test_str);
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::rule, &test_str[0..test_str.len()-1]);
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn line_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::line, "\n");
        assert!(pair.is_ok());

        let test_str = format!("aa:bb\n{}anything\n", RULE_PREFIX);
        let pair = MakeParser::parse(Rule::line, &test_str);
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::line, "aboba");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn file_test() -> anyhow::Result<()> {
        let pair = MakeParser::parse(Rule::file, "\n");
        assert!(pair.is_ok());

        let test_str = format!("aa:bb\n{}anything\n\n\n", RULE_PREFIX);
        let pair = MakeParser::parse(Rule::file, &test_str);
        assert!(pair.is_ok());

        let pair = MakeParser::parse(Rule::file, "aboba\naa");
        assert!(pair.is_err());

        Ok(())
    }
}
