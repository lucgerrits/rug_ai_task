use clap::Parser;
use log::{error as log_error, info};
mod tools;
mod task2;

#[derive(Parser, Debug)]
#[command(name = "Some rust for rug ai")]
#[command(version = "1.0")]
#[command(about = "Rust is good, rust is great :)", long_about = None)]
struct Cli {
    #[clap(short, long, value_name = "FILE")]
    filename: Option<String>,

    #[clap(long, value_name = "N")]
    find_n_common_words: Option<usize>,
}

/// Main function for task 4
/// This is in a separate file from main.rs to show how the "task modules" can be used
pub fn main() {
    tools::load_and_check_env();
    let cli = Cli::parse();
    let file_data;

    if let Some(filename) = cli.filename {
        let data_filename = filename.as_str();
        file_data = task2::get_merged_file(&data_filename);
        info!("File {} size: {} Bytes", data_filename, file_data.len());
    } else {
        log_error!("Filename is required");
        return;
    }

    if let Some(n) = cli.find_n_common_words {
        let word_frequencies = task2::calculate_word_frequencies(&file_data);
        let common_words = task2::top_n_common_words(&word_frequencies, n);
        println!("Most common words (word,count): {:?}", common_words);
    }
    info!("Done");
}
