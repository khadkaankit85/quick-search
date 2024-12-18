use clap::Parser;
use colored::*;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;
    println!(
        "{}",
        format!(
            "Searching for {} in your device 🔍",
            filename.green().bold()
        )
        .cyan()
        .bold()
    );
    let results = utils::search_files(&filename);
    let file_count = results.len().to_string();

    println!(
        "{}",
        format!(
            "I found {} files containing '{}' in your device",
            file_count.green().bold(),
            filename.green().bold()
        )
        .cyan()
        .bold()
    );
    utils::display_and_select(results);
}
