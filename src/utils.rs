use colored::*;
use ignore::WalkBuilder;
use inquire::{error::InquireError, Select};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
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
    let lookedcount = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    let results: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            lookedcount.fetch_add(1, Ordering::SeqCst);
            entry.file_name().to_string_lossy().contains(filename)
        })
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
    println!(
        "{}",
        format!(
            "I looked at {} files in your device 💻 in {} seconds {}",
            lookedcount.load(Ordering::SeqCst).to_string().green(),
            duration.to_string().green(),
            "⚡"
        )
        .cyan()
        .bold()
    );
    results
}

pub fn display_and_select(result: Vec<PathBuf>) {
    let result_str: Vec<String> = result
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    if result_str.len() == 1 {
        open_file_explorer(&result_str[0]);
        return;
    }

    let selected_file: Result<String, InquireError> =
        Select::new("Select a file 📁 to open", result_str).prompt();

    match selected_file {
        Ok(file) => {
            println!(
                "{}",
                format!("Opening {} just for you 💖", file.green().bold())
                    .magenta()
                    .italic()
                    .underline()
            );
            open_file_explorer(&file);
        }
        Err(_) => {
            println!(
                "{}",
                "You have exited the program 👋".bold().green().underline()
            );
        }
    };
}

fn open_file_explorer(filepath: &str) {
    let path = Path::new(filepath);

    if let Some(parent_path) = path.parent() {
        let os = env::consts::OS;
        let command = match os {
            "linux" => "xdg-open",
            "macos" => "open",
            "windows" => "explorer",
            _ => {
                println!("{}", "Your OS is not supported ⚠️".bold().red().underline());
                return;
            }
        };

        let _ = Command::new(command)
            .arg(parent_path)
            .spawn()
            .expect("Failed to open file");
    } else {
        println!("Sorry We weren't expecting that:(")
    }
}

/*
#[allow(dead_code)]
pub fn clear_terminal() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}
*/
