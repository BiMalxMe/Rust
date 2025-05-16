
//show only extensions with .rs
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |d| d=="rs"))
    {
        println!("Rust File: {}", entry.path().display());
    }
}