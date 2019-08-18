use serde::Deserialize;
use std::cmp::Ordering;

use crate::semver::SemVer;

#[derive(Deserialize, Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: SemVer,
}

impl PackageInfo {
    pub fn new(name: String, version: SemVer) -> PackageInfo {
        PackageInfo { name, version }
    }

    pub fn from_pkg_str(pkg_str: &str) -> Option<PackageInfo> {
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
            version: SemVer::from_str(segments[1]),
        };
        Some(parsed_pkg)
    }

    pub fn compare_versions(first: &PackageInfo, second: &PackageInfo) -> Ordering {
        return SemVer::compare_versions(&first.version, &second.version);
    }
}
