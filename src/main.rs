use rand::Rng;
use serde_yaml::{self};
use crate::{file_list::{get_file_list, get_rand_image}, metadata::{read_metadata, landscape, qual_control}};
use self::config::Config;
use std::{env, mem};
use log;

pub mod config;
pub mod file_list;
pub mod metadata;

extern crate env_logger;

fn main() {
    // Parsing arguments
    let args: Vec<String> = env::args().collect();
    let mut i = 0;
    let mut cf_path: String = "".to_string();
    for arg in args.to_vec() {
        i += 1;
        if arg == "--help" {
            help();
            std::process::exit({
                match env::consts::OS {
                    "linux" => 0,
                    "windows" => 256,
                    _ => 0
                }
            });
        }
        if arg == "--config" || arg == "-c" {
            cf_path = args[i].to_string();
        }
    }

    if cf_path == "" {
        cf_path = "./config.yaml".to_string();
    }    
    
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();        

    // Opens a config file
    let file_p = std::path::Path::new(&cf_path).to_str().unwrap();

    //Reading the content from file
    let f = std::fs::File::open(file_p).expect("Must be a file.");
    let mut configs: Config = serde_yaml::from_reader(&f).expect("Can't read values");
    mem::drop(f);

    // Checking critical parameters

    if configs.conf.global.useDirectory && configs.conf.local.directories.len() < 1 {
        log::warn!("Directory mode enabled but list is empty. Disabling...");
        configs.conf.global.useDirectory = false;
    }

    if configs.conf.global.useUrls && configs.conf.online.urls.len() < 1 {
        log::warn!("URL mode enabled but list is empty. Disabling...");
        configs.conf.global.useUrls = false;
    }

    if !configs.conf.global.useDirectory && !configs.conf.global.useUrls {        
        log::error!("Directory mode and URL mode are disabled.");
        std::process::exit(1);
    }

    if configs.conf.local.enableFileBlacklist && configs.conf.local.blacklist_files.len() < 1 {
        log::warn!("File blacklist enabled but list is empty. Disabling...");
        configs.conf.local.enableFileBlacklist = false;
    }

    if configs.conf.local.enableFolderBlacklist && configs.conf.local.blacklist_folders.len() < 1 {
        log::warn!("Folder blacklist enabled but list is empty. Disabling...");
        configs.conf.local.enableFolderBlacklist = false;
    }

    if configs.conf.local.landscapeCoef < 1.0 {        
        log::warn!("Too low landscape coefficient: {} < 1.0", configs.conf.local.landscapeCoef);
        configs.conf.local.landscapeCoef = 1.2;        
        log::warn!("Set to default value: {}", configs.conf.local.landscapeCoef);
    } else if configs.conf.local.landscapeCoef >= 3.0 {        
        log::warn!("Too high landscape coefficient: {} >= 3.0", configs.conf.local.landscapeCoef);
        configs.conf.local.landscapeCoef = 1.2;        
        log::warn!("Set to default value: {}", configs.conf.local.landscapeCoef);
    }

    if configs.conf.global.interval < 10 {
        log::warn!("Too low time interval: {} < 10", configs.conf.global.interval);        
        configs.conf.global.interval = 10;        
        log::warn!("Set to default value: {}", configs.conf.global.interval);
    }

    // Starting loop
    loop {
        if !configs.conf.global.useDirectory && configs.conf.global.useUrls {
            setFromUrl(&configs);
        } else if configs.conf.global.useDirectory && !configs.conf.global.useUrls {
            setFromFile(&configs);
        } else if configs.conf.global.useDirectory && configs.conf.global.useUrls {

            let mut rng = rand::thread_rng();
            let chance = rng.gen_range(0..100);
            mem::drop(rng);

            if chance > 0 && chance < 51 {
                setFromFile(&configs);
            } else if chance > 50 && chance < 101 {
                setFromUrl(&configs);
            }
        }
    }
}

#[allow(non_snake_case)]
fn setFromUrl(configs: &crate::Config) -> bool {    
    log::info!("Using URL mode");

    let tmp_dir = tempfile::tempdir();
    let tm_er = tmp_dir.as_ref();
    match tm_er {
        Err(err) => {
            log::warn!("Can't create temp dir: {:?}", err);
            return false;
        }
        Ok(pass) => pass
    };

    let url: String;
    if configs.conf.online.urls.len() > 1 {
        log::info!("Choosing random URL");

        let mut rng = rand::thread_rng();
        url = configs.conf.online.urls[rng.gen_range(0..configs.conf.online.urls.len())].to_string();
    } else {
        url = configs.conf.online.urls[0].to_string();
    }

    log::info!("URL \"{}\" will be used", url);

    let req = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(!configs.conf.online.tls)
        .build()
        .unwrap()
        .get(url)
        .send();

    match req.as_ref() {
        Ok(res) => {
            if res.status() != 200 {
                log::warn!("Error. Status code {}", res.status())
            }
        },
        Err(err) => {
            log::warn!("Can't connect: {:?}", err);
            return false;
        }
    }    

    let img_t: &str = req.as_ref().unwrap().headers().get("content-type").unwrap().to_str().unwrap();

    log::info!("Creating temporary file.");

    let img_path = tmp_dir.as_ref().unwrap().path().join("img".to_string() + {
        match img_t {
                "image/png" => ".png",
                "image/jpeg" => ".jpg",
                _ => {
                    log::warn!("Requested file is not an image. Aborting.");
                    return false;
                }
        }
        });

    let file = std::fs::File::create(img_path.clone());
    let copy_res = std::io::copy(&mut req.unwrap(), &mut file.as_ref().unwrap());
    match copy_res {
        Ok(ok) => ok,
        Err(err) => {
            log::warn!("Error writing data to file: {:?}", err);
            return false;
        }
    };
    mem::drop(copy_res);
    mem::drop(file);

    log::info!("Setting wallpaper");

    let res = wallpaper::set_from_path(img_path.to_str().unwrap());
    match res {
        Ok(ok) => ok,
        Err(err) => {
            log::warn!("Can't change wallpaper: {:?}", err);
            return false;
        }
    };
    mem::drop(res);
    mem::drop(img_path);

    log::info!("Setting wallpaper mode");

    let mode = wallpaper::set_mode({
        match configs.conf.global.wallmode.as_str() {
            "Fit" => wallpaper::Mode::Fit,
            "Center" => wallpaper::Mode::Center,
            "Crop" => wallpaper::Mode::Crop,
            "Span" => wallpaper::Mode::Span,
            "Stretch" => wallpaper::Mode::Stretch,
            "Tile" => wallpaper::Mode::Tile,
            _ => {
                log::warn!("Can't set the mode. Fallback to Center.");
                wallpaper::Mode::Center
            }
        }
    });
    mem::drop(mode);

    std::thread::sleep(std::time::Duration::from_secs(configs.conf.global.interval.into()));

    let cls_res = tmp_dir.unwrap().close();
    match cls_res {
        Err(err) => {
            log::warn!("Can't close temp directory: {:?}", err);
            return false;
        }
        Ok(_) => return true
    };
}

#[allow(non_snake_case)]
fn setFromFile(configs: &crate::Config) -> bool {    
    let mut rng = rand::thread_rng();
    let path = std::path::Path::new(&configs.conf.local.directories[rng.gen_range(0..configs.conf.local.directories.len())]);
    let files = get_file_list(path, &configs);    

    match files {
        Err(err) => {
            log::warn!("Error: \"{:?}\". Skipping...", err);
            return false;
        },
        Ok(_) => (),
    };  

    let img = get_rand_image(&files.as_ref().unwrap());

    if configs.conf.local.setQualityControl || !configs.conf.local.usePortrait {
        let img_d = read_metadata(&img.to_str().unwrap().to_string());
        let img_r = match img_d {
            Ok(ref res) => res,
            Err(_) => return false
        };

        if !configs.conf.local.usePortrait {
            if !landscape(configs.conf.local.landscapeCoef, &img_r) {
                return false;
            }
        }

        if configs.conf.local.setQualityControl {
            if !qual_control(configs.conf.local.minMps, configs.conf.local.maxMps, &img_r) {
                return false;
            }
        }

        mem::drop(img_d);
    }

    log::info!("Setting wallpaper from file: {}", &img.to_str().unwrap().to_string());

    let wall = wallpaper::set_from_path(&img.to_str().unwrap());
    let _wall_r = match wall {
        Ok(res) => res,
        Err(err) => {
            log::warn!("Can't set wallpaper: {:?}", err);
            return false;
        }
    };

    let mode = wallpaper::set_mode({
        match configs.conf.global.wallmode.as_str() {
            "Fit" => wallpaper::Mode::Fit,
            "Center" => wallpaper::Mode::Center,
            "Crop" => wallpaper::Mode::Crop,
            "Span" => wallpaper::Mode::Span,
            "Stretch" => wallpaper::Mode::Stretch,
            "Tile" => wallpaper::Mode::Tile,
            _ => {
                log::warn!("Can't set the mode. Fallback to Center.");
                wallpaper::Mode::Center
            }
        }   
    });
    mem::drop(mode);

    std::thread::sleep(std::time::Duration::from_secs(configs.conf.global.interval.into()));
    return true;    
}

fn help() {
    print!("Arguments:
    --config, -c PATH - Path to config file
    --help - display help message
    \n");
}