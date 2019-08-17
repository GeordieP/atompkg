use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

type SimpleResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
struct PackageInfo {
    name: String,
    version: String,
}

impl PackageInfo {
    fn from_pkg_string(pkg_str: &str) -> Option<PackageInfo> {
        if pkg_str.is_empty() {
            return None;
        }
        let segments: Vec<&str> = pkg_str.split("@").collect();

        if segments.len() < 2 {
            println!("skipping line {}", pkg_str);
            return None;
        }

        let parsed_pkg = PackageInfo {
            name: String::from(segments[0]),
            version: String::from(segments[1]),
        };
        Some(parsed_pkg)
    }
}

fn read_pkg_json(file_path: PathBuf) -> SimpleResult<PackageInfo> {
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

            match read_pkg_json(p) {
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
            PackageInfo::from_pkg_string(&pkg_string_line)
        })
        .collect();

    Ok(package_infos)
}

fn main() {
    let pkg_defs = read_user_pkg_defs("/home/gp/.atom/packages.list").unwrap();

    println!("There are {} packages defined\n", pkg_defs.len());

    for p in pkg_defs {
        println!("{}@{}", p.name, p.version);
    }

    /* ------------------ */

    // let installed_packages = get_installed_pkgs("/home/gp/.atom/packages").unwrap();

    // println!(
    //     "There are {} packages installed\n",
    //     installed_packages.len()
    // );

    // for p in installed_packages {
    //     println!("{}@{}", p.name, p.version);
    // }
}
