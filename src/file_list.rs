use std::{path::{PathBuf}};
use walkdir::WalkDir;

use rand::Rng;

// Read files in directory
pub fn get_file_list(path: String) -> Vec<PathBuf> {
    let walker = WalkDir::new(&path)
        .follow_links(false)
        .max_depth(6)
        .contents_first(true);
    let files = &mut vec![];
    for entry in walker {
        let path = entry.as_ref().unwrap().path();
        if !path.is_dir() {
            let ext = &path.extension().unwrap().to_os_string();
            if ext == "jpg" || ext == "png" {
                files.push(entry.unwrap().into_path());
            }
        }
    }
    if files.len() < 1 {
        println!("Too few files");
        std::process::abort();
    }
    return files.to_vec();        
}

// Choose random image from array
pub fn get_rand_image(list: Vec<PathBuf>) -> String {
    let mut ret: String = "".to_string();
    if list.len() > 1 {
        let mut rng = rand::thread_rng();
        let file: &String = &list[rng.gen_range(0..list.len())].to_str().unwrap().to_string();
        ret = file.to_string();
    } else if list.len() == 1 {
        ret = list[0].to_str().unwrap().to_string();    
    }
    ret
}