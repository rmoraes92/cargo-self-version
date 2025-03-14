///! # cargo self-version
///!
///! retrieves current version from a Cargo.toml
///!
///!
///! ## Install
///!
///! `$ cargo install cargo-self-version`
///!
///!
///! ## Usage
///!
///! ```
///! $ cargo self-version
///! x.y.z
///! ```
///!
///!
///! ## Examples
///!
///! ```bash
///! # store current Cargo.toml version in a variable
///! pkg_ver=$(cargo self-version)
///!
///! # maybe create a git tag
///! git tag $pkg_ver
///!
///! # or create a release with github cli
///! gh release create --title v$pkg_ver $pkg_ver
///! ```
use std::{
    path::{Path, PathBuf},
    process::exit,
};

use cfo::read_file;
use clap::Parser;
use toml::Table;

#[derive(Parser)]
#[command(version, about, long_about = None, name = "cargo", bin_name = "cargo")]
enum CargoWrapper {
    SelfVersion(Args),
}

/// retrieves current version from a given Cargo.toml file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "self-version", bin_name = "self-version")]
struct Args {
    /// path to either the Cargo.toml file or the folder where it is located.
    /// (if empty it will use current path to find a Cargo.toml)
    #[arg(short, long)]
    cargo_toml_path: Option<PathBuf>,
}

fn main() {
    let args = match CargoWrapper::parse() {
        CargoWrapper::SelfVersion(a) => a,
    };

    let path = match args.cargo_toml_path {
        Some(p) => {
            if p.is_dir() {
                p.join("Cargo.toml")
            } else {
                p
            }
        }
        None => Path::new("Cargo.toml").to_path_buf(),
    };

    if !path.exists() {
        println!("path does not exists: {}", path.display());
        exit(1);
    }

    let cargo_toml_body: String = match read_file(path.as_path()) {
        Ok(s) => s,
        Err(e) => {
            println!("err while reading {}: {}", path.display(), e);
            exit(1);
        }
    };
    let cargo_toml_table: Table = match cargo_toml_body.parse::<Table>() {
        Ok(t) => t,
        Err(e) => {
            println!("err while parsing {}: {}", path.display(), e);
            exit(1);
        }
    };
    match cargo_toml_table.get("package") {
        Some(v) => match v.as_table() {
            Some(m) => match m.get("version") {
                Some(v) => print!("{}", v.as_str().expect("version is not in string format")),
                None => (),
            },
            None => (),
        },
        None => (),
    };
}
