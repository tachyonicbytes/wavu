use indicatif::ProgressBar;

use std::fs::{create_dir_all, File};

use zip::read::ZipArchive;

use super::common::{copy_directory_contents, download_file, Config};

static SPIDERMONKEY_RELEASES: &str = "https://archive.mozilla.org/pub/firefox/releases";

pub fn download(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Downloading spidermonkey";
    pb.set_message(message_prefix);

    let release: &str = SPIDERMONKEY_RELEASES;
    let binary = "116.0/jsshell/jsshell-linux-x86_64.zip";
    let url: String = format!("{release}/{binary}");
    let url = url.as_str();

    pb.set_message(format!("{message_prefix}: Creating the target directory"));
    let download_dir = &config.cache_dir.join(".wavu/runtimes/spidermonkey/");
    create_dir_all(download_dir).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    download_file(url, download_dir.join("spidermonkey.zip").to_str().unwrap());

    pb.finish_with_message("Downloaded spidermonkey");
}

pub fn install(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Installing spidermonkey";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/spidermonkey/");
    let install_dir = &config.home_dir.join(".wavu/bin/spidermonkey/");

    pb.set_message(message_prefix);
    let path = download_dir.join("spidermonkey.zip");

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));

    let zip = File::open(path).expect("Could not open the tarball");
    let mut archive = ZipArchive::new(zip).unwrap();
    archive.extract(install_dir).unwrap();

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );

    pb.finish_with_message("Installed spidermonkey");
}
