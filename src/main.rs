#![allow(warnings)]
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use serde::Deserialize;
use std::{env, fs};
use toml::de::Error;
use toml::de::*;
mod create_package;
mod init;

pub struct Create;

#[derive(Deserialize)]
pub struct Config {
    package_name: String,
    exec_name: String,
    install_sh: String,
    exe_file: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum Targets {
    Macos,
    Windows,
    Linux,
}

#[derive(Subcommand)]
enum Commands {
    /// create a package
    Build {
        #[arg(long, value_enum)]
        target: Option<Targets>, // --target オプションを定義
    },
    /// Create files necessary for package creation such as config.toml in this directory
    Init,
    /// Return config.toml etc. to the original version
    Clean,
}

#[derive(Parser)]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands, // サブコマンドとして "build" を受け取る
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("{} {}", ">>>".green().bold(), "Create init...".bold());
            Create::create_init().unwrap();

            println!(
                "{} {} {}",
                ">>>".green().bold(),
                "Created init at".bold(),
                env::current_dir().unwrap().display()
            );
        }
        Commands::Build { target } => {
            if let Some(target) = target {
                match target {
                    Targets::Macos => {
                        println!("{} {}", ">>>".green().bold(), "Building for macOS".bold());
                        if let Err(e) = Create::create_for_macos() {
                            eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                            );
                            eprint!("{} {}", "Error:", e);
                        }
                    }
                    Targets::Windows => {
                        println!("{} {}", ">>>".green().bold(), "Building for Windows".bold());
                        if let Err(e) = Create::create_for_windows() {
                            eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                            );
                            eprint!("{} {}", "Error:", e);
                        }
                    }
                    Targets::Linux => {
                        println!("Building for Linux");
                        if let Err(e) = Create::create_for_linux() {
                            eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                            );
                            eprint!("{} {}", "Error:", e);
                        }
                    }
                }
            } else {
                if cfg!(target_os = "macos") {
                    if let Err(e) = Create::create_for_macos(){
                        eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                        );
                        eprint!("{} {}", "Error:", e);
                        return;
                    }
                } else if cfg!(target_os = "linux") {
                    if let Err(e) = Create::create_for_linux(){
                        eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                        );
                        eprint!("{} {}", "Error:", e);
                        return;
                    }
                } else if cfg!(target_os = "windows") {
                    if let Err(e) = Create::create_for_windows(){
                        eprintln!(
                                "{} {}",
                                ">>>".red().bold(),
                                "Failed to create package".bold()
                        );
                        eprint!("{} {}", "Error:", e);
                        return;
                    }
                } else {
                    panic!("not support os");
                }
                println!("Building for default target");
            }
        }
        Commands::Clean => {
            Create::clean().unwrap();
        }
    }
}
