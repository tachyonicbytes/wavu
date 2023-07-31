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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
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
    /// Runtimes to install
    runtimes: Vec<String>,
}

/// Config struct
#[derive(Serialize, Deserialize)]
struct Config {
    /// Where `.wavu` should reside
    home_dir: PathBuf,
    /// Where wavu should keep its caches
    cache_dir: PathBuf,
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
// wget url -O target, but it's the caller's job to create the target directory.
#[doc = "download a file to a target"]
fn download_file(url: &str, target: &str) {
    println!("Downloading {}", url);

    let resp = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(target).unwrap();
    let bytes = resp.bytes().unwrap();
    file.write_all(&bytes).unwrap();

    println!("Downloaded {}", url);
}

// Runtimes
fn download_wasmer(config: &Config, pb: &ProgressBar) {
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

fn install_wasmer(config: &Config, pb: &ProgressBar) {
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

fn get_versions_wasmer() {}
fn sort_versions_wasmer() {}
fn get_latest_wasmer() {}
fn run_wasmer() {}
fn benchmark_wasmer() {} // Should call out to hyperfine?
fn run_wasm_testsuite_wasmer() {}
fn run_wasi_testsuite_wasmer() {}

fn download_wasmtime(config: &Config, pb: &ProgressBar) {
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

fn install_wasmtime(config: &Config, pb: &ProgressBar) {
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

fn download_wasm3(config: &Config, pb: &ProgressBar) {
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

fn install_wasm3(config: &Config, pb: &ProgressBar) {
    pb.set_message("Installing wasm3");

    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasm3/");
    let install_dir = &config.home_dir.join(".wavu/bin/wasm3/");

    copy_directory_contents(
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );
    pb.finish_with_message("Installed wasm3");
}

fn download_wazero(config: &Config, pb: &ProgressBar) {
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

fn install_wazero(config: &Config, pb: &ProgressBar) {
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

fn install_all(
    config: &Config,
    runtimes: HashMap<&str, (fn(&Config, &ProgressBar), fn(&Config, &ProgressBar))>,
) {
    let mp = MultiProgress::new();
    let pstyle = ProgressStyle::default_spinner()
        .template("{spinner:.green} {wide_msg}")
        .expect("ProgresStyle::template direct input to be correct");

    mp.println(format!("Starting downloading {} runtimes!", runtimes.len()))
        .unwrap();

    thread::scope(|scope| {
        scope.spawn(|| {
            for (_, (download, install)) in runtimes.iter() {
                let pb = mp.insert_from_back(0, ProgressBar::new_spinner());
                pb.set_style(pstyle.clone());
                pb.enable_steady_tick(Duration::from_millis(80));

                download(config, &pb);
                install(config, &pb);
            }
        });
    });

    mp.println("Finished installing all runtimes!").unwrap();
}

fn main() {
    let args = Cli::parse();

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    println!("{:?}", args.runtimes);

    // The default config
    let mut config = Config {
        home_dir: dirs::home_dir().expect("Home dir to be present"),
        cache_dir: dirs::cache_dir().expect("Cache dir to be present"),
    };

    if config.home_dir.join(".wavu/wavu.conf.json").is_file() {
        let json_string = fs::read_to_string(config.home_dir.join(".wavu/wavu.conf.json"))
            .expect("Failed to read the config.");
        config = serde_json::from_str(&json_string).expect("Failed to parse the config");
    }
    // The config should be loaded correctly after this point

    // mkdir -p $HOME_DIR/.wavu/bin/
    create_dir_all(config.home_dir.join(".wavu/bin/")).expect("Directory to be creted");

    // mkdir -p $CACHE_DIR/.wavu/runtimes/
    create_dir_all(config.cache_dir.join(".wavu/runtimes/")).expect("Directory to be creted");

    match os {
        "linux" => println!("OS is linux"),
        "macos" => panic!("TODO: macos aarch64"),
        "windows" => panic!("TODO: windows64"),
        _ => panic!("Not actually implemented"),
    }

    match arch {
        "x86_64" => println!("Arch is x86_64"),
        _ => panic!("Not actually implemented"),
    }

    let mut runtimes: HashMap<&str, (fn(&Config, &ProgressBar), fn(&Config, &ProgressBar))> =
        HashMap::new();

    for runtime in args.runtimes.iter() {
        match runtime.as_ref() {
            "wasm3" => {
                runtimes.insert("wasm3", (download_wasm3, install_wasm3));
            }
            "wasmer" => {
                runtimes.insert("wasmer", (download_wasmer, install_wasmer));
            }
            "wasmtime" => {
                runtimes.insert("wasmtime", (download_wasmtime, install_wasmtime));
            }
            "wazero" => {
                runtimes.insert("wazero", (download_wazero, install_wazero));
            }
            runtime => {
                panic!("Unknown runtime '{}'", runtime);
            }
        }
    }

    install_all(&config, runtimes);
}
