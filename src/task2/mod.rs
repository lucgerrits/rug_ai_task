use log::{error as log_error, info};
use rayon::prelude::*; // found this crate to parallelize iteration directly on the go
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// #[path = "../lib/spacy.rs"]
// mod spacy;
// use spacy::doc::Callable;

#[allow(dead_code)]
pub async fn do_task2() {
    let file_data = get_merged_file("merged_files.txt");
    // print file size
    println!("File size: {} Bytes", file_data.len());
    let word_frequencies = calculate_word_frequencies(&file_data);
    let common_words = top_n_common_words(&word_frequencies, 10);
    println!("Most common words (word,count): {:?}", common_words);
    let unique_words = word_with_unique_occurrences(&word_frequencies);
    println!(
        "Words with unique occurrences length: {}",
        unique_words.len()
    );
    info!("complexity_of_sentences() is a WIP. See code for details.");
    // complexity_of_sentences(&file_data);
    info!("Task 2 done");
}

pub fn get_merged_file(file: &str) -> String {
    if !std::path::Path::new(file).exists() {
        log_error!("File {} not found", file);
        return String::new();
    }
    std::fs::read_to_string(file).unwrap()
}

pub fn top_n_common_words(
    word_frequencies: &HashMap<String, usize>,
    n: usize,
) -> Vec<(&String, &usize)> {
    // Sort and find the n most common words
    let mut words: Vec<_> = word_frequencies.into_iter().collect();
    words.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    //return something that can be manipulated later
    words.into_iter().take(n).collect()
}

/// Find the words with unique occurrences
pub fn word_with_unique_occurrences(word_frequencies: &HashMap<String, usize>) -> Vec<&String> {
    // Filter out the ones that have a frequency == 1
    word_frequencies
        .into_iter()
        .filter(|(_, &v)| v == 1)
        .map(|(k, _)| k)
        .collect()
}

pub fn calculate_word_frequencies(text: &str) -> HashMap<String, usize> {
    let word_re = Regex::new(r"\b\w+\b").unwrap();
    let frequencies = Arc::new(Mutex::new(HashMap::new()));

    // //This bit duplicates the text and can take a lot of memory
    // // TODO: find a way to avoid this duplication
    // let chunks: Vec<&str> = text.split_whitespace().collect();
    // chunks.par_iter().for_each(|&chunk| {
    //     // parallel iteration using par_iter from rayon
    //     for word in word_re
    //         .find_iter(chunk)
    //         .map(|mat| mat.as_str().to_lowercase())
    //     {
    //         let mut freqs = frequencies.lock().unwrap();
    //         *freqs.entry(word).or_insert(0) += 1;
    //     }
    // });

    // This avoids the duplication of data but its slower
    word_re.find_iter(text).par_bridge().for_each(|mat| {
        let word = mat.as_str().to_lowercase();
        let mut freqs = frequencies.lock().unwrap();
        *freqs.entry(word).or_insert(0) += 1;
    });

    let frequencies = Arc::try_unwrap(frequencies).unwrap().into_inner().unwrap();
    frequencies
}

#[allow(dead_code, unused_variables)]
/// complexity of sentences
fn complexity_of_sentences(text: &str) {
    // This is a WIP
    // Note: I wanted to use the spacy module to get some text analysis of the text
    // but I'm having trouble with the local module that come from https://github.com/dluman/rusTy
    // rusTy is also a WIP and hasn't been updated in a while

    // PS: It's indeed not very common to find a Rust NLP library similar to spaCy, that's why I find the rusTy bindings a good start
    // 
    //
    // let spacy = spacy::Module::init();
    // spacy.load("en_core_web_lg");
    // let doc = spacy::nlp(text);
    // let text_stats = doc.call("sentiment").kwargs(None).invoke(); //there needs to be a type annotation here, but even with it I get an error
    // println!("Text stats: {:?}", text_stats);
}

// Create tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_n_common_words() {
        let mut word_frequencies = HashMap::new();
        word_frequencies.insert("hello".to_string(), 1);
        word_frequencies.insert("world".to_string(), 2);
        word_frequencies.insert("foo".to_string(), 3);
        word_frequencies.insert("bar".to_string(), 4);
        word_frequencies.insert("baz".to_string(), 5);

        let common_words = top_n_common_words(&word_frequencies, 3);
        assert_eq!(common_words.len(), 3);
        assert_eq!(common_words[0].0, "baz");
        assert_eq!(common_words[1].0, "bar");
        assert_eq!(common_words[2].0, "foo");
    }

    #[test]
    fn test_word_with_unique_occurrences() {
        let mut word_frequencies = HashMap::new();
        word_frequencies.insert("hello".to_string(), 1);
        word_frequencies.insert("world".to_string(), 2);
        word_frequencies.insert("foo".to_string(), 1);
        word_frequencies.insert("bar".to_string(), 4);
        word_frequencies.insert("baz".to_string(), 5);

        let unique_words = word_with_unique_occurrences(&word_frequencies);
        assert_eq!(unique_words.len(), 2);
        assert!(unique_words.contains(&&"hello".to_string()));
        assert!(unique_words.contains(&&"foo".to_string()));
    }

    #[test]
    fn test_calculate_word_frequencies() {
        let text = "hello world foo bar baz world foo bar";
        let word_frequencies = calculate_word_frequencies(text);
        assert_eq!(word_frequencies.len(), 5);
        assert_eq!(word_frequencies.get("hello"), Some(&1));
        assert_eq!(word_frequencies.get("world"), Some(&2));
        assert_eq!(word_frequencies.get("foo"), Some(&2));
        assert_eq!(word_frequencies.get("bar"), Some(&2));
        assert_eq!(word_frequencies.get("baz"), Some(&1));
    }
}
