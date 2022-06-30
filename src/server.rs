use crate::FlatStmt;

pub enum Event {
    Dialogue {
        speaker: Option<String>,
        text: String,
        choices: Vec<String>,
    },
    Ignore,
}

pub struct Server<'a> {
    stmts: &'a Vec<FlatStmt>,
    index: usize,
    choice_indexes: Option<Vec<usize>>,
}

impl<'a> Server<'a> {
    pub fn new(stmts: &'a Vec<FlatStmt>, index: usize) -> Self {
        Server {
            stmts,
            index,
            choice_indexes: None,
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
                            FlatStmt::If { condition } => todo!(),
                            FlatStmt::EndIf => todo!(),
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
                                if let FlatStmt::Choice { text, .. } = c {
                                    text.to_owned()
                                } else {
                                    unreachable!()
                                }
                            })
                            .collect(),
                    })
                }
                FlatStmt::Choice { .. } => {
                    self.index += 1;
                    Some(Event::Ignore)
                }
                FlatStmt::EndDialogue => {
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
                                FlatStmt::If { condition } => todo!(),
                                FlatStmt::EndIf => todo!(),
                            }
                            next_index += 1;
                            next_event = &self.stmts[next_index];
                        }
                    }
                    self.index = next_index;
                    Some(Event::Ignore)
                }
                FlatStmt::If { condition } => todo!(),
                FlatStmt::EndIf => todo!(),
            },
            None => None,
        }
    }
}
