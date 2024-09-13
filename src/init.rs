use crate::Create;
use colored::Colorize;
use serde::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::{env, fs, io};
use toml::*;

impl Create {
    pub fn create_init() -> Result<(), Box<dyn Error>> {
        if env::current_dir()?.join("config.toml").exists() {
            fs::remove_file(env::current_dir()?.join("config.toml"))?;
        }
        if env::current_dir()?.join("install.sh").exists() {
            fs::remove_file(env::current_dir()?.join("install.sh"))?;
        }
        if env::current_dir()?.join("exec_name").exists() {
            fs::remove_file(env::current_dir()?.join("exec_name"))?;
        }
        if env::current_dir()?.join("log").exists() {
            fs::remove_dir_all(env::current_dir()?.join("log"))?;
        }
        let mut config_toml = fs::File::create("config.toml")?;
        let mut exename = File::create("exec_name")?;
        let mut installsh = File::create("install.sh")?;
        let exec_name = include_bytes!("../exec_name");
        let install_sh = include_bytes!("../install_sh");
        let base = include_bytes!("../base.toml");
        config_toml.write_all(base)?;
        exename.write_all(exec_name)?;
        installsh.write_all(install_sh)?;

        fs::create_dir("log")?;

        Ok(())
    }
    pub fn clean() -> Result<(), Box<dyn Error>> {
        println!("{} {}", ">>>".green().bold(), "Cleaning...".bold());
        if env::current_dir()?.join("config.toml").exists() {
            fs::remove_file(env::current_dir()?.join("config.toml"))?;
            println!("{} {}", ">>>".yellow().bold(), "Clean config.toml...");
        }
        if env::current_dir()?.join("install.sh").exists() {
            fs::remove_file(env::current_dir()?.join("install.sh"))?;
            println!("{} {}", ">>>".yellow().bold(), "Clean install.sh...");
        }
        if env::current_dir()?.join("exec_name").exists() {
            fs::remove_file(env::current_dir()?.join("exec_name"))?;
            println!("{} {}", ">>>".yellow().bold(), "Clean exec_name...");
        }
        if env::current_dir()?.join("log").exists() {
            fs::remove_dir_all(env::current_dir()?.join("log"))?;
            println!("{} {}", ">>>".yellow().bold(), "Clean log directory...");
        }
        let mut config_toml = fs::File::create("config.toml")?;
        let mut exename = File::create("exec_name")?;
        let mut installsh = File::create("install.sh")?;
        let exec_name = include_bytes!("../exec_name");
        let install_sh = include_bytes!("../install_sh");
        let base = include_bytes!("../base.toml");
        config_toml.write_all(base)?;
        exename.write_all(exec_name)?;
        installsh.write_all(install_sh)?;

        fs::create_dir("log")?;

        Ok(())
    }
}
