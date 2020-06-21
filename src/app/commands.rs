//! Commands which are available to run
//!

use crate::app::config::{dir, BASE_REGISTRY};
use crate::app::download::get;
use crate::app::fs::copy;
use crate::app::registry::crate_latest;
use crate::app::zip::extract;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::ops::Add;
use std::path::Path;

/// Downloads template create and uses the leap directory to create a new project
pub fn leap_new(template_crate_name: &str, project_name: &str) {
    let dir = dir().unwrap();
    log::info!("Downloading latest project crate {}", template_crate_name);
    let version = crate_latest(template_crate_name).expect("Unable to find template crate");
    let download_url = String::from(BASE_REGISTRY).add(version.dl_path.as_str());

    let crate_name = String::from(template_crate_name) + "-" + version.num.as_ref();
    let template_path = crate_name.clone() + ".tar.gzip";
    let cache_path = dir.cache_dir().join(template_path);

    if !Path::exists(cache_path.as_ref()) {
        let downloaded_file = get(download_url.as_ref()).expect("Unable to download crate");
        if downloaded_file.is_empty() {
            log::error!("Download failed. Empty response.");
            return;
        }

        create_dir_all(dir.cache_dir()).expect("Unable to create cache path");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(cache_path.clone())
            .expect("Unable to open cache file");

        file.write_all(downloaded_file.as_ref())
            .expect("Unable to write crate to cache");
    }

    let extracted_path = dir
        .cache_dir()
        .join(crate_name.clone())
        .join(crate_name.clone());
    if !Path::exists(extracted_path.as_ref()) {
        extract(
            cache_path.as_path(),
            dir.cache_dir().join(crate_name.clone()).as_path(),
        )
        .expect("Unable to extract crate");
    }

    log::info!("Looking for Leap directory");
    let leap_path = extracted_path.clone().join("leap");
    if !Path::exists(leap_path.as_ref()) {
        println!("The reference crate does not appear to have a leap template");
        return;
    }

    log::info!("Creating project from project crate {}", project_name);
    let current_working_directory =
        std::env::current_dir().expect("Unable to get current working directory");
    copy(leap_path, current_working_directory.join(project_name))
        .expect("Unable to create project");
}
