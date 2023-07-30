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
use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use std::thread;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    time::Duration,
};
use tar::Archive;
use xz2::read::XzDecoder;
use serde::{Serialize, Deserialize};

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

/// The operation systems enum
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OperatingSystem {
    Linux,
    Darwin,
    Windows,
}

impl ToString for OperatingSystem {
    fn to_string(&self) -> String {
        match &self {
            OperatingSystem::Darwin => "darwin".to_string(),
            OperatingSystem::Windows => "windows".to_string(),
            OperatingSystem::Linux => "linux".to_string(),
        }
    }
}

impl Default for OperatingSystem {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            OperatingSystem::Darwin
        }

        #[cfg(target_os = "linux")]
        {
            OperatingSystem::Linux
        }

        #[cfg(target_os = "windows")]
        {
            OperatingSystem::Windows
        }
    }
}

/// The cpu architectures enum
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Architecture {
    Amd64,
    I386,
    Arm32v6,
    Arm32v7,
    Arm64v8,
    Ppc64le,
}

impl ToString for Architecture {
    fn to_string(&self) -> String {
        match &self {
            Architecture::Amd64 => "amd64".to_string(),
            Architecture::I386 => "i386".to_string(),
            Architecture::Arm32v6 => "arm32v6".to_string(),
            Architecture::Arm32v7 => "arm32v7".to_string(),
            Architecture::Arm64v8 => "arm64v8".to_string(),
            Architecture::Ppc64le => "ppc64le".to_string(),
        }
    }
}

impl Default for Architecture {
    fn default() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Architecture::Amd64
        }

        #[cfg(target_arch = "x86")]
        {
            Architecture::I386
        }

        #[cfg(target_arch = "arm")]
        {
            Architecture::Arm32v7
        }

        #[cfg(target_arch = "aarch64")]
        {
            Architecture::Arm64v8
        }

        #[cfg(target_arch = "powerpc64")]
        {
            Architecture::Ppc64le
        }
    }
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
    let resp = reqwest::blocking::get(url).expect(&format!("Could not download: {}", url));
    let mut file = File::create(target).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes);
}

// Runtimes
fn download_wasmer(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Downloading wasmer";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmer/");

    pb.set_message(message_prefix);

    let url: &str = WASMER_RELEASES;
    let binary = "download/v4.0.0/wasmer-linux-amd64.tar.gz";

    pb.set_message(format!(
        "{message_prefix}{}",
        ": Creating the target directory"
    ));
    create_dir_all(download_dir).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));

    let resp =
        reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasmer");
    let mut file = File::create(download_dir.join("wasmer.tar.gz")).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");

    pb.set_message(format!("{message_prefix}: writing to disk"));
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wasmer");
}

fn install_wasmer(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Installing wasmer";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmer");
    let install_dir = &config.home_dir.join(".wavu/bin/wasmer");

    pb.set_message(message_prefix);

    let home: String = env::var("HOME").expect("Could not get the $HOME directory");
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
    copy_directory_contents(download_dir.to_str().unwrap(), install_dir.to_str().unwrap());

    pb.finish_with_message("Installed wasmer");
}

fn download_wasmtime(config: &Config, pb: &ProgressBar) {
    let message_prefix = "Downloading wasmtime";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasmtime/");
    let install_dir = &config.home_dir.join(".wavu/bin/wasmtime/");

    pb.set_message(message_prefix);

    let url: &str = WASMTIME_RELEASES;
    let binary = "download/v11.0.0/wasmtime-v11.0.0-x86_64-linux.tar.xz";

    info!("Creating the target directory");
    create_dir_all(download_dir).expect("No error creating the directory");

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    let resp =
        reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasmtime");
    let mut file =
        File::create(download_dir.join("wasmtime.tar.xz")).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes).expect("No error writing the file");

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
    copy_directory_contents(download_dir.to_str().unwrap(), install_dir.to_str().unwrap());

    pb.finish_with_message("Installed wasmtime");
}

fn download_wasm3(config: &Config, pb: &ProgressBar) {
    pb.set_message("Downloading wasm3");
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasm3/");
    let install_dir = &config.home_dir.join(".wavu/bin/wasm3/");

    let url: &str = WASM3_RELEASES;
    let binary = "download/v0.5.0/wasm3-linux-x64.elf";

    info!("Creating the target directory");
    create_dir_all(download_dir.join("bin")).unwrap();

    info!("Starting the download");
    info!("The actual url: {}", format!("{url}/{binary}"));
    let resp = reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasm3");
    let mut file = File::create(download_dir.join("wasm3")).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wasm3");
}

fn install_wasm3(config: &Config, pb: &ProgressBar) {
    pb.set_message("Installing wasm3");

    let download_dir = &config.cache_dir.join(".wavu/runtimes/wasm3/");
    let install_dir = &config.home_dir.join(".wavu/bin/wasm3/");

    copy_directory_contents(download_dir.to_str().unwrap(), install_dir.to_str().unwrap());
    pb.finish_with_message("Installed wasm3");
}

fn download_wazero(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Downloading wazero";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wazero/");
    let install_dir = &config.home_dir.join(".wavu/bin/wazero/");

    pb.set_message(message_prefix);

    let url: &str = WAZERO_RELEASES;
    let binary = "download/v1.3.0/wazero_1.3.0_linux_amd64.tar.gz";

    info!("Creating the target directory");
    create_dir_all(download_dir).unwrap();

    pb.set_message(format!("{message_prefix}: getting {url}/{binary}"));
    let resp = reqwest::blocking::get(format!("{url}/{binary}")).expect("Could not download wasm3");

    let mut file = File::create(download_dir.join("wazero.tar.gz")).expect("Could not open file");
    let bytes = resp.bytes().expect("Could not decode the file downloaded");
    file.write_all(&bytes).unwrap();

    pb.finish_with_message("Downloaded wazero");
}

fn install_wazero(config: &Config, pb: &ProgressBar) {
    let message_prefix: &str = "Installing wazero";
    let download_dir = &config.cache_dir.join(".wavu/runtimes/wazero/");
    let install_dir = &config.home_dir.join(".wavu/bin/wazero/");

    pb.set_message(message_prefix);

    let home: String = env::var("HOME").expect("Could not get the $HOME directory");
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
    copy_directory_contents(download_dir.to_str().unwrap(), install_dir.to_str().unwrap());

    pb.finish_with_message("Installed wazero");
}

fn install_all(config: &Config, runtimes: HashMap<&str, (fn(&Config, &ProgressBar), fn(&Config, &ProgressBar))>) {
    let mp = MultiProgress::new();
    let pstyle = ProgressStyle::default_spinner()
        .template("{spinner:.green} {wide_msg}")
        .expect("ProgresStyle::template direct input to be correct");

    mp.println("Starting downloading runtimes!").unwrap();

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

    let os = OperatingSystem::default().to_string();
    let arch = Architecture::default().to_string();

    println!("{:?}", args.runtimes);

    // The default config
    let mut config = Config {
        home_dir: dirs::home_dir().expect("Home dir to be present"),
        cache_dir: dirs::cache_dir().expect("Cache dir to be present"),
    };

    if config.home_dir.join(".wavu/wavu.conf.json").is_file() {
        let json_string = fs::read_to_string(config.home_dir.join(".wavu/wavu.conf.json")).expect("Failed to read the config.");
        config = serde_json::from_str(&json_string).expect("Failed to parse the config");
    }
    // The config should be loaded correctly after this point

    // mkdir -p $HOME_DIR/.wavu/bin/
    create_dir_all(config.home_dir.join(".wavu/bin/")).expect("Directory to be creted");

    // mkdir -p $CACHE_DIR/.wavu/runtimes/
    create_dir_all(config.cache_dir.join(".wavu/runtimes/")).expect("Directory to be creted");

    match os.as_ref() {
        "linux" => println!("OS is linux"),
        "darwin" => panic!("TODO: darwin aarch64"),
        "windows" => panic!("TODO: windows64"),
        _ => panic!("Not actually implemented"),
    }

    match arch.as_ref() {
        "amd64" => println!("Arch is AMD64"),
        _ => panic!("Not actually implemented"),
    }

    let mut runtimes: HashMap<&str, (fn(&Config, &ProgressBar), fn(&Config, &ProgressBar))> = HashMap::new();

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
