use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare major
        return if self.major > other.major {
            Ordering::Greater
        } else if self.major == other.major {
            // Compare minor
            if self.minor > other.minor {
                Ordering::Greater
            } else if self.minor == other.minor {
                // Compare patch
                if self.patch > other.patch {
                    Ordering::Greater
                } else if self.patch == other.patch {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        };
    }
}
