use elsh::internals::variables::{Type, Variables};
use elsh::internals::parse;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "elsh.pest"]
pub struct ELSHParser;

fn main() {
    let unparsed_file = fs::read_to_string("test.els").expect("Could not read file");

    let file = ELSHParser::parse(Rule::program, &unparsed_file)
        .expect("Failed parse")
        .next()
        .unwrap();

    let mut elsh_variables = Variables::new();

    dbg!("{}", &file);

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::assignExpr => {
                let mut inner_rules = line.into_inner();
                let variable_name = inner_rules.next().unwrap().as_str();
                let variable_contents = inner_rules.peek().unwrap().as_str();
                let variable_type = match inner_rules.next().unwrap().as_rule() {
                    Rule::integer => Type::Integer(variable_contents.parse().unwrap()),
                    Rule::float => Type::Float(variable_contents.parse().unwrap()),
                    Rule::string => Type::String(variable_contents.parse().unwrap()),
                    Rule::array => ,
                    _ => unreachable!("Somewhere someone made 'Rule::assignExpr' take in non variable values. Shame on them."),
                };
                elsh_variables.set(&variable_name, variable_type);
            }
            Rule::eoi => println!("{}", "Finished parsing"),
            Rule::newline => continue,
            Rule::ident => continue,
            Rule::string => continue,
            _ => unreachable!("{:?}", line.as_rule()),
        }
    }
}
