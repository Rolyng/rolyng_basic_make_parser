use std::fmt::Display;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./basic_make.pest"]
pub struct MakeParser;

#[derive(Debug)]
pub struct MakeRule {
    name: String,
    dependencies: Vec<String>,
    commands: Vec<String>,
}

impl Display for MakeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rule \n\tname: {}\n\tdependencies: ", self.name)?;
        for d in self.dependencies.iter() {
            write!(f, "{},", d)?;
        }
        write!(f, "\n\tcommands:\n\t\t")?;
        for c in self.commands.iter() {
            write!(f, "{}\n\t\t", c)?;
        }
        Ok(())
    }
}

pub fn parse_make(input: &str) -> Result<Vec<MakeRule>, pest::error::Error<Rule>> {
    let mut ret = vec![];
    let parsed_data = MakeParser::parse(Rule::file, input)?;

    for pair in parsed_data
        .into_iter()
        .flatten()
        .filter(|a| a.as_rule() == Rule::rule)
    {
        let rule: MakeRule = MakeRule {
            name: pair
                .clone()
                .into_inner()
                .flatten()
                .find(|a| a.as_rule() == Rule::rule_name)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "Bad stuff".to_string(),
                        },
                        pair.as_span(),
                    )
                })?
                .as_str()
                .to_string(),
            dependencies: {
                let dep = pair
                    .clone()
                    .into_inner()
                    .flatten()
                    .find(|a| a.as_rule() == Rule::dependencies);
                match dep {
                    Some(d) => d
                        .into_inner()
                        .filter(|a| a.as_rule() == Rule::identifier)
                        .map(|a| a.as_str().to_string())
                        .collect(),
                    None => vec![],
                }
            },
            commands: {
                pair.into_inner()
                    .flatten()
                    .filter(|a| a.as_rule() == Rule::shell_command)
                    .map(|a| a.as_str().to_string())
                    .collect()
            },
        };
        ret.push(rule);
    }

    Ok(ret)
}
