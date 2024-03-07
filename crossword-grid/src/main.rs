use std::collections::HashSet;

use rand::Rng;

// https://gist.github.com/shmookey/b28e342e1b1756c4700f42f17102c2ff
static WORDS: &str = include_str!("WORDS");

fn main() {
    let mut rng = rand::thread_rng();

    let ordered_words: Vec<_> = WORDS.split_ascii_whitespace().collect();
    let words: HashSet<_> = ordered_words.iter().cloned().collect();

    let mut random_word = || {
        let i = rng.gen_range(0..ordered_words.len());
        ordered_words[i]
    };

    let mut rows = [&str; 5];
    let mut columns = [&str; 5];
    let start_word = random_word();



    println!("{start_word}");
}
