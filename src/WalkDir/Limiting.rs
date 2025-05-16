//limiting the number of files
use walkdir::WalkDir;

fn main() {
    let limit = 10;

    for (i, entry) in WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .enumerate()
    {
        if i >= limit {
            break;
        }
        println!("{}: {}", i + 1, entry.path().display());
    }
}
