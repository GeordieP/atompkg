use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use crate::package_info::PackageInfo;
use crate::util::SimpleResult;

pub fn parse_pkg_json_file(file_path: PathBuf) -> SimpleResult<PackageInfo> {
    let file = File::open(file_path)?;
    let contents_reader = BufReader::new(file);
    let parsed = serde_json::from_reader(contents_reader)?;
    Ok(parsed)
}

pub fn get_installed_pkgs(dir: &str) -> SimpleResult<Vec<PackageInfo>> {
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

pub fn read_user_pkg_defs(dir: &str) -> SimpleResult<Vec<PackageInfo>> {
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
pub fn compare_pkg_lists(defs: &[PackageInfo], installed: &[PackageInfo]) -> Vec<PackageInfo> {
    let mut out: Vec<PackageInfo> = Vec::new();

    for def in defs {
        match installed.iter().find(|&pkg| pkg.name == def.name) {
            None => out.push(def.clone()),
            Some(p) => {
                if let Ordering::Greater = PackageInfo::compare_versions(def, p) {
                    out.push(def.clone());
                }
            }
        }
    }

    out
}

pub fn install_pkgs(pkgs: Vec<PackageInfo>, pool_size: usize) -> SimpleResult<()> {
    let num_pkgs = pkgs.len();
    let pool = ThreadPool::new(pool_size);
    let (sender, receiver) = channel::<String>();

    for p in pkgs {
        let sender = sender.clone();

        pool.execute(move|| {
            let output = Command::new("apm")
                .arg("install")
                .arg(p.to_string())
                .output()
                .expect("Could not run command");

            let msg = std::str::from_utf8(&output.stdout)
                .expect("Could not parse string");

            sender.send(String::from(msg));
        });
    }

    receiver.iter().take(num_pkgs).for_each(|msg| {
        print!("{}", msg);
    });

    Ok(())
}
