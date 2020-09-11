use crate::metadata::Package;
use rustc_version::{self, Version};

fn create_package(text: &String) -> Package {
    Package {
        name: "Rust".to_string(),
        manifest_path: "".to_string(),
        repository: Some("https://github.com/rust-lang/rust".to_string()),
        license_text: Some(text.clone()),
    }
}

fn create_raw_license_url(version: &Version) -> String {
    let tag = version.to_string();
    let url = format!(
        "https://raw.githubusercontent.com/rust-lang/rust/{}/COPYRIGHT",
        tag
    );
    url
}

fn get_license(url: &String) -> Result<String, String> {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(e) => return Err(format!("failed to fetch rust license: {}", e)),
    };
    match resp.text() {
        Ok(text) => Ok(text),
        Err(e) => Err(format!("failed to parse rust license: {}", e)),
    }
}

fn get_version() -> Result<Version, String> {
    match rustc_version::version() {
        Err(e) => Err(format!("failed to get rust version: {}", e)),
        Ok(version) => Ok(version),
    }
}

pub fn get_rust_license() -> Result<Package, String> {
    let version = get_version()?;
    let url = create_raw_license_url(&version);
    let license = get_license(&url)?;
    let package = create_package(&license);
    Ok(package)
}
