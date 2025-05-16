// FILTERING OUT THE FILES WITH THE NAME STARTING WITH "Life"
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path().is_file() &&
            e.path().file_name()
                .and_then(|name| name.to_str()) 
                .map_or(false, |name_str| name_str.starts_with("Life"))
        })
    {
        println!("{}", entry.path().display());
    }
}
