use clap::Parser;
use std::fs;
use std::path::Path;
use std::io::Read;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File extensions to search for (e.g., pdf txt rs)
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    file: Option<Vec<String>>,

    /// Quote to search for in files (optional)
    #[arg(short, long)]
    quote: Option<String>,
}

fn main() {
    let args = Args::parse();

    let current_directory = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {}", current_directory.display());
    
    let files = if let Some(quote) = &args.quote {
        println!("Searching for files containing: \"{}\"", quote);
        search_quote(&current_directory, args.file.as_deref(), quote)
    } else if let Some(extensions) = &args.file {
        search_files(&current_directory, extensions)
    } else {
        println!("Please specify either --file or --quote");
        return;
    };

    if files.is_empty() {
        if args.quote.is_some() {
            println!("No files containing the specified quote found");
        } else if let Some(extensions) = &args.file {
            println!("No files with extensions {:?} found", extensions);
        }
    } else {
        if args.quote.is_some() {
            println!("Files containing the specified quote:");
        } else if let Some(extensions) = &args.file {
            println!("Files with extensions {:?}:", extensions);
        }
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

fn search_quote(dir: &Path, extensions: Option<&[String]>, quote: &str) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(extensions) = extensions {
                    if let Some(ext) = path.extension() {
                        if !extensions.iter().any(|e| ext.to_str().unwrap_or("") == e.trim_start_matches('.')) {
                            continue;
                        }
                    }
                }
                if file_contains_quote(&path, quote) {
                    result.push(path);
                }
            } else if path.is_dir() {
                result.extend(search_quote(&path, extensions, quote));
            }
        }
    }

    result
}

fn file_contains_quote(path: &Path, quote: &str) -> bool {
    if let Ok(mut file) = fs::File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return contents.contains(quote);
        }
    }
    false
}
