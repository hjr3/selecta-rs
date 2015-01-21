use std::str::StrExt;
use std::ascii::AsciiExt;

pub fn score(choice: &str, query: &str) -> f64 {
    if query.len() == 0 {
        return 1.0;
    }

    if choice.len() == 0 {
        return 0.0;
    }

    //let lower_choice = choice.into_ascii_lowercase();
    let lower_choice = choice.to_ascii_lowercase();
    let lower_query = query.to_ascii_lowercase();
    let lower_choice_len = lower_choice.len() as f64;

    println!("{} in {}", lower_query, lower_choice);
    let match_length = compute_match_length(lower_choice, lower_query.chars().collect());

    if match_length == 0 {
        return 0.0;
    }

    println!("{} / {}", lower_query.len(), match_length);
    let score = lower_query.len() as f64 / match_length as f64;

    println!("{} / {}", score, lower_choice_len);
    score / lower_choice_len
}

/// Find the length of the shortest substring matching the given characters.
fn compute_match_length(haystack: String, needles: Vec<char>) -> usize {
    let first_char = needles[0];
    let rest = needles.slice_from(1);
    let mut min_index = 0;

    let first_indexes = find_char_in_string(haystack.as_slice(), first_char);

    for first_index in first_indexes.iter() {
        println!("first_index = {}", first_index);
        let last_index = find_end_of_match(haystack.as_slice(), rest, *first_index);

        match last_index {
            Some(index) => {
                let new_index = index - *first_index + 1;
                println!("last_index = {}, new_index = {}, min_index = {}", index, new_index, min_index);
                if min_index == 0 || new_index < min_index {
                    min_index = new_index;
                }
            },
            None => {}
        }
    }

    println!("min_index = {}", min_index);
    min_index
}

/// Find all occurrences of the character in the string, returning their indexes.
fn find_char_in_string(haystack: &str, needle: char) -> Vec<usize> {
    let mut index: usize = 0;
    let mut indexes = Vec::new();
    let mut h = haystack.as_slice();

    println!("{}", "find_char_in_string");
    loop {
        index = match find_from_offset(h, needle, index) {
            Some(i) => {
                indexes.push(i);
                i + 1
            },
            None => { break; },
        };
    }

    indexes
}

/// Find each of the characters in the string, moving strictly left to right.
fn find_end_of_match(haystack: &str, needles: &[char], first_index: usize) -> Option<usize> {
    println!("at this point haystack = {}", haystack);
    let mut last_index = first_index;
    println!("{}", "find_end_of_match");
    for needle in needles.iter() {
        last_index = match find_from_offset(haystack, *needle, last_index + 1) {
            Some(i) => i,
            None => { return None; },
        };
    }

    Some(last_index)
}

/// Implements Ruby's `#index` method
fn find_from_offset(haystack: &str, needle: char, offset: usize) -> Option<usize> {
    let h = haystack.slice_from(offset);
    println!("from_from_offset: h = {}, needle = {}, offset = {}", h, needle, offset);

    let index = h.find(needle);

    match index {
        Some(i) => Some(i + offset),
        None => { return None; },
    }
}

#[test]
fn test_scores_zero_when_choice_is_empty() {
    assert!(score("", "a") == 0.0);

}

#[test]
fn test_scores_one_when_query_is_empty() {
    assert!(score("a", "") == 1.0);
}

#[test]
fn test_scores_zero_when_the_query_longer_than_choice() {
    assert!(score("short", "longer") == 0.0);
}

#[test]
fn test_scores_zero_when_query_does_not_match_at_all() {
    assert!(score("a", "b") == 0.0);
}

#[test]
fn test_scores_zero_when_only_prefix_of_query_matches() {
    assert!(score("ab", "ac") == 0.0);
}

#[test]
fn test_scores_greater_than_zero_when_matches() {
    let given_choices: Vec<&str> = vec!("a", "ab", "ba", "bab");

    for choice in given_choices.iter() {
        assert!(score(*choice, "a") > 0.0);
    }

    assert!(score("babababab", "aaaa") > 0.0);
}

#[test]
fn test_scores_1_normalized_to_length_when_the_query_equals_choice() {
    assert!(score("a", "a") == 1.0);
    assert!(score("ab", "ab") == 0.5);
    assert!(score("a long string", "a long string") == 
            1.0 / "a long string".len() as f64);
    assert!(score("spec/search_spec.rb", "sear") == 
            1.0 / "spec/search_spec.rb".len() as f64);
}

#[test]
fn test_matches_punctuation() {
    assert!(score("/! symbols $^", "/!$^") > 0.0);
}

#[test]
fn test_is_case_insensitive() {
    assert!(score("a", "A") == 1.0);
    assert!(score("A", "a") == 1.0);
}

#[test]
fn test_does_not_match_when_same_letter_is_repeated_in_choice() {
    assert!(score("a", "aa") == 0.0);
}

#[test]
fn test_scores_higher_for_better_matches() {
    assert!(score("selecta.gemspec", "asp") > score("algorithm4_spec.rb", "asp"));
    assert!(score("README.md", "em") > score("benchmark.rb", "em"));
    assert!(score("search.rb", "sear") > score("spec/search_spec.rb", "sear"));
}

#[test]
fn test_scores_shorter_matches_higher() {
    assert!(score("fbb", "fbb") > score("foo bar baz", "fbb"));
    assert!(score("foo", "foo") > score("longer foo", "foo"));
    assert!(score("foo", "foo") > score("foo longer", "foo"));
    assert!(score("1/2/3/4", "1/2/3") > score("1/9/2/3/4", "1/2/3"));
}

#[test]
fn test_sometimes_score_longer_strings_higher_if_better_match() {
    assert!(score("long 12 long", "12") > score("1 long 2", "12"));
}

#[test]
fn test_scores_higher_of_two_matches_regardless_of_order() {
    let tight = "12";
    let loose = "1padding2";
    let expect1 = tight.to_string() + loose;
    let expect2 = loose.to_string() + tight;

    assert!(score(expect1.as_slice(), "12") == 1.0 / expect1.len() as f64);
    assert!(score(expect2.as_slice(), "12") == 1.0 / expect2.len() as f64);

}
