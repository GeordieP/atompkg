#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

mod package_info;
mod semver;
use crate::package_info::PackageInfo;

type SimpleResult<T> = Result<T, Box<dyn std::error::Error>>;

fn parse_pkg_json_file(file_path: PathBuf) -> SimpleResult<PackageInfo> {
    let file = File::open(file_path)?;
    let contents_reader = BufReader::new(file);
    let parsed = serde_json::from_reader(contents_reader)?;
    Ok(parsed)
}

fn get_installed_pkgs(dir: &str) -> SimpleResult<Vec<PackageInfo>> {
    let paths = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) => panic!("Could not open the packages directory: {:?}", e),
    };

    let installed_pkgs = paths
        .filter_map(|p| {
            if let Err(e) = p {
                println!("[ERR] Unable to parse path {:?}", e);
                return None;
            }

            let mut p = p.unwrap().path();
            p.push("package.json");

            match parse_pkg_json_file(p) {
                Ok(pkg_info) => Some(pkg_info),
                _ => None,
            }
        })
        .collect();

    Ok(installed_pkgs)
}

fn read_user_pkg_defs(dir: &str) -> SimpleResult<Vec<PackageInfo>> {
    let file = File::open(dir)?;
    let package_infos = BufReader::new(file)
        .lines()
        .filter_map(|l| {
            if let Err(_e) = l {
                return None;
            }

            let l = l.unwrap();

            let pkg_string_line = l.as_str();
            PackageInfo::from_pkg_str(&pkg_string_line)
        })
        .collect();

    Ok(package_infos)
}

fn main() {
    // user's package list
    let pkg_defs = read_user_pkg_defs("/home/gp/.atom/packages.list").unwrap();

    println!("There are {} packages defined\n", pkg_defs.len());

    for p in pkg_defs {
        println!("{}@{}", p.name, p.version.to_string());
    }

    // already installed packages
    let installed_packages = get_installed_pkgs("/home/gp/.atom/packages").unwrap();

    println!(
        "\nThere are {} packages installed\n",
        installed_packages.len()
    );

    for p in installed_packages {
        println!("{}@{}", p.name, p.version.to_string());
    }
}
