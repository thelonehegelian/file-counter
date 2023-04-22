use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
fn main() {
    // read directory from command line
    let args: Vec<String> = std::env::args().collect();
    let current_dir = std::path::Path::new(&args[1]);
    println!("The current directory is {}", current_dir.display());
    print_result_to_file(current_dir);
}

fn get_num_pages(dir: &Path) -> HashMap<String, u32> {
    // hashmap of folder name to number of files in that folder
    let mut num_pages = HashMap::new();

    let paths = std::fs::read_dir(dir).unwrap();

    for path in paths {
        // count the number of files in each folder
        let path = path.unwrap().path();
        if !path.is_dir() {
            continue;
        } else {
            num_pages.insert(
                path.file_name().unwrap().to_str().unwrap().to_string(),
                path.read_dir().unwrap().count() as u32,
            );
        }
    }
    num_pages
}

fn sort_by_num_files(hash_map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut vec: Vec<_> = hash_map.into_iter().collect();
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    vec
}

fn print_result_to_file(dir: &Path) {
    let mut file = std::fs::File::create(dir.join("files.txt")).unwrap();
    for (folder, num) in sort_by_num_files(get_num_pages(dir)).iter() {
        writeln!(file, "{}: {}", folder, num).unwrap();
    }
}
