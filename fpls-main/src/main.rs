use std::io;
use std::fs;
use std::path::Path;

extern crate serde_json;
extern crate serde;

use env_logger::Env;
use fpls_lib::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub output_path: String,
}

fn log_init() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

fn run() -> Result<(), io::Error> {
    log_init();
    const FPLS_PATH: &str = "fpls.config.json";
    let content = fs::read_to_string(FPLS_PATH)?;
    let data = content.as_str();
    let v: Config = serde_json::from_str(data)?;

    let path = Path::new(v.path.as_str());
    let output_path = Path::new(v.output_path.as_str());
    if output_path.is_dir() {
        fs::remove_dir_all(output_path)?;
    }
    fs::create_dir(output_path)?;
    copy_file(&path, &output_path)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    run()?;
    Ok(())
}
