use walkdir::WalkDir;

fn main() {
    let mut entries: Vec<_> = WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    // Sort alphabetically by file name
    entries.sort_by_key(|e| e.file_name().to_owned());

    for entry in entries {
        println!("{}", entry.path().display());
    }
}
