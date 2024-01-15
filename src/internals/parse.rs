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
                let variable_type = parse_value(variable_contents, &elsh_variables);
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

fn parse_array(lines: Pairs<'_, Rule>, elsh_vars: &Variables) -> Type {
    let mut fucking_stupid_vector: Vec<Type> = Vec::new();
    for line in lines {
        match line.as_rule() {
            Rule::array | Rule::singlequotes | Rule::doublequotes | Rule::integer | Rule::float => {
                fucking_stupid_vector.push(parse_value(line, &elsh_vars));
            }
            _ => unreachable!("Fuck you"),
        };
    }
    Type::Array(fucking_stupid_vector)
}

fn parse_string(line: Pair<'_, Rule>, elsh_vars: &Variables) -> String {
    match line.as_rule() {
        Rule::singlequotes => return String::from(line.as_str()),
        Rule::doublequotes => {
            return String::from(parse_variables_from_string(line.as_str(), &elsh_vars))
        }
        _ => unreachable!("Fuck you: {:?}", line.as_rule()),
    }
}

fn parse_variables_from_string(interpolated_string: &str, elsh_vars: &Variables) -> String {
    if interpolated_string.is_empty() {
        return "".to_string();
    }
    let mut parsed_string: Vec<char> = Vec::new();
    let mut variable_buffer: Vec<char> = Vec::new();
    let mut inside_variable = false;
    // "foo{bar}baz"
    for var in interpolated_string.chars() {
        if inside_variable == false && var != '{' {
            parsed_string.push(var);
        } else {
            if var == '{' && variable_buffer.is_empty() {
                inside_variable = true;
            } else if var != '}' {
                // Continue till we reach the ending }
                variable_buffer.push(var);
            } else {
                // We have our variable name in the buffer
                let var_name_interp = variable_buffer.iter().collect::<String>();
                let var_contents = match elsh_vars.get(var_name_interp.as_str()) {
                    Some(inner_var) => Type::String((inner_var.var_type).to_string()),
                    None => Type::String("".to_string()),
                };
                for i in var_contents.to_string().chars() {
                    parsed_string.push(i);
                }
                variable_buffer.clear();
                inside_variable = false;
            }
        }
    }
    parsed_string.into_iter().collect::<String>()
}

fn parse_value(value: Pair<'_, Rule>, elsh_vars: &Variables) -> Type {
    match value.as_rule() {
        Rule::integer => Type::Integer(value.as_str().parse().unwrap()),
        Rule::float => Type::Float(value.as_str().parse().unwrap()),
        Rule::doublequotes | Rule::singlequotes => Type::String(parse_string(value, &elsh_vars)),
        Rule::array => parse_array(value.into_inner(), &elsh_vars),
        _ => unreachable!(
            "Somewhere someone made 'Rule::assignExpr' take in non variable values. Shame on them."
        ),
    }
}
