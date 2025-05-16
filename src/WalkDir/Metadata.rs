
// Give the metadata of the file

use walkdir::WalkDir;
use std::fs;

fn main() {
    for entry in WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e: &walkdir::DirEntry| e.file_type().is_file())
    {
        let metadata = fs::metadata(entry.path()).unwrap();
        //metadata.len() is for the files size
        println!("{} - {:?} bytes", entry.path().display(), metadata.len());
    }
}