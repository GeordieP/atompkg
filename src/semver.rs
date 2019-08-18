use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    pub fn from_str(vers_str: &str) -> SemVer {
        let mut result = SemVer::new(0, 0, 0);
        let segments: Vec<&str> = vers_str.split(".").collect();

        if let Some(first) = segments.get(0) {
            result.major = first.parse::<u32>().unwrap();
        }

        if let Some(first) = segments.get(1) {
            result.minor = first.parse::<u32>().unwrap();
        }

        if let Some(first) = segments.get(2) {
            result.patch = first.parse::<u32>().unwrap();
        }
        result
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn compare_versions(first: &SemVer, second: &SemVer) -> Ordering {
        if first.major == second.major && first.minor == second.minor && first.patch == second.patch
        {
            return Ordering::Equal;
        }
        if first.major > second.major {
            return Ordering::Greater;
        } else if second.major > first.major {
            return Ordering::Less;
        }

        if first.minor > second.minor {
            return Ordering::Greater;
        } else if second.minor > first.minor {
            return Ordering::Less;
        }

        if first.patch > second.patch {
            return Ordering::Greater;
        }

        Ordering::Less
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SemVer::from_str(&s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_versions(versions: (&str, &str), expected: Ordering) {
        let (a, b) = versions;
        let comparison = SemVer::compare_versions(&SemVer::from_str(a), &SemVer::from_str(b));
        assert_eq!(comparison, expected);
    }

    #[test]
    fn compare_versions_equal() {
        compare_versions(("0.0.0", "0.0.0"), Ordering::Equal);
    }

    #[test]
    fn compare_versions_major() {
        compare_versions(("1.2.3", "0.2.3"), Ordering::Greater);
        compare_versions(("0.2.3", "1.2.3"), Ordering::Less);
    }

    #[test]
    fn compare_versions_minor() {
        compare_versions(("1.2.3", "1.0.3"), Ordering::Greater);
        compare_versions(("1.0.3", "1.2.3"), Ordering::Less);
    }

    #[test]
    fn compare_versions_patch() {
        compare_versions(("1.2.3", "1.2.0"), Ordering::Greater);
        compare_versions(("1.2.2", "1.2.3"), Ordering::Less);
    }
}
