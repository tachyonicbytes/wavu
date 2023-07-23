//! Feature matrix
//!
//! | target \ runtime | wasmer | wasmtime | wasm3 | wazero | timecraft | wasmedge |
//! |------------------+--------+----------+-------+--------+-----------+----------|
//! | linux amd64      | X      | X        | X     | X      |           |          |
//! | linux aarch64    |        |          |       |        |           |          |
//! | darwin amd64     |        |          |       |        |           |          |
//! | darwin aarch64   |        |          |       |        |           |          |
//! | windows amd64    |        |          |       |        |           |          |

use clap::Parser;
use flate2::read::GzDecoder;
use fs_extra::dir::{copy, CopyOptions};
use indicatif::MultiProgress;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::env;
use std::thread;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    time::Duration,
};
use tar::Archive;
use xz2::read::XzDecoder;

// Pure WASM runtimes
static WASMER_RELEASES: &str = "https://github.com/wasmerio/wasmer/releases";
static WASM3_RELEASES: &str = "https://github.com/wasm3/wasm3/releases";
static WASMTIME_RELEASES: &str = "https://github.com/bytecodealliance/wasmtime/releases";
static WAZERO_RELEASES: &str = "https://github.com/tetratelabs/wazero/releases/";
static TIMECRAFT_RELEASES: &str = "";
static WASMEDGE_RELEASES: &str = "";

// TODO: Search more
// TODO: Check AWSM
static WAMR_RELEASES: &str = "";
static LIBWASM_RELEASES: &str = "";

// JavaScript runtimes containing WASM runtimes
// TODO: best to use jsvu here and just wrap that up.
static SPIDERMONKEY_RELEASES: &str = "";
static V8_RELEASES: &str = "";
static JAVASCRIPTCORE_RELEASES: &str = "";

// TODO: Run a module in multiple runtimes easily, and compare.

/// Cli struct
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    arch: String, // The architecture of your system
    /// The path to the file to read
    runtime: String, // Runtime to install
                     // let mut array: Vec<String>;  // TODO: In the future, we want to pass a list of runtimes
}

/// Config struct
struct Config {
    /// Where .wavu should reside
    home_dir: String,
}

// cp source/* target/
#[doc = "util function to copy directory contents to a target"]
fn copy_directory_contents(source: &str, target: &str) {
    let mut options: CopyOptions = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    copy(source, target, &options).unwrap();
}

// download_file("https://github.com/wasmerio/wasmer/releases/download/v4.0.0/wasmer-linux-amd64.tar.gz", "./runtimes/test/wasmer.tar.gz");
// wget url -O target
#[doc = "download a file to a target"]
fn download_file(url: &str, target: &str) {
    info!("Creating the target directory");
    // create_dir_all("./runtimes/wasmer/");

    info!("Starting the download");
    let mut resp = reqwest::blocking::get(url).expect(&format!("Could not download: {}", url));
    let mut file = File::create(target).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes);
}

// Runtimes
fn download_wasmer(pb: &ProgressBar) {
    let message_prefix: &str = "Downloading wasmer";

    pb.set_message(message_prefix);

    let url: &str = WASMER_RELEASES;
    let binary = "download/v4.0.0/wasmer-linux-amd64.tar.gz";

    pb.set_message(format!(
        "{message_prefix}{}",
        ": Creating the target directory"
    ));
    create_dir_all("./runtimes/wasmer/").unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));

    let resp =
        reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasmer");
    let mut file = File::create("./runtimes/wasmer/wasmer.tar.gz").expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");

    pb.set_message(format!("{message_prefix}: writing to disk"));
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wasmer");
}

fn install_wasmer(pb: &ProgressBar) {
    let message_prefix: &str = "Installing wasmer";

    pb.set_message(message_prefix);

    let home: String = env::var("HOME").expect("Could not get the $HOME directory");
    let path = "./runtimes/wasmer/wasmer.tar.gz";

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_gz = File::open(path).expect("Could not open the tarball");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    create_dir_all("./runtimes/wasmtime/");
    archive
        .unpack("./runtimes/wasmtime/")
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents("./runtimes/wasmtime/bin", &format!("{home}/.wavu/bin/"));

    pb.finish_with_message(format!("Installed wasmer"));
}

fn download_wasmtime(pb: &ProgressBar) {
    let message_prefix = "Downloading wasmtime";
    pb.set_message(message_prefix);

    let url: &str = WASMTIME_RELEASES;
    let binary = "download/v11.0.0/wasmtime-v11.0.0-x86_64-linux.tar.xz";

    info!("Creating the target directory");
    create_dir_all("./runtimes/wasmtime/");

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    let mut resp =
        reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasmtime");
    let mut file =
        File::create("./runtimes/wasmtime/wasmtime.tar.xz").expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes);

    pb.finish_with_message(format!("Downloaded wasmtime"));
}

fn install_wasmtime(pb: &ProgressBar) {
    let message_prefix = "Installing wasmtime";

    let home: String = env::var("HOME").expect("Could not get the $HOME directory");
    let path = "./runtimes/wasmtime/wasmtime.tar.xz";

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_xz = File::open(path).expect("Could not open the tarball");
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    create_dir_all("./runtimes/wasmtime/").unwrap();
    archive
        .unpack("./runtimes/wasmtime/")
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents("./runtimes/wasmtime/", &format!("{home}/.wavu/bin/"));

    pb.finish_with_message("Installed wasmtime");
}

fn download_wasm3(pb: &ProgressBar) {
    pb.set_message("Downloading wasm3");

    let url: &str = WASM3_RELEASES;
    let binary = "download/v0.5.0/wasm3-linux-x64.elf";

    info!("Creating the target directory");
    create_dir_all("./runtimes/wasm3/bin/").unwrap();

    info!("Starting the download");
    info!("The actual url: {}", format!("{url}/{binary}"));
    let resp = reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasm3");
    let mut file = File::create("./runtimes/wasm3/bin/wasm3").expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wasm3");
}

fn install_wasm3(pb: &ProgressBar) {
    pb.finish_with_message("Installing wasm3: wasm3 does not require installation");
}

fn download_wazero(pb: &ProgressBar) {
    let message_prefix: &str = "Downloading wazero";
    pb.set_message(message_prefix);

    let url: &str = WAZERO_RELEASES;
    let binary = "download/v1.3.0/wazero_1.3.0_linux_amd64.tar.gz";

    info!("Creating the target directory");
    create_dir_all("./runtimes/wazero/").unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    let resp = reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasm3");
    let mut file = File::create("./runtimes/wazero/wazero.tar.gz").expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wazero");
}

fn install_wazero(pb: &ProgressBar) {
    let message_prefix: &str = "Installing wazero";
    pb.set_message(message_prefix);

    let home: String = env::var("HOME").expect("Could not get the $HOME directory");
    let path = "./runtimes/wazero/wazero.tar.gz";

    pb.set_message(format!("{message_prefix}: Unzipping the archive"));
    let tar_xz = File::open(path).expect("Could not open the tarball");
    let tar = GzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    create_dir_all("./runtimes/wazero/").unwrap();
    archive
        .unpack("./runtimes/wazero/")
        .expect("Could not unpack tarball");

    pb.set_message(format!("{message_prefix}: Copying the contents"));
    copy_directory_contents("./runtimes/wazero/", &format!("{home}/.wavu/bin/"));

    pb.finish_with_message(format!("Installed wazero"));
}

fn download_all() {
    let mp = MultiProgress::new();
    let pstyle = ProgressStyle::default_spinner()
        .template("{spinner:.green} {wide_msg}")
        .expect("ProgresStyle::template direct input to be correct");

    mp.println("starting!").unwrap();

    thread::scope(|scope| {
        scope.spawn(|| {
            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            download_wasm3(&pb);

            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            install_wasm3(&pb)
        });
        scope.spawn(|| {
            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            download_wasmer(&pb);

            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            install_wasmer(&pb);
        });
        scope.spawn(|| {
            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            download_wasmtime(&pb);

            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            install_wasmtime(&pb);
        });
        scope.spawn(|| {
            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            download_wazero(&pb);

            let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
            pb.set_style(pstyle.clone());
            pb.enable_steady_tick(Duration::from_millis(80));
            install_wazero(&pb);
        });
    });
}

fn main() {
    download_all();
    return;

    let args = Cli::parse();
    info!("{}, {}", args.arch, args.runtime);

    let config = Config {
        home_dir: env::var("HOME").expect("Home directory not accessible"),
    };

    // mkdir -p ~/.wavu/bin/
    create_dir_all(format!("{}/.wavu/bin/", config.home_dir));

    match args.arch.as_ref() {
        "linux64" => println!("TODO: Good, but implement this arch"),
        "darwin64" => println!("TODO: Good, but implement macos (darwin)"),
        "windows64" => println!("TODO: Good, but implement windows 64"),
        _ => panic!("Not actually implemented"),
    }

    match args.runtime.as_ref() {
        "wasmer" => {
            info!("TODO: Good, but implement this runtime");
            // download_wasmer();
            // install_wasmer();
        }
        "wasmtime" => {
            info!("TODO: Good, but implement this runtime");
            // download_wasmtime();
            // install_wasmtime();
        }
        "wasm3" => {
            info!("TODO: Good, but implement wasm3");
            // download_wasm3();
            // install_wasm3();
        }
        "wazero" => {
            info!("TODO: Good, but implement wazero");
            // download_wazero();
            // install_wazero();
        }
        _ => panic!("Unknown runtime"),
    }
}
