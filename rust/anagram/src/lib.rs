use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted_word = sort_word(&lower_word);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let lower_candidate = candidate.to_lowercase();
            lower_candidate.len() == lower_word.len()
                && lower_candidate != lower_word
                && sort_word(&lower_candidate) == sorted_word

        })
        .copied()
        .collect()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort_unstable();
    sorted_word
}
