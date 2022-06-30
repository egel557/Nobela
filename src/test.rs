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
                text: "First".to_owned()
            },
            FlatStmt::EndChoice,
            FlatStmt::Choice {
                text: "Second".to_owned()
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
                text: "This is a choice.".to_owned()
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
                text: "This is a choice.".to_owned()
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
