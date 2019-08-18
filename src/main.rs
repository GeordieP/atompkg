#![allow(unused)]

use std::cmp::Ordering;
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
        Err(e) => panic!("Could not open the packages directory \"{}\" {:?}", dir, e),
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

// look through defs, for each def's name, find an installed pkg with matching name.
// if a match IS NOT found, add the def to the output.
// if a match IS found, compare versions. If def version is higher, add def to the output.
fn compare_pkg_lists(defs: &Vec<PackageInfo>, installed: &Vec<PackageInfo>) -> Vec<PackageInfo> {
    let mut out: Vec<PackageInfo> = Vec::new();

    for def in defs {
        match installed.iter().find(|&pkg| pkg.name == def.name) {
            None => out.push(def.clone()),
            Some(p) => match PackageInfo::compare_versions(def, p) {
                Ordering::Greater => out.push(def.clone()),
                _ => {}
            },
        }
    }

    out
}

fn compare_and_install_packages(def_dir: &str, pkg_dir: &str) {
    let pkg_defs = read_user_pkg_defs(def_dir).unwrap();
    let installed_packages = get_installed_pkgs(pkg_dir).unwrap();
    let to_install = compare_pkg_lists(&pkg_defs, &installed_packages);

    for p in to_install {
        println!("apm install {}@{}", p.name, p.version.to_string());
    }
}

static DEFS_DIR: &str = "./test_files/packages.list";
static PKGS_DIR: &str = "./test_files/packages";

fn main() {
    compare_and_install_packages(DEFS_DIR, PKGS_DIR);
}

mod test {
    #[test]
    fn check_test_list() {
        // TODO
    }
}
