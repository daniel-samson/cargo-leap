//! Provides the means to downloads a crate from registry
//!
use crate::app::config::USER_AGENT;
use async_std::task;

/// Make a GET request
pub fn get(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    task::block_on(async {
        let response = surf::get(url)
            .set_header("Accept", "*/*")
            .set_header("User-Agent", USER_AGENT)
            .await;
        match response {
            Ok(mut res) => {
                if res.status() == 302 {
                    let redirected_response = surf::get(res.header("Location").unwrap())
                        .set_header("Accept", "*/*")
                        .set_header("User-Agent", USER_AGENT)
                        .recv_bytes()
                        .await?;
                    return Ok(redirected_response);
                }

                return Ok(res.body_bytes().await?);
            }
            Err(e) => return Err(e),
        }
    })
}
