use std::collections::{HashSet, HashMap};

struct WordDict {
    dict: HashMap<char, i32>
}

impl WordDict {
    pub fn new(word: String) -> WordDict {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        WordDict { dict: map }
    }

    pub fn is_equal_to(&self, word_dict: &WordDict) -> bool {
        self.dict.len() == word_dict.dict.len() && self.dict.keys().all(|k| {
            word_dict.dict.contains_key(k) && word_dict.dict.get(k) == self.dict.get(k)
        })
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_dict = WordDict::new(word.to_string());
    let mut result: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        let word_dict2 = WordDict::new(possible_anagram.to_string());
        if word != *possible_anagram && word_dict.is_equal_to(&word_dict2) {
            result.insert(possible_anagram);
        }
    }
    result
}
