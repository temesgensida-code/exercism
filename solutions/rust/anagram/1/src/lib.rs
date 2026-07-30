use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[& 'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let mut sorted_word = lowercase_word.chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();
    
    possible_anagrams
        .iter()
        .copied()
        .filter(|&members|{
        members.len() == word.len()
        })

        .filter(|&members|{
            let lower_members = members.to_lowercase();
            lower_members != lowercase_word
        }) 

        .filter(|&members|{
            let mut sorted_members = members.to_lowercase().chars().collect::<Vec<char>>();
            sorted_members.sort_unstable();
            sorted_members == sorted_word
        })
        .collect()
        
}
