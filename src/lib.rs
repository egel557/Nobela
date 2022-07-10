extern crate pest;
#[macro_use]
extern crate pest_derive;

use character::Characters;
use pest::{
    iterators::{Pair, Pairs},
    Parser, RuleType,
};

mod character;
pub use character::Character;
pub mod server;
#[cfg(test)]
pub mod test;

#[derive(Parser)]
#[grammar = "nobela.pest"]
pub struct ScriptParser;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Dialogue {
        character_id: Option<String>,
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
    Call {
        jump: bool,
        timeline_name: String,
    },
}

pub struct NobelaParser {
    characters: Characters,
}
impl NobelaParser {
    pub fn new(characters: Characters) -> Self {
        NobelaParser { characters }
    }

    pub fn document<'a>(
        &'a self,
        input: &'a str,
    ) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
        ScriptParser::parse(Rule::document, input)
    }

    pub fn parse(&self, input: &str) -> Result<Vec<Stmt>, pest::error::Error<Rule>> {
        let pairs = self.document(input)?;
        let mut statements = Vec::new();

        for pair in pairs {
            statements.append(&mut self.events_pair(pair))
        }

        Ok(statements)
    }

    fn get_string_val<T: RuleType>(pair: Pair<T>) -> String {
        let str = pair.as_str();
        str[1..str.len() - 1].to_owned()
    }

    fn dialogue_pair(&self, pair: Pair<Rule>) -> Vec<Stmt> {
        let mut statements = Vec::new();
        let mut choices = Vec::new();
        let mut character_id = None;
        let mut speaker = None;
        let mut text = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::speaker => speaker = Some(NobelaParser::get_string_val(inner_pair)),
                Rule::text => text = NobelaParser::get_string_val(inner_pair),
                Rule::ident => {
                    let id = inner_pair.as_str();
                    let character = self.characters.iter().find(|c| *c.id() == id);
                    match character {
                        Some(character) => {
                            character_id = Some(id.to_owned());
                            speaker = Some(character.display_name().to_owned());
                        }
                        None => {
                            let character = self
                                .characters
                                .iter()
                                .find(|c| c.get_alias_name(id).is_some())
                                .unwrap_or_else(|| panic!("Character '{id}' not found."));
                            character_id = Some(character.id().to_owned());
                            speaker = Some(character.get_alias_name(id).unwrap().to_owned());
                        }
                    }
                }
                Rule::choice => {
                    choices.append(&mut self.choice_pair(inner_pair));
                }
                _ => (),
            }
        }

        statements.push(Stmt::Dialogue {
            character_id,
            speaker,
            text,
        });
        statements.append(&mut choices);
        statements.push(Stmt::EndDialogue);
        statements
    }

    fn choice_pair(&self, pair: Pair<Rule>) -> Vec<Stmt> {
        let mut statements = Vec::new();
        let mut children = Vec::new();
        let mut text = String::new();
        let mut condition = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::text => text = NobelaParser::get_string_val(inner_pair),
                Rule::bool_expr => condition = Some(inner_pair.as_str().to_owned()),
                _ => children.append(&mut self.events_pair(inner_pair)),
            }
        }

        statements.push(Stmt::Choice { text, condition });

        statements.append(&mut children);
        statements.push(Stmt::EndChoice);

        statements
    }

    fn if_pair(&self, pair: Pair<Rule>) -> Vec<Stmt> {
        let mut statements = Vec::new();
        let mut children = Vec::new();
        let mut condition = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::bool_expr => condition = inner_pair.as_str().to_owned(),
                _ => children.append(&mut self.events_pair(inner_pair)),
            }
        }

        statements.push(Stmt::If { condition });

        statements.append(&mut children);
        statements.push(Stmt::EndIf);

        statements
    }

    fn call_pair(&self, pair: Pair<Rule>) -> Vec<Stmt> {
        let mut jump = false;
        let mut timeline_name = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::jump => jump = true,
                Rule::string => timeline_name = NobelaParser::get_string_val(inner_pair),
                _ => (),
            }
        }

        vec![Stmt::Call {
            jump,
            timeline_name,
        }]
    }

    fn events_pair(&self, pair: Pair<Rule>) -> Vec<Stmt> {
        let mut statements = Vec::new();
        match pair.as_rule() {
            Rule::dialogue => statements = self.dialogue_pair(pair),
            Rule::if_stmt => statements = self.if_pair(pair),
            Rule::call => statements = self.call_pair(pair),
            _ => (),
        }

        statements
    }
}
