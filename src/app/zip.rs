//! Provides utilies to extract compressed archives
//!
use archiver_rs::{Archive, Compressed};

pub fn extract(path: &std::path::Path, extract_to: &std::path::Path) -> Result<(), String> {
    let mut gzip = archiver_rs::Gzip::open(path).expect("Unable to open compressed crate");
    let tarname = String::from(extract_to.to_str().unwrap()) + ".tar";
    gzip.decompress(extract_to.join(tarname.clone()).as_ref())
        .expect("unable to decompress crate");

    let mut tar = archiver_rs::Tar::open(extract_to.join(tarname).as_ref())
        .expect("Unable to decompressed crate");
    tar.extract(extract_to).expect("Unable to extract crate");
    Ok(())
}
