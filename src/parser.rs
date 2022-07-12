use serde_json::Result as SerdeResult;
use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

use pest::{
    iterators::{Pair, Pairs},
    Parser as PestParser, RuleType,
};
use walkdir::WalkDir;

use crate::{
    server::{Timeline, Timelines},
    Character, FILE_EXTENSION,
};

use super::character::Characters;

#[derive(Parser)]
#[grammar = "nobela.pest"]
pub struct ScriptParser;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Dialogue {
        character_id: Option<String>,
        speaker: Option<String>,
        text: String,
        expression: Option<String>,
        portraits: HashMap<String, String>,
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
    Set {
        variable_name: String,
        expression: String,
    },
}

pub fn characters_from_json(path: &str) -> SerdeResult<Vec<Character>> {
    let path = Path::new(path);
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents)
}

pub struct Parser {
    characters: Characters,
}
impl Parser {
    pub fn new(characters: Characters) -> Self {
        Parser { characters }
    }

    pub fn document<'a>(
        &'a self,
        input: &'a str,
    ) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
        ScriptParser::parse(Rule::document, input)
    }

    pub fn parse_file(&self, filename: &str) -> Result<Timeline, pest::error::Error<Rule>> {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading '{filename}'."));
        self.parse(&contents)
    }

    pub fn parse_dir(&self, dir_name: &str) -> Result<Timelines, pest::error::Error<Rule>> {
        let mut timelines = Timelines::new();
        let entries = WalkDir::new(dir_name)
            .into_iter()
            .filter_map(|v| v.ok())
            .filter(|x| x.path().extension().unwrap_or_else(|| OsStr::new("")) == FILE_EXTENSION);

        for entry in entries {
            let timeline = self.parse_file(entry.path().to_str().unwrap())?;
            let name = entry
                .path()
                .to_str()
                .unwrap()
                .to_owned()
                .strip_suffix(&format!(".{FILE_EXTENSION}"))
                .unwrap()
                .strip_prefix(&dir_name.to_string())
                .unwrap()
                .replace('\\', ".")
                .replace('/', ".")
                .strip_prefix('.')
                .unwrap()
                .to_owned();
            timelines.insert(name, timeline);
        }

        Ok(timelines)
    }

    pub fn parse(&self, input: &str) -> Result<Timeline, pest::error::Error<Rule>> {
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

    pub fn dialogue_pair(&self, pair: Pair<Rule>) -> Timeline {
        let mut statements = Vec::new();
        let mut choices = Vec::new();
        let mut character_id = None;
        let mut speaker = None;
        let mut text = String::new();
        let mut character: Option<&Character> = None;
        let mut expression: Option<String> = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::speaker => speaker = Some(Parser::get_string_val(inner_pair)),
                Rule::text => text = Parser::get_string_val(inner_pair),
                Rule::alias => speaker = Some(Parser::get_string_val(inner_pair)),
                Rule::expression => expression = Some(inner_pair.as_str().to_owned()),
                // Rule::portrait => portrait_path = Some(character.as_ref().unwrap().get_portrait_path(inner_pair.as_str()).unwrap().to_owned()),
                Rule::ident => {
                    let id = inner_pair.as_str();
                    let c = self.characters.iter().find(|c| *c.id() == id);
                    match c {
                        Some(c) => {
                            character = Some(c);
                            character_id = Some(id.to_owned());
                            speaker = Some(c.display_name().to_owned());
                        }
                        None => {
                            let c = self
                                .characters
                                .iter()
                                .find(|c| c.get_alias_name(id).is_some())
                                .unwrap_or_else(|| panic!("Character '{id}' not found."));
                            character = Some(c);
                            character_id = Some(c.id().to_owned());
                            speaker = Some(c.get_alias_name(id).unwrap().to_owned());
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
            expression,
            portraits: if let Some(character) = character {
                character.portraits().to_owned()
            } else {
                HashMap::new()
            },
        });
        statements.append(&mut choices);
        statements.push(Stmt::EndDialogue);
        statements
    }

    pub fn choice_pair(&self, pair: Pair<Rule>) -> Timeline {
        let mut statements = Vec::new();
        let mut children = Vec::new();
        let mut text = String::new();
        let mut condition = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::text => text = Parser::get_string_val(inner_pair),
                Rule::bool_expr => condition = Some(inner_pair.as_str().to_owned()),
                _ => children.append(&mut self.events_pair(inner_pair)),
            }
        }

        statements.push(Stmt::Choice { text, condition });

        statements.append(&mut children);
        statements.push(Stmt::EndChoice);

        statements
    }

    pub fn if_pair(&self, pair: Pair<Rule>) -> Timeline {
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

    pub fn call_pair(&self, pair: Pair<Rule>) -> Timeline {
        let mut jump = false;
        let mut timeline_name = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::jump => jump = true,
                Rule::string => timeline_name = Parser::get_string_val(inner_pair),
                _ => (),
            }
        }

        vec![Stmt::Call {
            jump,
            timeline_name,
        }]
    }

    pub fn set_pair(&self, pair: Pair<Rule>) -> Timeline {
        let mut variable_name = String::new();
        let mut expression = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ident => variable_name = inner_pair.as_str().to_owned(),
                Rule::expr => expression = inner_pair.as_str().to_owned(),
                _ => (),
            }
        }

        vec![Stmt::Set {
            variable_name,
            expression,
        }]
    }

    pub fn events_pair(&self, pair: Pair<Rule>) -> Timeline {
        let mut statements = Vec::new();
        match pair.as_rule() {
            Rule::dialogue => statements = self.dialogue_pair(pair),
            Rule::if_stmt => statements = self.if_pair(pair),
            Rule::call => statements = self.call_pair(pair),
            Rule::set => statements = self.set_pair(pair),
            _ => (),
        }

        statements
    }
}
