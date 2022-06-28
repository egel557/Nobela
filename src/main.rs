use std::io;

use nobela_parser2::{parse_flat, parse_nested, FlatStmt, NestedStmt};

fn main() {
    let events = parse_nested(
        r#"
"Elira" "Short hair or long hair?"
-- "Short!"
	"You picked short!"
	-- "Hell yeah!"
		"Nice choice!"
-- "Long!"
	"You picked long!"
"Another one"
	"#,
    )
    .unwrap_or_else(|e| panic!("{}", e));

    println!("{:#?}", events);

    execute_nested(&events);
}

fn execute_choice(choice: &NestedStmt) {
    if let NestedStmt::Choice { children, .. } = choice {
        for event in children {
            match event {
                NestedStmt::Dialogue { .. } => execute_dialogue(event),
                NestedStmt::Choice { .. } => (),
            }
        }
    }
}

fn execute_dialogue(dialogue: &NestedStmt) {
    if let NestedStmt::Dialogue {
        choices,
        speaker,
        text,
    } = dialogue
    {
        let mut choice_texts = Vec::new();

        for choice in choices {
            if let NestedStmt::Choice { text, .. } = choice {
                choice_texts.push(text);
            }
        }

        println!("{speaker}: {text} {:?}", &choice_texts);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if choice_texts.len() > 1 {
            execute_choice(&choices[0])
        }
    }
}
fn execute_nested(events: &Vec<NestedStmt>) {
    for event in events {
        match event {
            NestedStmt::Dialogue { .. } => execute_dialogue(event),
            NestedStmt::Choice { .. } => (),
        }
    }
}

fn execute_flat(events: &Vec<FlatStmt>) {
    let mut index = 0;

    while index < events.len() {
        match &events[index] {
            FlatStmt::Dialogue { speaker, text } => {
                let mut next_index = index + 1;
                let mut choice_texts = Vec::new();
                let mut choice_indexes = Vec::new();
                let mut nested_count = 0;

                loop {
                    // println!("{nested_count}");
                    match &events[next_index] {
                        FlatStmt::EndDialogue => {
                            if nested_count > 0 {
                                nested_count -= 1
                            } else {
                                break;
                            }
                        }
                        FlatStmt::Dialogue { .. } => nested_count += 1,
                        FlatStmt::Choice { text } => {
                            if nested_count > 0 {
                                nested_count += 1
                            } else {
                                choice_texts.push(text);
                                choice_indexes.push(next_index.to_owned())
                            }
                        }
                        FlatStmt::EndChoice => {
                            if nested_count > 0 {
                                nested_count -= 1
                            } else {
                            }
                        } // _ => ()
                    }
                    next_index += 1;
                }

                println!("{speaker}: {text} {:?}", &choice_texts);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                index = if !choice_indexes.is_empty() {
                    choice_indexes[0]
                } else {
                    index + 1
                }; // Change depending on user choice
            }
            FlatStmt::Choice { .. } => {
                index += 1;
            }
            FlatStmt::EndDialogue => {
                index += 1;
            }
            FlatStmt::EndChoice => {
                let mut next_index = index + 1;
                let mut nested_count = 0;

                if matches!(&events[next_index], FlatStmt::Choice { .. }) {
                    loop {
                        match &events[next_index] {
                            FlatStmt::Dialogue { .. } => nested_count += 1,
                            FlatStmt::EndDialogue => {
                                if nested_count > 0 {
                                    nested_count -= 1
                                } else {
                                    break;
                                }
                            }
                            FlatStmt::Choice { .. } => nested_count += 1,
                            FlatStmt::EndChoice => nested_count -= 1,
                        }

                        next_index += 1;
                    }
                }
                index = next_index;
            }
        }
    }
}
