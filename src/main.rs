//! Feature matrix
//!
//! | target \ runtime | wasmer | wasmtime | wasm3 | wazero | timecraft | wasmedge |
//! |------------------+--------+----------+-------+--------+-----------+----------|
//! | linux amd64      | X      | X        | X     | X      |           |          |
//! | linux aarch64    |        |          |       |        |           |          |
//! | darwin amd64     |        |          |       |        |           |          |
//! | darwin aarch64   |        |          |       |        |           |          |
//! | windows amd64    |        |          |       |        |           |          |
//!
//! TODO: OS
//! linux aarch64
//! darwin
//! windows
//!
//! TODO: Runtimes
//! timecraft
//! wasmedge
//! wamr
//! libwasm
//! awsm
//! v8
//! jsc
//! deno
//! node
//! wasm2c
//!
//! TODO: features
//! get_versions
//! sort_versions
//! get_latest_version
//! run
//! benchmark
//! wasm_testsuite
//! wasi_testsuite
//! run same wasm module in multiple runtimes
mod runtimes;

use runtimes::common::Config;

use clap::Parser;
use indicatif::MultiProgress;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::{fs::create_dir_all, time::Duration};

use crate::runtimes::common::Cli;
use crate::runtimes::{spidermonkey, wasm3, wasmer, wasmtime, wazero};

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
                runtimes.insert("wasm3", (wasm3::download, wasm3::install));
            }
            "wasmer" => {
                runtimes.insert("wasmer", (wasmer::download, wasmer::install));
            }
            "wasmtime" => {
                runtimes.insert("wasmtime", (wasmtime::download, wasmtime::install));
            }
            "wazero" => {
                runtimes.insert("wazero", (wazero::download, wazero::install));
            }
            "spidermonkey" => {
                runtimes.insert(
                    "spidermonkey",
                    (spidermonkey::download, spidermonkey::install),
                );
            }
            runtime => {
                panic!("Unknown runtime '{}'", runtime);
            }
        }
    }

    install_all(&config, runtimes);
}
