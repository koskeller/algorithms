#![allow(unused)]
use std::collections::HashMap;

pub fn hash_char(c: char) -> usize {
    match c {
        'a' | 'b' | 'c' => 2,
        'd' | 'e' | 'f' => 3,
        'g' | 'h' | 'i' => 4,
        'j' | 'k' | 'l' => 5,
        'm' | 'n' | 'o' => 6,
        'p' | 'q' | 'r' | 's' => 7,
        't' | 'u' | 'v' => 8,
        'w' | 'x' | 'y' | 'z' => 9,
        _ => {
            // Handle cases for other characters not specified above
            // If needed, you can return a default value or perform additional actions
            0 // Default value for unmatched characters
        }
    }
}

pub struct Dict {
    map: HashMap<Vec<usize>, Vec<String>>,
}

impl Dict {
    pub fn new() -> Self {
        Dict {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut hash = Vec::new();
        for c in word.chars() {
            hash.push(hash_char(c));
        }
        self.map
            .entry(hash)
            .and_modify(|v| v.push(word.clone()))
            .or_insert(vec![word]);
    }

    pub fn get_words(&self, seq: Vec<usize>) -> Option<&Vec<String>> {
        self.map.get(&seq)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut dict = Dict::new();
        dict.insert("any".to_string());
        dict.insert("box".to_string());
        dict.insert("boy".to_string());

        assert_eq!(
            dict.get_words(vec![2, 6, 9]).unwrap(),
            &vec!["any".to_string(), "box".to_string(), "boy".to_string()]
        );

        assert!(dict.get_words(vec![2, 9, 6]).is_none());
    }
}
