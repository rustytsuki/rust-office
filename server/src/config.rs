use tini::Ini;
use lazy_static::lazy_static;
use std::sync::RwLock;
use crate::args::Args;

lazy_static!{
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::new());
}
pub struct Default {
    pub proxy: bool,
}

pub struct Postgres {
    pub ip: String,
    pub port: String,
    pub user: String,
    pub password: String,
}
pub struct Config {
    pub default: Default,
    pub postgres: Postgres,
}

impl Config {
    pub fn new() -> Self {
        Self {
            default: Default { proxy: false },
            postgres: Postgres {
                ip: String::from(""),
                port: String::from(""),
                user: String::from(""),
                password: String::from("")
            }
        }
    }

    pub fn from_ini(&mut self, ini_file: &str) -> Result<Ini, tini::Error> {
        let ini = Ini::from_file(ini_file)?;
        if let Some(value) = ini.get("default", "proxy") {
            self.default.proxy = value;
        }
        Ok(ini)
    }

    pub fn to_ini(&self, ini_file: &str) {
        let conf = Ini::new().section("default").item("debug", self.default.proxy);
        if let Err(why) = conf.to_file(ini_file) {
            println!("write {} failure: {}", ini_file, why);
        }
    }

    pub fn merge_args(&mut self, args: &Args) {
        self.default.proxy = args.proxy;
    }
}

pub fn load(ini_file: &str, args: &Args) -> Result<Ini, tini::Error> {
    let mut config = CONFIG.write().unwrap();
    let ret = config.from_ini(ini_file);
    config.merge_args(args);
    ret
}

pub fn inst() -> &'static RwLock<Config> {
    return &*CONFIG;
}
