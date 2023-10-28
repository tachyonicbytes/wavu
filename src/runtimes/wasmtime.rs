use indicatif::ProgressBar;

use std::fs::{create_dir_all, File};
use tar::Archive;
use xz2::read::XzDecoder;

use super::common::{copy_directory_contents, download_file, Config};

static WASMTIME_RELEASES: &str = "https://github.com/bytecodealliance/wasmtime/releases";

pub fn download(config: &Config, pb: &ProgressBar) {
    let message_prefix = "Downloading wasmtime";
    pb.set_message(message_prefix);

    let release: &str = WASMTIME_RELEASES;
    let binary = "download/v11.0.0/wasmtime-v11.0.0-x86_64-linux.tar.xz";
    let url: String = format!("{release}/{binary}");
    let url = url.as_str();

    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmtime/");
    create_dir_all(download_dir).expect("No error creating the directory");
    pb.set_message(format!("{message_prefix}/Creating the target directory"));

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    download_file(url, download_dir.join("wasmtime.tar.xz").to_str().unwrap());

    pb.finish_with_message("Downloaded wasmtime");
}

pub fn install(config: &Config, pb: &ProgressBar) {
    let message_prefix = "Installing wasmtime";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmtime");
    let install_dir = &config.home_dir.join(".wavu/bin/wasmtime");

    let path = download_dir.join("wasmtime.tar.xz");

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_xz = File::open(path).expect("Could not open the tarball");
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    create_dir_all(download_dir.join("wasmtime/")).unwrap();
    archive
        .unpack("./runtimes/wasmtime/")
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );

    pb.finish_with_message("Installed wasmtime");
}
