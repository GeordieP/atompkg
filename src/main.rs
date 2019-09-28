#![allow(unused)]

use std::fs;
use std::vec::Vec;

mod package_info;
mod packages;
mod semver;
mod util;

use crate::package_info::PackageInfo;
use crate::packages::*;
use crate::util::SimpleResult;

static DEFS_FILE: &str = "./test_files/packages.list";
// static PKGS_DIR: &str = "./test_files/packages-all";
// static PKGS_DIR: &str = "./test_files/packages-few";
static PKGS_DIR: &str = "/home/gp/.atom/packages";
static PARALLEL_INSTALLS: usize = 5;

fn get_list_to_install(def_dir: &str, pkg_dir: &str) -> Vec<PackageInfo> {
    let pkg_defs = read_user_pkg_defs(def_dir).unwrap();
    let installed_packages = get_installed_pkgs(pkg_dir).unwrap();
    compare_pkg_lists(&pkg_defs, &installed_packages)
}

fn dump_installed_packages(packages_dir: &str, definitions_file: &str) {
    let installed_list =
        get_installed_pkgs(PKGS_DIR).expect("Couldn't get installed packages list");

    let installed_strs: Vec<String> = installed_list.iter().map(|p| p.to_string()).collect();

    fs::write(definitions_file, installed_strs.join("\n"));
}

fn install_or_update_packages(packages_dir: &str, definitions_file: &str) {
    let to_install = get_list_to_install(definitions_file, packages_dir);
    install_pkgs(to_install, PARALLEL_INSTALLS);
    println!("Package install complete");
}

fn main() {
    install_or_update_packages(PKGS_DIR, DEFS_FILE);
    // dump_installed_packages(PKGS_DIR, DEFS_FILE);
}

/* Tests */

mod test {
    use super::*;

    static TEST_DEFS_FILE: &str = "./test_files/packages.list";
    static TEST_PKGS_DIR: &str = "./test_files/packages-all";

    #[test]
    fn check_test_list() {
        let to_install = get_list_to_install(TEST_DEFS_FILE, TEST_PKGS_DIR);
        let mapped: Vec<String> = to_install.iter().map(|p| p.to_string()).collect();
        let res = mapped.join("\n");

        let expected = "ide-rust@0.21.0\nslime@3.4.0";

        assert_eq!(res, expected);
    }
}
