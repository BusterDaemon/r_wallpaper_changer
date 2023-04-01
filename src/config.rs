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
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobCnf {
    pub interval: u32,
    pub useDirectory: bool,
    pub useUrls: bool,
    pub wallmode: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cnf {
    pub online: OnlineCnf,
    pub local: LocalCnf,
    pub global: GlobCnf
}
