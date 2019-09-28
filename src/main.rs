mod package_info;
mod packages;
mod semver;

use crate::packages::*;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::vec::Vec;

static DEFS_FILE: &str = "./test_files/packages.list";
// static PKGS_DIR: &str = "./test_files/packages-all";
static PKGS_DIR: &str = "./test_files/packages-few";
// static PKGS_DIR: &str = "/home/gp/.atom/packages";

pub type SimpleResult<T> = Result<T, Box<dyn std::error::Error>>;

macro_rules! log {
    ($($arg:tt)*) => (println!("[atompkg] {}", format!($($arg)*)))
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
                 .default_value("5")
                 .short("b")
                 .long("batchsize")))
        .subcommand(SubCommand::with_name("dump")
            .about("Dumps all installed Atom packages and their version to ~/.atom/packages.list"))
        .get_matches();

    if matches.is_present("dump") {
        let installed_list =
            get_installed_pkgs(PKGS_DIR).expect("Couldn't get installed packages list");

        let installed_strs: Vec<String> = installed_list.iter().map(|p| p.to_string()).collect();

        fs::write(DEFS_FILE, installed_strs.join("\n"))
            .expect("Could not write to definitions file");

        log!("Package dump finished");
        std::process::exit(0);
    }

    if matches.is_present("install") {
        let batch_size = matches
            .subcommand_matches("install")
            .map(|matches| matches.value_of("batchsize"))
            .map(|batch_size| batch_size.unwrap().parse::<usize>().unwrap())
            .expect("Could not parse batch size");

        let pkg_defs = read_user_pkg_defs(DEFS_FILE).unwrap();
        let installed_packages = get_installed_pkgs(PKGS_DIR).unwrap();
        let to_install = compare_pkg_lists(&pkg_defs, &installed_packages);

        log!(
            "{} packages listed, {} already installed",
            pkg_defs.len(),
            installed_packages.len()
        );

        log!("Installing {} packages", to_install.len());
        install_pkgs(to_install, batch_size).expect("Failed to install packages");
        log!("Package install finished");

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
        let pkg_defs = read_user_pkg_defs(TEST_DEFS_FILE).unwrap();
        let installed_packages = get_installed_pkgs(TEST_PKGS_DIR).unwrap();
        let to_install = compare_pkg_lists(&pkg_defs, &installed_packages);

        let mapped: Vec<String> = to_install.iter().map(|p| p.to_string()).collect();
        let res = mapped.join("\n");

        let expected = "ide-rust@0.21.0\nslime@3.4.0";

        assert_eq!(res, expected);
    }
}
