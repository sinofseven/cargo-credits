pub fn get_metadata(manifest_path: &Option<String>) -> Result<cargo_metadata::Metadata, String> {
    let mut command = cargo_metadata::MetadataCommand::new();

    if let Some(manifest_path) = manifest_path {
        command.manifest_path(manifest_path);
    }

    command.exec().map_err(|e| format!("failed to exec metadata: manifest_path={:?} err={}", manifest_path, e))
}