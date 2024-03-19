use clap::Parser;
use log::{error as log_error, info};
use std::fs;
use tokio::task;
mod task2;
mod tools;

#[derive(Parser, Debug)]
#[command(name = "Some rust for rug ai")]
#[command(version = "1.0")]
#[command(about = "Rust is good, rust is great :)", long_about = None)]
struct Cli {
    #[clap(short, long, value_name = "word")]
    grep_word_in_files: Option<String>,
}

/// Main function for task 5
/// This is in a separate file from main.rs to show how the "task modules" can be used
#[tokio::main]
pub async fn main() {
    tools::load_and_check_env();
    let cli = Cli::parse();
    let cache_dir = "cache";

    if let Some(word) = cli.grep_word_in_files {
        info!("Searching for word: {}", word);
        search_word_in_files(&word, cache_dir).await;
    } else {
        log_error!("Word is required");
        return;
    }
    info!("Done");
}

async fn search_word_in_files(word: &str, cache_dir: &str) {
    let mut tasks = Vec::new();

    // Read the directory and get a list of files
    let entries = fs::read_dir(cache_dir).expect("Failed to read directory");

    // Iterate over the directory entries
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        let w = word.to_string(); //had to do this to avoid borrowing issues

        // Spawn a new task to search for the word in each file
        let task = task::spawn(async move {
            let file_content = fs::read_to_string(&path).expect("Failed to read file");

            if file_content.contains(w.as_str()) {
                // File contains the word, do something
                println!("Found word '{}' in file: {:?}", w, path);
            }
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Failed to join task");
    }
}
