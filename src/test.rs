use std::collections::HashMap;

use super::*;

#[test]
fn test_dialogue_pair() {
    assert_eq!(
        NobelaParser::new(vec![]).dialogue_pair(
            ScriptParser::parse(Rule::dialogue, r#""Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Dialogue {
                character_id: None,
                speaker: None,
                text: "Hello world!".to_owned()
            },
            Stmt::EndDialogue
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![Character::new("Elira", "Elira", HashMap::new())]).dialogue_pair(
            ScriptParser::parse(Rule::dialogue, r#"Elira "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Dialogue {
                character_id: Some("Elira".to_owned()),
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            Stmt::EndDialogue
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![Character::new(
            "Elira",
            "Elira",
            HashMap::from([("Sheesh".to_owned(), "Sheesh Dragon".to_owned())])
        )])
        .dialogue_pair(
            ScriptParser::parse(Rule::dialogue, r#"Sheesh "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Dialogue {
                character_id: Some("Elira".to_owned()),
                speaker: Some("Sheesh Dragon".to_owned()),
                text: "Hello world!".to_owned()
            },
            Stmt::EndDialogue
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).dialogue_pair(
            ScriptParser::parse(Rule::dialogue, r#""Elira" "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Dialogue {
                character_id: None,
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            Stmt::EndDialogue
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).dialogue_pair(
            ScriptParser::parse(
                Rule::dialogue,
                r#""Elira" "Hello world!"
-- "First"
-- "Second""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            Stmt::Dialogue {
                character_id: None,
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            Stmt::Choice {
                text: "First".to_owned(),
                condition: None,
            },
            Stmt::EndChoice,
            Stmt::Choice {
                text: "Second".to_owned(),
                condition: None,
            },
            Stmt::EndChoice,
            Stmt::EndDialogue
        ]
    );
}

#[test]
fn test_choice_pair() {
    assert_eq!(
        NobelaParser::new(vec![]).choice_pair(
            ScriptParser::parse(Rule::choice, r#"-- "This is a choice.""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            Stmt::EndChoice
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).choice_pair(
            ScriptParser::parse(Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            Stmt::EndChoice
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).choice_pair(
            ScriptParser::parse(Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            Stmt::EndChoice
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).choice_pair(
            ScriptParser::parse(
                Rule::choice,
                r#"-- "This is a choice."
	"Nested"
	"Nested again""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            Stmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            Stmt::Dialogue {
                character_id: None,
                speaker: None,
                text: "Nested".to_owned()
            },
            Stmt::EndDialogue,
            Stmt::Dialogue {
                character_id: None,
                speaker: None,
                text: "Nested again".to_owned()
            },
            Stmt::EndDialogue,
            Stmt::EndChoice
        ]
    );
}

#[test]
fn test_if_pair() {
    assert_eq!(
        NobelaParser::new(vec![]).if_pair(
            ScriptParser::parse(Rule::if_stmt, r#"if 1 == 1:"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            Stmt::If {
                condition: "1 == 1".to_owned()
            },
            Stmt::EndIf
        ]
    );

    assert_eq!(
        NobelaParser::new(vec![]).if_pair(
            ScriptParser::parse(
                Rule::if_stmt,
                r#"if true:
	"Nested""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            Stmt::If {
                condition: "true".to_owned()
            },
            Stmt::Dialogue {
                character_id: None,
                speaker: None,
                text: "Nested".to_owned()
            },
            Stmt::EndDialogue,
            Stmt::EndIf
        ]
    );
}

#[test]
fn test_call_pair() {
    assert_eq!(
        NobelaParser::new(vec![]).call_pair(
            ScriptParser::parse(Rule::call, r#"call "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![Stmt::Call {
            jump: false,
            timeline_name: "foo".to_owned()
        }]
    );

    assert_eq!(
        NobelaParser::new(vec![]).call_pair(
            ScriptParser::parse(Rule::call, r#"jump "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![Stmt::Call {
            jump: true,
            timeline_name: "foo".to_owned()
        }]
    );
}

#[test]
fn test_parse() {
    assert_eq!(
        NobelaParser::new(vec![]).parse(r#""Hello World""#).unwrap(),
        vec![
            Stmt::Dialogue {
                character_id: None,
                speaker: None,
                text: "Hello World".to_owned()
            },
            Stmt::EndDialogue,
        ]
    );
}
