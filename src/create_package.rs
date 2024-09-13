use crate::{Config, Create};
use colored::Colorize;
use serde::*;
use std::io::{Read, Write};
use std::path::Path;
use std::{
    error::Error,
    fs::{self, File},
    io,
};
use toml::*;
use zip::write::FileOptions;

impl Create {
    pub fn create_for_linux() -> Result<(), Box<dyn Error>> {
        let cfgtml = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&cfgtml)?;

        let fomt = format!("{}-x86_64-unknown-linux-gnu.radepkg", config.package_name);
        let zip_file = File::create(fomt)?;
        let mut zip = zip::ZipWriter::new(zip_file);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated) // 圧縮方式
            .unix_permissions(0o755);

        let file_names = &[
            config.exe_file.as_str(),
            config.exec_name.as_str(),
            config.install_sh.as_str(),
        ];

        for file_name in file_names {
            println!(
                "{} {}{}{}",
                ">>>".cyan().bold(),
                "Compressing ".bold(),
                file_name.bold(),
                "...".bold()
            );
            let path = Path::new(file_name);

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
            zip.write_all(&buffer)?;
        }
        zip.finish()?;

        println!(
            "{} {}",
            ">>>".yellow().bold(),
            "The package has been successfully completed!".bold()
        );

        Ok(())
    }
    pub fn create_for_windows() -> Result<(), Box<dyn Error>> {
        let cfgtml = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&cfgtml)?;

        let fomt = format!("{}-x86_64-pc-windows-gnu.radepkg", config.package_name);
        let zip_file = File::create(fomt)?;
        let mut zip = zip::ZipWriter::new(zip_file);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated) // 圧縮方式
            .unix_permissions(0o755);

        let file_names = &[
            config.exe_file.as_str(),
            config.exec_name.as_str(),
            config.install_sh.as_str(),
        ];

        for file_name in file_names {
            println!(
                "{} {}{}{}",
                ">>>".cyan().bold(),
                "Compressing ".bold(),
                file_name.bold(),
                "...".bold()
            );
            let path = Path::new(file_name);

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
            zip.write_all(&buffer)?;
        }
        zip.finish()?;

        println!(
            "{} {}",
            ">>>".yellow().bold(),
            "The package has been successfully completed!".bold()
        );

        Ok(())
    }
    pub fn create_for_macos() -> Result<(), Box<dyn Error>> {
        let cfgtml = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&cfgtml)?;

        let fomt = format!("{}-aarch64-apple-darwin.radepkg", config.package_name);
        let zip_file = File::create(fomt)?;
        let mut zip = zip::ZipWriter::new(zip_file);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated) // 圧縮方式
            .unix_permissions(0o755);

        let file_names = &[
            config.exe_file.as_str(),
            config.exec_name.as_str(),
            config.install_sh.as_str(),
        ];

        for file_name in file_names {
            println!(
                "{} {}{}{}",
                ">>>".cyan().bold(),
                "Compressing ".bold(),
                file_name.bold(),
                "...".bold()
            );
            let path = Path::new(file_name);

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
            zip.write_all(&buffer)?;
        }
        zip.finish()?;

        println!(
            "{} {}",
            ">>>".yellow().bold(),
            "The package has been successfully completed!".bold()
        );

        Ok(())
    }
}
