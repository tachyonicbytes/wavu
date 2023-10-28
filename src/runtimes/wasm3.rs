use super::common::{copy_directory_contents, download_file, Config};

use indicatif::ProgressBar;

use std::fs::create_dir_all;

static WASM3_RELEASES: &str = "https://github.com/wasm3/wasm3/releases";

pub fn download(config: &Config, pb: &ProgressBar) {
    let message_prefix = "Downloading wasm3";
    pb.set_message(message_prefix);

    let release: &str = WASM3_RELEASES;
    let binary = "download/v0.5.0/wasm3-linux-x64.elf";
    let url: String = format!("{release}/{binary}");
    let url = url.as_str();

    pb.set_message(format!("{message_prefix}: Creating the target directory"));
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasm3/");
    create_dir_all(download_dir.join("bin")).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    download_file(url, download_dir.join("wasm3").to_str().unwrap());

    pb.finish_with_message("Downloaded wasm3");
}

pub fn install(config: &Config, pb: &ProgressBar) {
    pb.set_message("Installing wasm3");

    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasm3/");
    let install_dir = &config.home_dir.join(".wavu/bin/wasm3/");

    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );
    pb.finish_with_message("Installed wasm3");
}
