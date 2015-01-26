use config::Configuration;
use score::score;
use std::cmp::min;

pub struct Search {
    config: Configuration,
    choices: Vec<String>,
    index: u64,
    query: String,
    done: bool,
    matches: Vec<String>,
}

impl Search {
    pub fn new(config: Configuration,
           choices: Vec<String>,
           index: u64,
           query: String,
           done: bool,
           matches: Option<Vec<String>>) -> Search {

        let m = match matches {
            Some(m) => m,
            _ => compute_matches(&choices, query.as_slice())
        };

        Search { config: config,
                 choices: choices,
                 index: index,
                 query: query,
                 done: done,
                 matches: m }
    }

    pub fn blank(config: Configuration) -> Search {
        let choices = config.get_choices();
        let initial_search = config.get_initial_search();

        Search::new(config, 
                    choices,
                    0,
                    initial_search,
                    false,
                    None)
    }

    pub fn down(self) -> Search {
        let max_visible_choices = self.max_visible_choices();

        if max_visible_choices > 0 {
            Search::new(self.config,
                        self.choices,
                        (self.index + 1) % max_visible_choices,
                        self.query,
                        self.done,
                        None)
        } else {
            self
        }
    }

    pub fn up(self) -> Search {
        let max_visible_choices = self.max_visible_choices();

        if max_visible_choices > 0 {
            let mut index;

            // Rust handles negative modulo differently than ruby
            if self.index == 0 {
                index = max_visible_choices - 1;
            } else {
                index = self.index - 1;
            }

            Search::new(self.config,
                        self.choices,
                        index,
                        self.query,
                        self.done,
                        None)
        } else {
            self
        }
    }

    fn max_visible_choices(&self) -> u64 {
        min(self.config.get_visible_choices() as usize, self.matches.len()) as u64
    }

    pub fn append_search_string(self, string: &str) -> Search {

        let search = Search::new(self.config, 
                    self.choices,
                    0,
                    self.query + string,
                    self.done,
                    None);

        search
    }

    pub fn backspace(&mut self) -> &Search {
        self.query.pop();
        self
    }

    pub fn clear_query(&mut self) -> &Search {
        self.query.truncate(0);
        self
    }

    pub fn delete_word(&mut self) -> &Search {
        let re = regex!(r"[^ ]* *$");
        self.query = re.replace(self.query.as_slice(), "");
        self
    }

    pub fn selection(&self) -> Option<&String> {
        self.matches.get(self.index as usize)
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done(&mut self) -> &Search {
        self.done = true;
        self
    }
}

fn compute_matches(choices: &Vec<String>, query: &str) -> Vec<String> {
    choices.iter().map(|choice|
        (choice, score(choice.as_slice(), query))
    ).filter(|&(_choice, score)|
        score > 0.0
    ).map(|(choice, _score)| {
        // we have to clone here unless we want to pass 
        // the matches around as references everywhere.
        choice.clone()
    }).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use config::Configuration;

    fn get_test_config() -> Configuration {
        let choices: Vec<String> = vec!(String::from_str("one"),
        String::from_str("two"),
        String::from_str("three"));

        Configuration::from_inputs(choices, 21)
    }

    fn get_blank_search() -> Search {
        let config = get_test_config();
        Search::blank(config)
    }

    #[test]
    fn test_blank_search() {
        let _search = get_blank_search();
    }

    #[test]
    fn test_selects_first_choice() {
        let search = get_blank_search();
        assert!(search.selection().unwrap().as_slice() == "one");
    }

    #[test]
    fn test_moves_down_list() {
        let search = get_blank_search();
        assert!(search.down().selection().unwrap().as_slice() == "two");
    }

    #[test]
    fn test_moves_up() {
        let search = get_blank_search();
        assert!(search.down().up().selection().unwrap().as_slice() == "one");
    }

    #[test]
    fn test_loops_around_when_reaching_end_of_the_list() {
        let search = get_blank_search();
        assert!(search.down().down().down().down().selection().unwrap().as_slice() == "two");
    }

    #[test]
    fn test_loops_around_when_reaching_top_of_the_list() {
        let search = get_blank_search();
        assert!(search.up().up().selection().unwrap().as_slice() == "two");
    }

    #[test]
    fn test_loops_around_when_reaching_visible_choice_limit() {
        let choices: Vec<String> = vec!(String::from_str("one"),
        String::from_str("two"),
        String::from_str("three"));

        let config = Configuration::new(2, "".to_string(), choices);
        let search = Search::blank(config);
        assert!(search.down().down().down().selection().unwrap().as_slice() == "two");
    }

    #[test]
    fn test_filtered_search_results_moves_up_down_list() {
        let search = get_blank_search().append_search_string("t");
        assert!(search.down().selection().unwrap().as_slice() == "three");

        let search = get_blank_search().append_search_string("t");
        assert!(search.up().selection().unwrap().as_slice() == "three");
    }

    #[test]
    fn test_filtered_search_results_loops_around_when_reaching_the_end() {
        let search = get_blank_search().append_search_string("t");
        assert!(search.down().down().selection().unwrap().as_slice() == "two");

        let search = get_blank_search().append_search_string("t");
        assert!(search.up().selection().unwrap().as_slice() == "three");
    }

    #[test]
    fn test_everything_filtered_out_cannot_move_up_or_down() {
        let search = get_blank_search().append_search_string("zzz");
        assert!(search.down().selection().is_none());

        let search = get_blank_search().append_search_string("zzz");
        assert!(search.up().selection().is_none());
    }

    #[test]
    fn test_nothing_matches() {
        let search = get_blank_search().append_search_string("doesnt-mtch");
        assert!(search.selection().is_none());
    }

    #[test]
    fn test_backspaces_over_characters_and_resets_index() {
        let mut search = get_blank_search();
        search = search.append_search_string("e");

        assert!(search.get_query().as_slice() == "e");
        assert!(search.backspace().get_query().as_slice() == "");
        assert!(search.get_index() == 0);

    }

    #[test]
    fn test_deletes_words() {
        assert!(get_blank_search().append_search_string("").delete_word().get_query().as_slice() == "");
        assert!(get_blank_search().append_search_string("a").delete_word().get_query().as_slice() == "");
        assert!(get_blank_search().append_search_string("a ").delete_word().get_query().as_slice() == "");
        assert!(get_blank_search().append_search_string("a b").delete_word().get_query().as_slice() == "a ");
        assert!(get_blank_search().append_search_string("a b ").delete_word().get_query().as_slice() == "a ");
        assert!(get_blank_search().append_search_string(" a b").delete_word().get_query().as_slice() == " a ");
    }

    #[test]
    fn test_clears_query() {
        let given_queries = ["", "a", "a ", "a b", "a b ", " a b"];

        for query in given_queries.iter() {
            let search = get_blank_search();
            assert!(search.append_search_string(*query).clear_query().get_query().as_slice() == "");
        }
    }

    #[test]
    fn test_only_returns_matching_choices() {
    }

    #[test]
    fn test_sorts_the_choices_by_score() {
    }

    #[test]
    fn test_knows_when_done() {
        let mut search = get_blank_search();
        assert!(search.is_done() == false);
        assert!(search.done().is_done() == true);
    }
}
