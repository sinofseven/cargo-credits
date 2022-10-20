use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Package {
    name: String
}

#[derive(Debug, Deserialize)]
struct Manifest {
    package: Package
}

pub fn get_package_name(manifest_path: &Option<String>, current_directory: &std::path::Path) -> Result<String, String> {
    let path = if let Some(manifest_path) = manifest_path {
        current_directory.join(manifest_path)
    } else {
        current_directory.join("Cargo.toml")
    };

    let text = std::fs::read_to_string(&path).map_err(|e| format!("failed to read manifest: path={}, err={}", path.display(), e))?;

    let manifest: Manifest = toml::from_str(&text).map_err(|e| format!("failed to deserialize manifest: {}", e))?;

    Ok(manifest.package.name)
}