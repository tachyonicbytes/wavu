use flate2::read::GzDecoder;

use indicatif::ProgressBar;

use std::fs::{create_dir_all, File};
use tar::Archive;

use super::common::{copy_directory_contents, download_file, Config};

static WAZERO_RELEASES: &str = "https://github.com/tetratelabs/wazero/releases";

pub fn download(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Downloading wazero";

    pb.set_message(message_prefix);

    let release: &str = WAZERO_RELEASES;
    let binary = "download/v1.3.0/wazero_1.3.0_linux_amd64.tar.gz";
    let url: String = format!("{release}/{binary}");
    let url = url.as_str();

    pb.set_message(format!("{message_prefix}: Creating the target directory"));
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wazero/");
    create_dir_all(download_dir).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    download_file(url, download_dir.join("wazero.tar.gz").to_str().unwrap());

    pb.finish_with_message("Downloaded wazero");
}

pub fn install(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Installing wazero";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wazero/");
    let install_dir = &config.home_dir.join(".wavu/bin/wazero/");

    pb.set_message(message_prefix);

    let path = download_dir.join("wazero.tar.gz");

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_xz = File::open(path).expect("Could not open the tarball");
    let tar = GzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    create_dir_all(download_dir).unwrap();
    archive
        .unpack(download_dir)
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );

    pb.finish_with_message("Installed wazero");
}
