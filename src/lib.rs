extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{
    iterators::{Pair, Pairs},
    Parser, RuleType,
};

pub mod server;
#[cfg(test)]
pub mod test;

#[derive(Parser)]
#[grammar = "nobela.pest"]
pub struct NobelaParser;

#[derive(Debug, PartialEq)]
pub enum FlatStmt {
    Dialogue {
        speaker: Option<String>,
        text: String,
    },
    EndDialogue,
    Choice {
        text: String,
        condition: Option<String>,
    },
    EndChoice,
    If {
        condition: String,
    },
    EndIf,
}
#[derive(Debug)]
pub enum NestedStmt {
    Dialogue {
        speaker: Option<String>,
        text: String,
        choices: Vec<NestedStmt>,
    },
    Choice {
        text: String,
        children: Vec<NestedStmt>,
    },
}

pub fn document(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    NobelaParser::parse(Rule::document, input)
}

pub fn parse_nested(input: &str) -> Result<Vec<NestedStmt>, pest::error::Error<Rule>> {
    let pairs = document(input)?;
    let mut statements = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::dialogue {
            statements.push(nested_dialogue_pair(pair))
        }
    }

    Ok(statements)
}

pub fn parse_flat(input: &str) -> Result<Vec<FlatStmt>, pest::error::Error<Rule>> {
    let pairs = document(input)?;
    let mut statements = Vec::new();

    for pair in pairs {
        statements.append(&mut flat_events_pair(pair))
    }

    Ok(statements)
}

fn nested_dialogue_pair(pair: Pair<Rule>) -> NestedStmt {
    let mut choices = Vec::new();

    let mut speaker = None;
    let mut text = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::speaker => speaker = Some(get_string_val(inner_pair)),
            Rule::text => text = get_string_val(inner_pair),
            Rule::choice => {
                choices.push(nested_choice_pair(inner_pair));
            }
            _ => (),
        }
    }

    NestedStmt::Dialogue {
        speaker,
        text,
        choices,
    }
}

fn nested_choice_pair(pair: Pair<Rule>) -> NestedStmt {
    let mut children = Vec::new();
    let mut text = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::text => text = get_string_val(inner_pair),
            Rule::dialogue => children.push(nested_dialogue_pair(inner_pair)),

            _ => (),
        }
    }

    NestedStmt::Choice { text, children }
}

fn get_string_val<T: RuleType>(pair: Pair<T>) -> String {
    let str = pair.as_str();
    str[1..str.len() - 1].to_owned()
}

fn flat_dialogue_pair(pair: Pair<Rule>) -> Vec<FlatStmt> {
    let mut statements = Vec::new();
    let mut choices = Vec::new();

    let mut speaker = None;
    let mut text = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::speaker => speaker = Some(get_string_val(inner_pair)),
            Rule::text => text = get_string_val(inner_pair),

            Rule::choice => {
                choices.append(&mut flat_choice_pair(inner_pair));
            }
            _ => (),
        }
    }

    statements.push(FlatStmt::Dialogue { speaker, text });
    statements.append(&mut choices);
    statements.push(FlatStmt::EndDialogue);
    statements
}

fn flat_choice_pair(pair: Pair<Rule>) -> Vec<FlatStmt> {
    let mut statements = Vec::new();
    let mut children = Vec::new();
    let mut text = String::new();
    let mut condition = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::text => text = get_string_val(inner_pair),
            Rule::bool_expr => condition = Some(inner_pair.as_str().to_owned()),
            _ => children.append(&mut flat_events_pair(inner_pair)),
        }
    }

    statements.push(FlatStmt::Choice { text, condition });

    statements.append(&mut children);
    statements.push(FlatStmt::EndChoice);

    statements
}

fn flat_if_pair(pair: Pair<Rule>) -> Vec<FlatStmt> {
    let mut statements = Vec::new();
    let mut children = Vec::new();
    let mut condition = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::bool_expr => condition = inner_pair.as_str().to_owned(),
            _ => children.append(&mut flat_events_pair(inner_pair)),
        }
    }

    statements.push(FlatStmt::If { condition });

    statements.append(&mut children);
    statements.push(FlatStmt::EndIf);

    statements
}

fn flat_events_pair(pair: Pair<Rule>) -> Vec<FlatStmt> {
    let mut statements = Vec::new();
    match pair.as_rule() {
        Rule::dialogue => statements = flat_dialogue_pair(pair),
        Rule::if_stmt => statements = flat_if_pair(pair),
        _ => (),
    }

    statements
}
