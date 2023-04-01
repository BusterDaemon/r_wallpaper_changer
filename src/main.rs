use rand::Rng;
use serde_yaml::{self};
use crate::{file_list::{get_file_list, get_rand_image}, metadata::{read_metadata, landscape, qual_control}};
use self::config::Config;
use wallpaper::{set_from_path, set_mode};
use clap::{Parser};

pub mod config;
pub mod file_list;
pub mod metadata;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();
    // Opens a config file
    let file_p = std::path::Path::new(&args.path).to_str().unwrap();

    //Reading the content from file
    let f = std::fs::File::open(file_p).expect("Must be a file.");
    let mut configs: Config = serde_yaml::from_reader(f).expect("Can't read values");

    //println!("{:?}", configs.conf);

    // Checking critical parameters
    if !configs.conf.global.useDirectory && !configs.conf.global.useUrls {
        println!("Directory mode and URL mode are disabled.");
        std::process::abort();
    }

    if configs.conf.local.landscapeCoef < 1.0 {
        println!("Too low landscape coefficient: {} < 1.0", configs.conf.local.landscapeCoef);
        configs.conf.local.landscapeCoef = 1.2;
        println!("Set to default value: {}", configs.conf.local.landscapeCoef);
    } else if configs.conf.local.landscapeCoef >= 3.0 {
        println!("Too high landscape coefficient: {} >= 3.0", configs.conf.local.landscapeCoef);
        configs.conf.local.landscapeCoef = 1.2;
        println!("Set to default value: {}", configs.conf.local.landscapeCoef);
    }

    if configs.conf.global.interval < 10 {
        println!("Too low time interval: {} < 10", configs.conf.global.interval);
        configs.conf.global.interval = 10;
        println!("Set to default value: {}", configs.conf.global.interval);
    }    

    loop {
        let mut rng = rand::thread_rng();
        let path = &configs.conf.local.directories[rng.gen_range(0..configs.conf.local.directories.len())];    
        let files = get_file_list(path.to_string());
        //println!("{:?}", files);
        
        let img = get_rand_image(files);
        let img_d = read_metadata(&img);
        let img_r = match img_d {
            Ok(res) => res,
            Err(_) => continue
        };
        if !configs.conf.local.usePortrait {
            if !landscape(configs.conf.local.landscapeCoef, &img_r) {
                continue;
            }
        }
        if configs.conf.local.setQualityControl {
            if !qual_control(configs.conf.local.minMps, configs.conf.local.maxMps, &img_r) {
                continue;
            }
        }        

        println!("Setting wallpaper from file: {}", &img);
        let wall = set_from_path(&img.as_str());
        let wall_r = match wall {
            Ok(res) => res ,
            Err(_) => continue
        };
        drop(wall_r);
        let mode = set_mode(wallpaper::Mode::Fit);
        let mode_r = match mode {
            Ok(res) => res,
            Err(err) => println!("Can't change wallmode: {:?}", err),
        };
        drop(mode_r);

        std::thread::sleep(std::time::Duration::from_secs(configs.conf.global.interval as u64));
    }
}
