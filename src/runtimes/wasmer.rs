use flate2::read::GzDecoder;

use indicatif::ProgressBar;

use std::fs::{create_dir_all, File};
use tar::Archive;

use super::common::{copy_directory_contents, download_file, Config};

static WASMER_RELEASES: &str = "https://github.com/wasmerio/wasmer/releases";

// Runtimes
pub fn download(config: &Config, pb: &ProgressBar) {
    let release: &str = WASMER_RELEASES;
    let binary: &str = "download/v4.0.0/wasmer-linux-amd64.tar.gz";
    let url: String = format!("{release}/{binary}");
    let url = url.as_str();

    let message_prefix: &str = "Downloading wasmer";
    pb.set_message(message_prefix);

    pb.set_message(format!(
        "{message_prefix}{}",
        ": Creating the target directory"
    ));

    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmer/");
    create_dir_all(download_dir).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    download_file(url, download_dir.join("wasmer.tar.gz").to_str().unwrap());

    pb.finish_with_message("Downloaded wasmer");
}

pub fn install(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Installing wasmer";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmer");
    let install_dir = &config.home_dir.join(".wavu/bin/wasmer");

    pb.set_message(message_prefix);

    let path = download_dir.join("wasmer.tar.gz");

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_gz = File::open(path).expect("Could not open the tarball");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    create_dir_all(install_dir).expect("No error creating the directory");
    archive
        .unpack(install_dir)
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );

    pb.finish_with_message("Installed wasmer");
}
