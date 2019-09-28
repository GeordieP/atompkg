#![allow(unused)]
mod package_info;
mod packages;
mod semver;
mod util;

use crate::package_info::PackageInfo;
use crate::packages::*;
use crate::util::SimpleResult;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::vec::Vec;

static DEFS_FILE: &str = "./test_files/packages.list";
// static PKGS_DIR: &str = "./test_files/packages-all";
// static PKGS_DIR: &str = "./test_files/packages-few";
static PKGS_DIR: &str = "/home/gp/.atom/packages";

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

fn install_or_update_packages(
    packages_dir: &str,
    definitions_file: &str,
    parallel_installs: usize,
) {
    let to_install = get_list_to_install(definitions_file, packages_dir);
    install_pkgs(to_install, PARALLEL_INSTALLS);
    println!("Package install complete");
}

fn main() {
    let matches = App::new("atompkg")
        .version("1.0")
        .author("Geordie P <gp@gpow.ca>")
        .about("Syncs Atom packages with a local definitions file.")
        .subcommand(SubCommand::with_name("install")
            .about("Installs or upgrades all Atom packages based on what is listed in your ~/.atom/packages.list file")
            .arg(Arg::with_name("batchsize")
                 .takes_value(true)
                 .short("b")
                 .long("batchsize")))
        .subcommand(SubCommand::with_name("dump")
            .about("Dumps all installed Atom packages and their version to ~/.atom/packages.list"))
        .get_matches();

    if matches.is_present("dump") {
        dump_installed_packages(PKGS_DIR, DEFS_FILE);
        println!("Dump finished");
        std::process::exit(0);
    }

    if matches.is_present("install") {
        let batch_size = matches
            .subcommand_matches("install")
            .map(|matches| matches.value_of("batchsize"))
            .map(|batch_size| batch_size.unwrap_or("").parse::<usize>().unwrap_or(5))
            .unwrap();

        install_or_update_packages(PKGS_DIR, DEFS_FILE, batch_size);

        println!("Install finished");
        std::process::exit(0);
    }
}

/* Tests */

#[cfg(test)]
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
