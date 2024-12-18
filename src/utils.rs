use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use ignore::WalkBuilder;

use std::{
    io::{stdout, Result},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

pub fn search_files(filename: &str) -> Vec<PathBuf> {
    /*
        rayon::ThreadPoolBuilder::new()
            .num_threads(20)
            .build_global()
            .unwrap();
    */
    let walker = WalkBuilder::new("/").build();

    let filecount = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    let results: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().contains(filename))
        .map(|entry| {
            filecount.fetch_add(1, Ordering::SeqCst);
            entry.path().to_path_buf()
        })
        .collect();
    /*
    let results: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .filter(|entry| entry.file_name().to_string_lossy().contains(filename))
        .map(|entry| {
            filecount.fetch_add(1, Ordering::SeqCst);
            entry.path().to_path_buf()
        })
        .collect();
    */

    let duration = start.elapsed().as_secs_f64();
    println!("The time taken to find your files is {duration} seconds");
    results
}
pub fn display_and_select(result: Vec<PathBuf>) -> Option<String> {
    println!("Found {} files containing '{}'", result.len(), "filename");

    println!("Please select a file to open (or 'q' to quit):");
    None
}

#[allow(dead_code)]
pub fn clear_terminal() -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All) // Correct enum variant
    )?;
    Ok(())
}
