use std::collections::HashMap;

pub type Characters = Vec<Character>;
pub struct Character {
    id: String,
    display_name: String,
    aliases: HashMap<String, String>,
}

impl Character {
    pub fn new(id: &str, display_name: &str, aliases: HashMap<String, String>) -> Self {
        Character {
            id: id.to_owned(),
            display_name: display_name.to_owned(),
            aliases,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn get_alias_name(&self, alias_id: &str) -> Option<&String> {
        self.aliases.get(alias_id)
    }
}
