use std::cmp::min;

pub struct Configuration {
    visible_choices: u8,
    initial_search: String,
    choices: Vec<String>,
}

impl Configuration {
    pub fn from_inputs(choices: Vec<String>, screen_height: u16) -> Configuration {
        let visible_choices = min(20, screen_height as u8);
        Configuration::new(visible_choices, String::from_str(""), choices)
    }

    pub fn new(visible_choices: u8, initial_search: String, choices: Vec<String>) -> Configuration {
        Configuration { visible_choices: visible_choices,
                        initial_search: initial_search,
                        choices: choices }
    }

    // TODO should i be using clone here?
    pub fn get_choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    pub fn get_initial_search(&self) -> String {
        self.initial_search.clone()
    }

    pub fn get_visible_choices(&self) -> u8 {
        self.visible_choices
    }
}
