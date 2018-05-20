extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let tokens = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();

    let cache = create_cache(tokens);
    let text = generate_text(cache, 500);
    println!("{}", text);
}

pub fn create_cache(tokens: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut cache = HashMap::new();

    for i in 0..tokens.len() - 2 {
        let first = tokens[i].clone();
        let second = tokens[i + 1].clone();
        let item = tokens[i + 2].clone();

        let key = format!("{} {}", first, second);

        if !cache.contains_key(&key) {
            cache.insert(key, vec![item]);
        } else {
            cache.get_mut(&key).unwrap().push(item);
        }
    }

    cache
}

pub fn generate_text(cache: HashMap<String, Vec<String>>, num_words: i32) -> String {
    // Random number, range rng.gen_range(0, 5);

    let mut output = vec![];

    // Choose a random seed key
    let mut rng = rand::thread_rng();
    let mut keys = cache.keys();
    let random_idx = rng.gen_range(0, keys.len());

    // Our random key
    let seed_key = keys.nth(random_idx).unwrap();

    let words: Vec<&str> = seed_key.split(" ").collect();
    let mut first_word = String::from(words[0]);
    let mut second_word = String::from(words[1]);

    for _ in 0..num_words {
        let key = format!("{} {}", first_word, second_word);

        // Here, there can be a terminal key!
        let options = match cache.get(&key) {
            Some(opt) => opt,
            None => {
               return output.join(" ") 
            }
        };
        
        let new_word_idx = rng.gen_range(0, options.len());
        let new_word = options[new_word_idx].clone();

        output.push(first_word);

        first_word = second_word;
        second_word = new_word;
    }

    output.join(" ")
}