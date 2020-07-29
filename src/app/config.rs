//! Config for the command line tool
//!

use directories::ProjectDirs;

pub const BASE_REGISTRY: &str = "https://crates.io";
pub const USER_AGENT: &str = "cargo-leap (https://crates.io/crates/cargo-leap)";

/// Get project directories such as cache, config, etc...
pub fn dir() -> Option<ProjectDirs> {
    directories::ProjectDirs::from("rs", "cargo", "leap")
}
