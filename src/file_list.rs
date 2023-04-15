use std::path::{PathBuf, Path};
use walkdir::WalkDir;

use rand::Rng;

#[derive(Debug)]
pub struct BlklsError {
    description: String
}

impl BlklsError {
    fn new(msg: &str) -> BlklsError {
        BlklsError { description: msg.to_string() }
    }
}

impl std::fmt::Display for BlklsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for BlklsError {
    fn description(&self) -> &str {
        &self.description
    }
}

// Get files from directories
#[allow(non_snake_case)]
pub fn get_file_list(path: &Path, config: &crate::config::Config) -> Result<Vec<PathBuf>, BlklsError> {
    let files: &mut Vec<PathBuf> = &mut vec![];
    let folders: &mut Vec<PathBuf> = &mut vec![];
    

    for entry in WalkDir::new(&path).follow_links(false).max_depth(3) {
        if entry.as_ref().unwrap().path().is_dir() {
            if config.conf.local.enableFolderBlacklist {
                if !check_black_list(&entry.as_ref().unwrap().path().to_str().unwrap().to_string(), &config.conf.local.blacklist_folders) {
                    continue;                    
                }
            }
            folders.push(entry.unwrap().into_path());     
        }        
    }

    let searcher = |root_f: &Path, file_list: &mut Vec<PathBuf>| -> Result<Vec<PathBuf>, BlklsError> {
        log::info!("Getting file list from \"{}\"", root_f.to_str().unwrap().to_string());
        for entry in WalkDir::new(root_f).contents_first(true).follow_links(false) {
            if !entry.as_ref().unwrap().path().is_dir() {
                if entry.as_ref().unwrap().path().extension().unwrap().to_os_string() == "jpg" || entry.as_ref().unwrap().path().extension().unwrap().to_os_string() == "png" {
                    file_list.push(entry.unwrap().into_path());
                }
            }
        }

        if file_list.len() < 1 {
            return Err(BlklsError::new("File list is empty"));
        }

        return Ok(file_list.to_vec());
    };

    if folders.len() < 1 {
        let files_res = searcher(&path, files);

        match files_res {
            Ok(ok) => *files = ok,
            Err(err) => return Err(err)
        }

        return Ok(files.to_vec());   
    } else {
        log::info!("Getting the random folder from the list.");
        let mut rng = rand::thread_rng();
        let folder = &folders[rng.gen_range(0..folders.len())];                    
        
        let file_res = searcher(&folder.as_path(), files);
        match file_res {
            Ok(res) => *files = res,
            Err(err) => return Err(err)
        }
        return Ok(files.to_vec());
    }    
}

// Choose random image from array
pub fn get_rand_image(list: &Vec<PathBuf>) -> &std::path::Path {
    let mut ret: &std::path::Path = &std::path::Path::new("");
    if list.len() > 1 {
        let mut rng = rand::thread_rng();
        let file: &std::path::Path = &list[rng.gen_range(0..list.len())];
        ret = file;
    } else if list.len() == 1 {
        ret = &list[0].as_path();
    }
    ret
}

pub fn check_black_list(name: &String, black_list: &Vec<String>) -> bool {
    for word in black_list {
        if word.len() > 2 {
            if name.contains(word) {
                log::trace!("\"{}\" contains \"{}\". Skipping...", name, word);
                return false;
            }
        } else {
            log::warn!("Blacklist word \"{}\" is too short for checking. Skipping...", word);
            continue;
        }
    }
    return true;
}