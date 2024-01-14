use crate::internals::variables::VariableStatus;

use super::variables::{ElshLvl, ExportStatus, Type, Variable, Variables};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "elsh.pest"]
pub struct ELSHParser;

pub fn parse_file(path: impl Into<PathBuf> + std::convert::AsRef<std::path::Path>) {
    let unparsed_file = fs::read_to_string(&path).expect("Could not read file");

    let file = ELSHParser::parse(Rule::program, &unparsed_file)
        .expect("Failed parse")
        .next()
        .unwrap();

    let mut elsh_variables = Variables::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::assignExpr => {
                let mut inner_rules = line.into_inner();
                let variable_name = inner_rules.next().unwrap().as_str();
                let variable_contents = inner_rules.next().unwrap();
                let variable_type = parse_value(variable_contents);
                elsh_variables.set(
                    &variable_name,
                    Variable {
                        var_type: variable_type,
                        var_status: VariableStatus { readonly: false },
                        var_export_status: ExportStatus::Normal,
                        var_lvl: ElshLvl(1),
                    },
                );
            }
            Rule::eoi => println!("{}", "Finished parsing"),
            Rule::newline | Rule::ident | Rule::string => continue,
            _ => unreachable!("{:?}", line.as_rule()),
        }
    }

    dbg!("{:?}", elsh_variables);
}

fn parse_array(lines: Pairs<'_, Rule>) -> Type {
    let mut fucking_stupid_vector: Vec<Type> = Vec::new();
    for line in lines {
        match line.as_rule() {
            Rule::array | Rule::string | Rule::integer | Rule::float => {
                fucking_stupid_vector.push(parse_value(line));
            }
            _ => unreachable!("Fuck you"),
        };
    }
    Type::Array(fucking_stupid_vector)
}

fn parse_value(value: Pair<'_, Rule>) -> Type {
    match value.as_rule() {
        Rule::integer => Type::Integer(value.as_str().parse().unwrap()),
        Rule::float => Type::Float(value.as_str().parse().unwrap()),
        Rule::string => Type::String(value.as_str().to_string()),
        Rule::array => parse_array(value.into_inner()),
        _ => unreachable!(
            "Somewhere someone made 'Rule::assignExpr' take in non variable values. Shame on them."
        ),
    }
}
