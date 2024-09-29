use clap::Parser;
use std::fs;
use std::path::Path;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

}

fn main() {
    let args = Args::parse();

    let current_directory = std::env::current_dir().expect("Failed to get current directory");
    let files = search_files(&current_directory, &args.file);

    if files.is_empty(){
        println!("No files with extension {} in current directory found", args.file);
    }
    else{
        for file in files{
            println!("{}", file.display());
        }
    }
}

/// Searches all files in curr directory with a given extension.
fn search_files(dir: &Path, extension: &str) -> Vec<std::path::PathBuf> {
    let extension = extension.trim_start_matches('.');
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_str().unwrap_or("") == extension {
                        result.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursively search subdirectories
                result.extend(search_files(&path, extension));
            }
        }
    }

    result
}