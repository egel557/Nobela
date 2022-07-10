use std::collections::HashMap;

use evalexpr::{eval_boolean_with_context, HashMapContext};

use crate::Stmt;

pub type Timeline = Vec<Stmt>;

pub trait Stack<T> {
    fn peek(&self) -> Option<&T>;
    fn set_top(&mut self, new_val: T);
}

impl<T> Stack<T> for Vec<T> {
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self[self.len() - 1])
        }
    }

    fn set_top(&mut self, new_val: T) {
        let last_index = self.len() - 1;
        self[last_index] = new_val
    }
}
pub enum Event {
    Dialogue {
        character_id: Option<String>,
        speaker: Option<String>,
        text: String,
        choices: Vec<(String, bool)>,
    },
    Ignore,
}

pub struct Config<'a> {
    pub timelines: HashMap<String, &'a Timeline>,
    pub timeline_stack: Vec<&'a Timeline>,
    pub index_stack: Vec<usize>,
    pub context: &'a HashMapContext,
}

pub struct Server<'a> {
    timelines: HashMap<String, &'a Timeline>,
    timeline_stack: Vec<&'a Timeline>,
    index_stack: Vec<usize>,
    choice_indexes: Option<Vec<usize>>,
    context: &'a HashMapContext,
}

impl<'a> Server<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Server {
            timelines: config.timelines,
            timeline_stack: config.timeline_stack,
            index_stack: config.index_stack,
            choice_indexes: None,
            context: config.context,
        }
    }

    pub fn choose(&mut self, choice: usize) {
        self.index_stack.set_top(
            *self
                .choice_indexes
                .as_ref()
                .expect("No choices.")
                .get(choice)
                .expect("Invalid choice index"),
        );
    }
}

impl Iterator for Server<'_> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // let increment_top_index = || self.index_stack.set_top(self.index_stack.peek().unwrap() + 1);

        match self.timeline_stack.peek() {
            Some(timeline) => {
                let mut new_timeline_name: Option<String> = None;
                let mut jump: Option<bool> = None;

                let index = *self.index_stack.peek().unwrap();
                let curr = &timeline[index];
                let event = match curr {
                    Stmt::Dialogue {
                        character_id,
                        speaker,
                        text,
                    } => {
                        let mut next_index = index + 1;
                        let mut choices = Vec::new();
                        let mut choice_indexes = Vec::new();
                        let mut nested_count = 0;

                        loop {
                            let next_event = &timeline[next_index];
                            match next_event {
                                Stmt::EndDialogue => {
                                    if nested_count > 0 {
                                        nested_count -= 1
                                    } else {
                                        break;
                                    }
                                }
                                Stmt::Dialogue { .. } => nested_count += 1,
                                Stmt::Choice { .. } => {
                                    if nested_count > 0 {
                                        nested_count += 1
                                    } else {
                                        choices.push(next_event);
                                        choice_indexes.push(next_index.to_owned())
                                    }
                                }
                                Stmt::EndChoice => {
                                    if nested_count > 0 {
                                        nested_count -= 1
                                    }
                                }
                                Stmt::If { .. } => nested_count += 1,
                                Stmt::EndIf => nested_count -= 1,
                                Stmt::Call { .. } => (),
                            }
                            next_index += 1;
                        }

                        self.choice_indexes = Some(choice_indexes);
                        self.index_stack.set_top(index + 1);
                        // self.index += 1;
                        Some(Event::Dialogue {
                            character_id: character_id.to_owned(),
                            speaker: speaker.to_owned(),
                            text: text.to_owned(),
                            choices: choices
                                .into_iter()
                                .map(|c| {
                                    if let Stmt::Choice { text, condition } = c {
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
                    Stmt::Choice { .. } | Stmt::EndDialogue | Stmt::EndIf => {
                        // self.index += 1;
                        self.index_stack.set_top(index + 1);
                        Some(Event::Ignore)
                    }
                    Stmt::EndChoice => {
                        let mut next_index = index + 1;
                        let mut nested_count = 0;
                        let mut next_event = &timeline[next_index];

                        if matches!(next_event, Stmt::Choice { .. }) {
                            loop {
                                match next_event {
                                    Stmt::EndDialogue => {
                                        if nested_count > 0 {
                                            nested_count -= 1
                                        } else {
                                            break;
                                        }
                                    }
                                    Stmt::Dialogue { .. }
                                    | Stmt::Choice { .. }
                                    | Stmt::If { .. } => nested_count += 1,
                                    Stmt::EndChoice | Stmt::EndIf => nested_count -= 1,
                                    Stmt::Call { .. } => (),
                                }
                                next_index += 1;
                                next_event = &timeline[next_index];
                            }
                        }
                        self.index_stack.set_top(next_index);
                        // self.index = next_index;
                        Some(Event::Ignore)
                    }
                    Stmt::If { condition } => {
                        let evaluation = eval_boolean_with_context(condition, self.context)
                            .unwrap_or_else(|_| panic!("Error evaluating '{condition}'"));

                        if evaluation {
                            // self.index += 1;
                            self.index_stack.set_top(index + 1);
                        } else {
                            let mut next_index = index + 1;
                            let mut nested_count = 0;

                            loop {
                                let next = &timeline[next_index];
                                match next {
                                    Stmt::Dialogue { .. }
                                    | Stmt::Choice { .. }
                                    | Stmt::If { .. } => nested_count += 1,
                                    Stmt::EndChoice | Stmt::EndDialogue => nested_count -= 1,
                                    Stmt::EndIf => {
                                        if nested_count > 0 {
                                            nested_count -= 1
                                        } else {
                                            break;
                                        }
                                    }
                                    Stmt::Call { .. } => (),
                                }
                                next_index += 1;
                            }
                            // self.index = next_index;
                            self.index_stack.set_top(next_index);
                        }
                        Some(Event::Ignore)
                    }
                    Stmt::Call {
                        jump: j,
                        timeline_name,
                    } => {
                        jump = Some(*j);
                        new_timeline_name = Some(timeline_name.to_owned());
                        self.index_stack.set_top(index + 1);
                        Some(Event::Ignore)
                    }
                };

                if timeline.len() <= *self.index_stack.peek().unwrap() {
                    self.timeline_stack.pop();
                    self.index_stack.pop();
                }

                if let Some(new_timeline_name) = new_timeline_name {
                    if jump.unwrap() {
                        self.timeline_stack.pop();
                        self.index_stack.pop();
                    }

                    let new_timeline = self
                        .timelines
                        .get(&new_timeline_name)
                        .unwrap_or_else(|| panic!("Timeline '{new_timeline_name}' not found."));
                    self.timeline_stack.push(new_timeline);
                    self.index_stack.push(0);
                }

                // self.timeline_stack.push(todo!());

                event
            }
            None => None,
        }
    }
}
