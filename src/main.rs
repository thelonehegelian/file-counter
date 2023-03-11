use std::collections::HashMap;
fn main() {
    // read directory from command line
    let args: Vec<String> = std::env::args().collect();
    let current_dir = std::path::Path::new(&args[1]);
    println!("The current directory is {}", current_dir.display());

    // hashmap of folder name to number of files in that folder
    let mut num_pages = HashMap::new();

    // list all the folders in the current directory
    let paths = std::fs::read_dir(current_dir).unwrap();

    for path in paths {
        // count the number of files in each folder
        let path = path.unwrap().path();
        // if not a folder, skip
        if !path.is_dir() {
            continue;
        } else {
            num_pages.insert(
                path.file_name().unwrap().to_str().unwrap().to_string(),
                path.read_dir().unwrap().count() as u32,
            );
        }
    }
    println!("Number of pages: {:?}", num_pages);
}
