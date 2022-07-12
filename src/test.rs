use crate::server::{Timeline, Timelines};

use super::*;
use pest::Parser;
use std::collections::HashMap;

#[test]
fn test_dialogue_pair() {
    assert_eq!(
        parser::Parser::new(vec![]).dialogue_pair(
            parser::ScriptParser::parse(parser::Rule::dialogue, r#""Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![Character::new(
            "Elira",
            "Elira",
            HashMap::new(),
            HashMap::from([("default".to_owned(), "test_images/elira1.png".to_owned())])
        )])
        .dialogue_pair(
            parser::ScriptParser::parse(parser::Rule::dialogue, r#"Elira "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::from([(
                    "default".to_owned(),
                    "test_images/elira1.png".to_owned()
                )]),
                character_id: Some("Elira".to_owned()),
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![Character::new(
            "Elira",
            "Elira",
            HashMap::new(),
            HashMap::new()
        )])
        .dialogue_pair(
            parser::ScriptParser::parse(parser::Rule::dialogue, r#"Elira "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: Some("Elira".to_owned()),
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![Character::new(
            "Elira",
            "Elira",
            HashMap::new(),
            HashMap::new()
        )])
        .dialogue_pair(
            parser::ScriptParser::parse(
                parser::Rule::dialogue,
                r#"Elira as "Sheesh Dragon" "Hello world!""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: Some("Elira".to_owned()),
                speaker: Some("Sheesh Dragon".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![Character::new(
            "Elira",
            "Elira",
            HashMap::from([("Sheesh".to_owned(), "Sheesh Dragon".to_owned())]),
            HashMap::new()
        )])
        .dialogue_pair(
            parser::ScriptParser::parse(parser::Rule::dialogue, r#"Sheesh "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: Some("Elira".to_owned()),
                speaker: Some("Sheesh Dragon".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).dialogue_pair(
            parser::ScriptParser::parse(parser::Rule::dialogue, r#""Elira" "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::EndDialogue
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).dialogue_pair(
            parser::ScriptParser::parse(
                parser::Rule::dialogue,
                r#""Elira" "Hello world!"
-- "First"
-- "Second""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            parser::Stmt::Choice {
                text: "First".to_owned(),
                condition: None,
            },
            parser::Stmt::EndChoice,
            parser::Stmt::Choice {
                text: "Second".to_owned(),
                condition: None,
            },
            parser::Stmt::EndChoice,
            parser::Stmt::EndDialogue
        ]
    );
}

#[test]
fn test_choice_pair() {
    assert_eq!(
        parser::Parser::new(vec![]).choice_pair(
            parser::ScriptParser::parse(parser::Rule::choice, r#"-- "This is a choice.""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            parser::Stmt::EndChoice
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).choice_pair(
            parser::ScriptParser::parse(parser::Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            parser::Stmt::EndChoice
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).choice_pair(
            parser::ScriptParser::parse(parser::Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            parser::Stmt::EndChoice
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).choice_pair(
            parser::ScriptParser::parse(
                parser::Rule::choice,
                r#"-- "This is a choice."
	"Nested"
	"Nested again""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            parser::Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Nested".to_owned()
            },
            parser::Stmt::EndDialogue,
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Nested again".to_owned()
            },
            parser::Stmt::EndDialogue,
            parser::Stmt::EndChoice
        ]
    );
}

#[test]
fn test_if_pair() {
    assert_eq!(
        parser::Parser::new(vec![]).if_pair(
            parser::ScriptParser::parse(parser::Rule::if_stmt, r#"if 1 == 1:"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            parser::Stmt::If {
                condition: "1 == 1".to_owned()
            },
            parser::Stmt::EndIf
        ]
    );

    assert_eq!(
        parser::Parser::new(vec![]).if_pair(
            parser::ScriptParser::parse(
                parser::Rule::if_stmt,
                r#"if true:
	"Nested""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            parser::Stmt::If {
                condition: "true".to_owned()
            },
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Nested".to_owned()
            },
            parser::Stmt::EndDialogue,
            parser::Stmt::EndIf
        ]
    );
}

#[test]
fn test_call_pair() {
    assert_eq!(
        parser::Parser::new(vec![]).call_pair(
            parser::ScriptParser::parse(parser::Rule::call, r#"call "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![parser::Stmt::Call {
            jump: false,
            timeline_name: "foo".to_owned()
        }]
    );

    assert_eq!(
        parser::Parser::new(vec![]).call_pair(
            parser::ScriptParser::parse(parser::Rule::call, r#"jump "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![parser::Stmt::Call {
            jump: true,
            timeline_name: "foo".to_owned()
        }]
    );
}

#[test]
fn test_set_pair() {
    assert_eq!(
        parser::Parser::new(vec![]).set_pair(
            parser::ScriptParser::parse(parser::Rule::set, r#"foo = "bar""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![parser::Stmt::Set {
            variable_name: "foo".to_owned(),
            expression: r#""bar""#.to_owned()
        }]
    );
}

#[test]
fn test_parse() {
    assert_eq!(
        parser::Parser::new(vec![])
            .parse(r#""Hello World""#)
            .unwrap(),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Hello World".to_owned()
            },
            parser::Stmt::EndDialogue,
        ]
    );
}

#[test]
fn test_parse_file() {
    assert_eq!(
        parser::Parser::new(vec![])
            .parse_file("test_files/start.nobela")
            .unwrap(),
        vec![
            parser::Stmt::Dialogue {
                expression: None,
                portraits: HashMap::new(),
                character_id: None,
                speaker: None,
                text: "Hello World!".to_owned()
            },
            parser::Stmt::EndDialogue,
        ]
    );
}

#[test]
fn test_parse_dir() {
    assert_eq!(
        parser::Parser::new(vec![]).parse_dir("test_files").unwrap(),
        Timelines::from([
            (
                "start".to_owned(),
                Timeline::from([
                    parser::Stmt::Dialogue {
                        expression: None,
                        portraits: HashMap::new(),
                        character_id: None,
                        speaker: None,
                        text: "Hello World!".to_owned()
                    },
                    parser::Stmt::EndDialogue
                ])
            ),
            (
                "group.nested".to_owned(),
                Timeline::from([
                    parser::Stmt::Dialogue {
                        expression: None,
                        portraits: HashMap::new(),
                        character_id: None,
                        speaker: None,
                        text: "Nested Timeline".to_owned()
                    },
                    parser::Stmt::EndDialogue
                ])
            )
        ])
    )
}

#[test]
fn test_characters_from_json() {
    assert_eq!(
        parser::characters_from_json("characters.json").unwrap(),
        vec![Character::new(
            "Elira",
            "Ewiwa",
            HashMap::new(),
            HashMap::new()
        )]
    )
}


//TODO Create tests for server.