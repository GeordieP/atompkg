use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

type SimpleResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
struct PackageInfo {
    name: String,
    version: String,
}

fn read_pkg_json(file_path: PathBuf) -> SimpleResult<PackageInfo> {
    let file = File::open(file_path)?;
    let contents_reader = BufReader::new(file);
    let parsed = serde_json::from_reader(contents_reader)?;
    Ok(parsed)
}

fn get_installed_pkgs(dir: &str) -> Vec<PackageInfo> {
    let paths = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) => panic!("Could not open the packages directory: {:?}", e),
    };

    paths
        .filter_map(|p| -> Option<PackageInfo> {
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
        .collect()
}

fn main() {
    let installed_packages = get_installed_pkgs("/home/gp/.atom/packages");

    println!(
        "There are {} packages installed\n",
        installed_packages.len()
    );

    for p in installed_packages {
        println!("{}@{}", p.name, p.version);
    }
}
