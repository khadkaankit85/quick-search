use clap::Parser;
mod utils;
#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;

    println!("Searching for {filename} in your device");

    let results = utils::search_files(&filename);

    println!("Found {} files containing '{}'", results.len(), &filename);

    utils::display_and_select(results);
}
