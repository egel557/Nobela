use super::*;

#[test]
fn test_flat_dialogue_pair() {
    assert_eq!(
        flat_dialogue_pair(
            NobelaParser::parse(Rule::dialogue, r#""Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::Dialogue {
                speaker: None,
                text: "Hello world!".to_owned()
            },
            FlatStmt::EndDialogue
        ]
    );

    assert_eq!(
        flat_dialogue_pair(
            NobelaParser::parse(Rule::dialogue, r#""Elira" "Hello world!""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::Dialogue {
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            FlatStmt::EndDialogue
        ]
    );

    assert_eq!(
        flat_dialogue_pair(
            NobelaParser::parse(
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
            FlatStmt::Dialogue {
                speaker: Some("Elira".to_owned()),
                text: "Hello world!".to_owned()
            },
            FlatStmt::Choice {
                text: "First".to_owned(),
                condition: None,
            },
            FlatStmt::EndChoice,
            FlatStmt::Choice {
                text: "Second".to_owned(),
                condition: None,
            },
            FlatStmt::EndChoice,
            FlatStmt::EndDialogue
        ]
    );
}

#[test]
fn test_flat_choice_pair() {
    assert_eq!(
        flat_choice_pair(
            NobelaParser::parse(Rule::choice, r#"-- "This is a choice.""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            FlatStmt::EndChoice
        ]
    );

    assert_eq!(
        flat_choice_pair(
            NobelaParser::parse(Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            FlatStmt::EndChoice
        ]
    );

    assert_eq!(
        flat_choice_pair(
            NobelaParser::parse(Rule::choice, r#"-- "This is a choice." if true"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: Some("true".to_owned()),
            },
            FlatStmt::EndChoice
        ]
    );

    assert_eq!(
        flat_choice_pair(
            NobelaParser::parse(
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
            FlatStmt::Choice {
                text: "This is a choice.".to_owned(),
                condition: None,
            },
            FlatStmt::Dialogue {
                speaker: None,
                text: "Nested".to_owned()
            },
            FlatStmt::EndDialogue,
            FlatStmt::Dialogue {
                speaker: None,
                text: "Nested again".to_owned()
            },
            FlatStmt::EndDialogue,
            FlatStmt::EndChoice
        ]
    );
}

#[test]
fn test_flat_if_pair() {
    assert_eq!(
        flat_if_pair(
            NobelaParser::parse(Rule::if_stmt, r#"if 1 == 1:"#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![
            FlatStmt::If {
                condition: "1 == 1".to_owned()
            },
            FlatStmt::EndIf
        ]
    );

    assert_eq!(
        flat_if_pair(
            NobelaParser::parse(
                Rule::if_stmt,
                r#"if true:
	"Nested""#
            )
            .unwrap()
            .next()
            .unwrap()
        ),
        vec![
            FlatStmt::If {
                condition: "true".to_owned()
            },
            FlatStmt::Dialogue {
                speaker: None,
                text: "Nested".to_owned()
            },
            FlatStmt::EndDialogue,
            FlatStmt::EndIf
        ]
    );
}

#[test]
fn test_flat_call_pair() {
    assert_eq!(
        flat_call_pair(
            NobelaParser::parse(Rule::call, r#"call "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![FlatStmt::Call {
            jump: false,
            timeline_name: "foo".to_owned()
        }]
    );

    assert_eq!(
        flat_call_pair(
            NobelaParser::parse(Rule::call, r#"jump "foo""#)
                .unwrap()
                .next()
                .unwrap()
        ),
        vec![FlatStmt::Call {
            jump: true,
            timeline_name: "foo".to_owned()
        }]
    );
}

#[test]
fn test_parse_flat() {
    assert_eq!(
        parse_flat(r#""Hello World""#).unwrap(),
        vec![
            FlatStmt::Dialogue {
                speaker: None,
                text: "Hello World".to_owned()
            },
            FlatStmt::EndDialogue,
        ]
    );
}
