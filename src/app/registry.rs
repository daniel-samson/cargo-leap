//! Provides meta data of a registry
//!

use crate::app::config::USER_AGENT;
use crates_io_api::{CrateResponse, Error, SyncClient, Version};

pub fn crate_latest(name: &str) -> Result<Version, Error> {
    // Instantiate the client.
    let client = SyncClient::new(USER_AGENT, std::time::Duration::from_millis(1000))?;
    // Retrieve summary data.
    let crate_response: CrateResponse = client.get_crate(name)?;
    let latest_version = crate_response.versions.first().unwrap();
    Ok(latest_version.clone())
}
