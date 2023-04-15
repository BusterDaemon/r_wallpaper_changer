use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub conf: Cnf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineCnf {
    pub urls: Vec<String>,
    pub tls: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCnf {
    pub directories: Vec<String>,
    pub usePortrait: bool,
    pub landscapeCoef: f32,
    pub setQualityControl: bool,
    pub minMps: f32,
    pub maxMps: f32,
    pub enableFileBlacklist: bool,
    pub enableFolderBlacklist: bool,
    pub blacklist_files: Vec<String>,
    pub blacklist_folders: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobCnf {
    pub interval: u32,
    pub useDirectory: bool,
    pub useUrls: bool,
    pub wallmode: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cnf {
    pub online: OnlineCnf,
    pub local: LocalCnf,
    pub global: GlobCnf
}
