use evalexpr::{eval_boolean_with_context, HashMapContext};

use crate::FlatStmt;

pub enum Event {
    Dialogue {
        speaker: Option<String>,
        text: String,
        choices: Vec<(String, bool)>,
    },
    Ignore,
}

pub struct Server<'a> {
    stmts: &'a Vec<FlatStmt>,
    index: usize,
    choice_indexes: Option<Vec<usize>>,
    context: &'a HashMapContext,
}

impl<'a> Server<'a> {
    pub fn new(stmts: &'a Vec<FlatStmt>, index: usize, context: &'a HashMapContext) -> Self {
        Server {
            stmts,
            index,
            choice_indexes: None,
            context,
        }
    }

    pub fn choose(&mut self, choice: usize) {
        self.index = *self
            .choice_indexes
            .as_ref()
            .expect("No choices.")
            .get(choice)
            .expect("Invalid choice index");
    }
}

impl Iterator for Server<'_> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.stmts.get(self.index);

        match curr {
            Some(curr) => match curr {
                FlatStmt::Dialogue { speaker, text } => {
                    let mut next_index = self.index + 1;
                    let mut choices = Vec::new();
                    let mut choice_indexes = Vec::new();
                    let mut nested_count = 0;

                    loop {
                        let next_event = &self.stmts[next_index];
                        match next_event {
                            FlatStmt::EndDialogue => {
                                if nested_count > 0 {
                                    nested_count -= 1
                                } else {
                                    break;
                                }
                            }
                            FlatStmt::Dialogue { .. } => nested_count += 1,
                            FlatStmt::Choice { .. } => {
                                if nested_count > 0 {
                                    nested_count += 1
                                } else {
                                    choices.push(next_event);
                                    choice_indexes.push(next_index.to_owned())
                                }
                            }
                            FlatStmt::EndChoice => {
                                if nested_count > 0 {
                                    nested_count -= 1
                                }
                            }
                            FlatStmt::If { .. } => nested_count += 1,
                            FlatStmt::EndIf => nested_count -= 1,
                        }
                        next_index += 1;
                    }

                    self.choice_indexes = Some(choice_indexes);
                    self.index += 1;
                    Some(Event::Dialogue {
                        speaker: speaker.to_owned(),
                        text: text.to_owned(),
                        choices: choices
                            .into_iter()
                            .map(|c| {
                                if let FlatStmt::Choice { text, condition } = c {
                                    let hide = match condition {
                                        Some(condition) => {
                                            !eval_boolean_with_context(condition, self.context)
                                                .unwrap_or_else(|_| {
                                                    panic!("Error evaluating '{condition}'")
                                                })
                                        }
                                        None => false,
                                    };
                                    (text.to_owned(), hide)
                                } else {
                                    unreachable!()
                                }
                            })
                            .collect(),
                    })
                }
                FlatStmt::Choice { .. } | FlatStmt::EndDialogue | FlatStmt::EndIf => {
                    self.index += 1;
                    Some(Event::Ignore)
                }
                FlatStmt::EndChoice => {
                    let mut next_index = self.index + 1;
                    let mut nested_count = 0;
                    let mut next_event = &self.stmts[next_index];

                    if matches!(next_event, FlatStmt::Choice { .. }) {
                        loop {
                            match next_event {
                                FlatStmt::EndDialogue => {
                                    if nested_count > 0 {
                                        nested_count -= 1
                                    } else {
                                        break;
                                    }
                                }
                                FlatStmt::Dialogue { .. }
                                | FlatStmt::Choice { .. }
                                | FlatStmt::If { .. } => nested_count += 1,
                                FlatStmt::EndChoice | FlatStmt::EndIf => nested_count -= 1,
                            }
                            next_index += 1;
                            next_event = &self.stmts[next_index];
                        }
                    }
                    self.index = next_index;
                    Some(Event::Ignore)
                }
                FlatStmt::If { condition } => {
                    let evaluation = eval_boolean_with_context(condition, self.context)
                        .unwrap_or_else(|_| panic!("Error evaluating '{condition}'"));

                    if evaluation {
                        self.index += 1;
                    } else {
                        let mut next_index = self.index + 1;
                        let mut nested_count = 0;

                        loop {
                            let next = &self.stmts[next_index];
                            match next {
                                FlatStmt::Dialogue { .. }
                                | FlatStmt::Choice { .. }
                                | FlatStmt::If { .. } => nested_count += 1,
                                FlatStmt::EndChoice | FlatStmt::EndDialogue => nested_count -= 1,
                                FlatStmt::EndIf => {
                                    if nested_count > 0 {
                                        nested_count -= 1
                                    } else {
                                        break;
                                    }
                                }
                            }
                            next_index += 1;
                        }
                        self.index = next_index;
                    }
                    Some(Event::Ignore)
                }
            },
            None => None,
        }
    }
}
