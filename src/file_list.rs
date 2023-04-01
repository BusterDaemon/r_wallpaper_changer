use std::{path::{PathBuf, Path}, ffi::OsStr};

use rand::Rng;

// Read files in directory
pub fn get_file_list(path: String) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(&path).unwrap();    

    let mut files = vec![];
    
    for entry in entries {
        if !entry.as_ref().unwrap().path().is_dir() {
            let f_p: String = entry.as_ref().unwrap().path().to_owned().to_str().unwrap().to_string();
            let ext = Path::new(&f_p).extension().and_then(OsStr::to_str);
            if &ext.unwrap().to_string() == "jpg" || &ext.unwrap().to_string() == "png" {
                files.push(entry.unwrap().path());
            }            
        }
    }    

//    println!("{:?}", files);    
    return files;
}

// Choose random image from array
pub fn get_rand_image(list: Vec<PathBuf>) -> String {
    let mut rng = rand::thread_rng();
    let file: &String = &list[rng.gen_range(0..list.len())].to_str().unwrap().to_string();
    let ret: String = file.to_string();
    return ret;
}