mod cmd;
mod metadata;
mod manifest;
mod bundle;

fn main() -> Result<(), String> {
    let current_directory = std::env::current_dir().map_err(|e| format!("failed to resolve current directory: {}", e))?;
    let cli_args = crate::cmd::get_cli_input()?;
    let metadata = crate::metadata::get_metadata(&cli_args.manifest_path)?;
    let package_name = crate::manifest::get_package_name(&cli_args.manifest_path, &current_directory)?;
    let lines = crate::bundle::bundle(&package_name, &metadata.packages);

    if let Some(output_path) = cli_args.output_file {
        let path = current_directory.join(output_path);
        let text = lines.join("\n");
        std::fs::write(&path, text).map_err(|e| format!("failed to write credits file: path={}, err={}", &path.display(), e))
    } else {
        for l in lines {
            println!("{}", l);
        }
        Ok(())
    }
}

