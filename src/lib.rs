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
    },
    EndChoice,
    // If {
    // 	condition: String,
    // },
    // EndIf
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
        if pair.as_rule() == Rule::dialogue {
            statements.append(&mut flat_dialogue_pair(pair))
        }
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

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::text => text = get_string_val(inner_pair),
            Rule::dialogue => children.append(&mut flat_dialogue_pair(inner_pair)),
            _ => (),
        }
    }

    statements.push(FlatStmt::Choice { text });

    statements.append(&mut children);
    statements.push(FlatStmt::EndChoice);

    statements
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_document() {
//         let output = document(
//             r#"
// "Elira" "Hello World"
// -- "First"
// 	"In first"
// 	-- "Nested choice"
// 	"In first again"
// -- "Second"
// -- "Third"
// "Another one"
// "#,
//         )
//         .unwrap();
//         println!("{:#?}", output);
//     }

//     #[test]
//     fn parse_into_statements() {
//         let output = parse_flat(
//             r#"
// "Elira" "Hello World"
// -- "First"
// 	"In first"
// 	-- "Nested choice"
// 	"In first again"
// -- "Second"
// -- "Third"
// "Another one"
// "#,
//         )
//         .unwrap();
//         println!("{:#?}", output);
//     }
// }
