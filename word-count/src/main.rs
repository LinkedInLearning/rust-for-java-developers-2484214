use std::collections::HashMap;

static TEXT: &str = "
hello, world!
how are you?
i'm good, how are you?
i'm good too. i hope that you are doing well.
";

fn tokenize(text: &str) -> impl Iterator<Item = &str> {
    // Split input text on whitespace
    text.split_whitespace()
        // Strip punctuation from tokens
        .map(|x| x.trim_matches(|c: char| c.is_ascii_punctuation()))
        // Set minimum length of 2
        .filter(|x| x.len() > 2)
}

fn main() {
    let word_counts = tokenize(TEXT)
        .map(|token| (token, 1))
        .fold(
            HashMap::<&str, usize>::new(),
            |mut counts, (token, count)| {
                *counts.entry(token).or_insert(0) += count;
                counts
        },
    );

    for (word, count) in word_counts.iter() {
        println!("{}: {}", word, count);
    }
}
