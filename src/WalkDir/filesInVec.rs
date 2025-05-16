// /  Making a vector consisting the files of a directory

use walkdir::WalkDir;
fn main() {
    let files: Vec<_> = WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    for file in files {
        println!("{}", file.path().display());
    }
}
