// Task 1
// Load the text files

use log::info;
use reqwest;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering}, // some atomic boolean 
    Arc, // Arc is a thread-safe reference-counting pointer used to share ownership of data between threads
};
use tokio::sync::Mutex; // a mutex to lock the variables
use tokio::sync::Semaphore; // a semaphore to limit the number of concurrent tasks

/// Do the task 1
pub async fn do_task1() {
    info!("Load the text files");
    let merged_filename = "merged_files.txt";
    let cache_dir = std::env::var("CACHE_DIR").unwrap_or("cache".to_string());

    if std::env::var("CACHE_MERGED_DATA").unwrap() != "true" {
        // delete the file if CACHE_MERGED_DATA is not true
        info!("Deleting {}", merged_filename);
        delete_file(&merged_filename);
        delete_cache_dir(&cache_dir);
    }
    if !std::path::Path::new(&merged_filename).exists() {
        info!("merged_files.txt not found, getting files....");
        create_cache_dir_if_not_exists(&cache_dir);
        get_files(&merged_filename, &cache_dir).await;
    } else {
        info!("File merged_files.txt exists")
    }
    info!("Task 1: Done");
}

/// Delete the file merged_files.txt
fn delete_file(file: &str) {
    let _ = std::fs::remove_file(file);
}
fn delete_cache_dir(dir: &str) {
    let _ = std::fs::remove_dir_all(dir);
}
fn create_cache_dir_if_not_exists(dir: &str) {
    if !std::path::Path::new(dir).exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}
/// Get the files from the base_data_url
async fn get_files(filename: &str, cache_dir: &str) {
    // Note 1:
    // I'm doing this code in a area with high pings and not sure if the first task
    // is asked to be done in parallel or not, it would take forever to test all files otherwise.
    // But it's better anyway in parallel (and slower ping + parallel -> faster)


    // Note 2:
    // Now all is the files are stored in memory in file_contents vetor
    // If file_contents is too large, it will consume a lot of memory
    // So to be even more safe, we could add a max file_contents size
    // and write the file_contents to disk when it reaches the max size
    // and then clear file_contents and continue to get the files

    let base_data_url = std::env::var("BASE_DATA_URL").unwrap();
    info!("BASE_DATA_URL: {}", base_data_url);
    let max_threads = std::env::var("MAX_THREADS")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    info!("MAX_THREADS: {}", max_threads);

    // Load file 1.txt up to file not found, in parallel with a maximum of max_threads
    // Append in file content in a single file in the order of the file number
    let semaphore = Arc::new(Semaphore::new(max_threads as usize)); // a semaphore to limit the number of concurrent tasks
    let file_contents = Arc::new(Mutex::new(Vec::new())); // a vector of file contents, so we can keep the order of the files
    let mut i = 1; // some file number
    let mut tasks = Vec::new(); // a vector of tasks
    let stop_signal = Arc::new(AtomicBool::new(false)); // a stop signal: true if a file is not found
    
    while !stop_signal.load(Ordering::SeqCst) {
        // Clone the variables to be used in the task
        let base_data_url = base_data_url.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let task_file_contents = file_contents.clone();
        let task_stop_signal = stop_signal.clone();
        let cache_dir = cache_dir.to_string();

        // Spawn a task
        let task = tokio::spawn(async move {
            // Check if the stop signal is true
            if task_stop_signal.load(Ordering::SeqCst) {
                permit.forget();
                return Ok(());
            }

            let url = format!("{}/{}.txt", base_data_url, i);
            let response = reqwest::get(&url).await;
            match response {
                Ok(response) => {
                    info!("File {}.txt found", i);
                    let content = response.text().await.unwrap();
                    // store content (i, content)
                    let mut f = task_file_contents.lock().await;
                    f.push((i, content.clone())); // need to clone the content to be able to use it below. The perf cost is relatively low here
                    // write the file to disk in cache dir
                    let mut file = File::create(format!("{}/{}.txt", cache_dir, i)).unwrap();
                    file.write_all(content.as_bytes()).unwrap();
                    drop(permit);
                }
                Err(_) => {
                    info!("File {}.txt not found", i);
                    // We presume that if a file is not found, the next files are not found too and thus we stop
                    task_stop_signal.store(true, Ordering::SeqCst);
                    return Err(i);
                }
            }
            // drop(permit);
            Ok(())
        });
        tasks.push(task);
        i += 1;

        if stop_signal.load(Ordering::SeqCst) {
            // stop the loop if a file is not found
            break;
        }
    }

    for task in tasks {
        let _ = task.await;
    }

    let contents = file_contents.lock().await;
    let mut file = File::create(filename).unwrap();
    // write all in order in one file
    for (_, content) in contents.iter() {
        file.write_all(content.as_bytes()).unwrap();
    }

    info!("Task 1 done");
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_files() {
      // TODO: Add test for get_files
    }

    #[test]
    fn test_delete_file() {
        let filename = "test_delete_file.txt";
        let mut file = File::create(filename).unwrap();
        file.write_all(b"test").unwrap();
        assert!(std::path::Path::new(filename).exists());
        delete_file(filename);
        assert!(!std::path::Path::new(filename).exists());
    }

    #[test]
    fn test_delete_cache_dir() {
        let dir = "test_delete_cache_dir";
        std::fs::create_dir_all(dir).unwrap();
        assert!(std::path::Path::new(dir).exists());
        delete_cache_dir(dir);
        assert!(!std::path::Path::new(dir).exists());
    }

    #[test]
    fn test_create_cache_dir_if_not_exists() {
        let dir = "test_create_cache_dir_if_not_exists";
        assert!(!std::path::Path::new(dir).exists());
        create_cache_dir_if_not_exists(dir);
        assert!(std::path::Path::new(dir).exists());
        delete_cache_dir(dir);
    }
}