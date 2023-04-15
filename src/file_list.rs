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

// Read files in directory
#[allow(non_snake_case)]
pub fn get_file_list(path: &Path, config: &crate::config::Config) -> Result<Vec<PathBuf>, BlklsError> {
    if config.conf.local.enableFolderBlacklist {
        log::info!("Checking folder name \"{}\" for blacklist words", path.file_name().unwrap().to_str().unwrap().to_string());
        if !check_black_list(&path.file_name().unwrap().to_str().unwrap().to_string(), &config.conf.local.blacklist_folders) {
            return Err(BlklsError::new("Blacklist match."));
        }
    }

    log::info!("Getting file list from folder {}", &path.to_string_lossy());

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
        log::error!("No files! Exiting.");
        std::process::exit(1);
    }
    
    return Ok(files.to_vec());
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
                log::info!("\"{}\" contains \"{}\". Skipping...", name, word);
                return false;
            }
        } else {
            log::warn!("Blacklist word \"{}\" is too short for checking. Skipping...", word);
            continue;
        }
    }
    return true;
}