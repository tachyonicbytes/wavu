use clap::Parser;
use fs_extra::dir::{copy, CopyOptions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs::File, io::Write};

/// Cli struct
#[derive(Parser)]
pub struct Cli {
    /// Runtimes to install
    pub runtimes: Vec<String>,
}

/// Config struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Where `.wavu` should reside
    pub home_dir: PathBuf,
    /// Where wavu should keep its caches
    pub cache_dir: PathBuf,
}

// cp source/* target/
#[doc = "util function to copy directory contents to a target"]
pub fn copy_directory_contents(source: &str, target: &str) {
    let mut options: CopyOptions = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    copy(source, target, &options).unwrap();
}

// download_file("https://github.com/wasmerio/wasmer/releases/download/v4.0.0/wasmer-linux-amd64.tar.gz", "./runtimes/test/wasmer.tar.gz");
// wget url -O target, but it's the caller's job to create the target directory.
#[doc = "Download a file to a target"]
pub fn download_file(url: &str, target: &str) {
    println!("Downloading {}", url);

    let resp = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(target).unwrap();
    let bytes = resp.bytes().unwrap();
    file.write_all(&bytes).unwrap();

    println!("Downloaded {}", url);
}
