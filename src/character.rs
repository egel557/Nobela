use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Characters = Vec<Character>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Character {
    id: String,
    display_name: String,
    aliases: HashMap<String, String>,
    portraits: HashMap<String, String>,
}

impl Character {
    pub fn new(
        id: &str,
        display_name: &str,
        aliases: HashMap<String, String>,
        portraits: HashMap<String, String>,
    ) -> Self {
        Character {
            id: id.to_owned(),
            display_name: display_name.to_owned(),
            aliases,
            portraits,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn get_portrait_path(&self, portrait_name: &str) -> Option<String> {
        let portrait = self.portraits.get(portrait_name);
        portrait.map(|portrait| portrait.to_owned())
    }

    pub fn get_alias_name(&self, alias_id: &str) -> Option<String> {
        let alias = self.aliases.get(alias_id);
        alias.map(|alias| alias.to_owned())
    }

    pub fn portraits(&self) -> &HashMap<String, String> {
        &self.portraits
    }
}
