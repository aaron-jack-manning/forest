use std::env;
use std::fs;
use std::path;

fn generate_tree(root : &path::PathBuf, prefix : &String, is_last_item : bool) {

    let name =
        root
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
        
    if is_last_item {
        println!("{}└──{}", prefix, name);
    }
    else {
        println!("{}├──{}", prefix, name);
    }

    if root.is_dir() {
        let mut paths : Vec<_> =
            fs::read_dir(root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        
        paths
        .sort_by(|a, b| { a.partial_cmp(b).unwrap()});

        for i in 0..paths.len() {
            let new_is_last_item = i == paths.len() - 1;

            let new_prefix = if is_last_item {
                format!("{}   ", prefix)
            }
            else {
                format!("{}│  ", prefix)
            };
        
            generate_tree(&paths[i], &new_prefix, new_is_last_item);
        }
    }
}


fn main() {

    match env::current_dir() {
        Ok(path) => {
            generate_tree(&path, &format!(""), true);
        },
        Err(_) => {
            println!("Failed to read the current directory.");
        }

    }
}
