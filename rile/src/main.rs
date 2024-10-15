use clap::Parser;
use std::fs;
use std::path::Path;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// --file to search for a specific file
    /// --quote for querying using natural language.
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    file: Vec<String>,
    quote: String,
}

fn main() {
    let args = Args::parse();

    let current_directory = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {}", current_directory.display());
    
    let files = search_files(&current_directory, &args.file);

    if files.is_empty() {
        println!("No files with extensions {:?} found", args.file);
    } else {
        println!("Files with extensions {:?}:", args.file);
        for file in files {
            println!("{}", file.display());
        }
    }
}

fn search_files(dir: &Path, extensions: &[String]) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if extensions.iter().any(|e| ext.to_str().unwrap_or("") == e.trim_start_matches('.')) {
                        result.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursively search subdirectories
                result.extend(search_files(&path, extensions));
            }
        }
    }

    result
}
