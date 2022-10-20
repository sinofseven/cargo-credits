// I used the following code as a reference
//  https://github.com/Nemo157/cargo-lichking/blob/master/src/bundle.rs

use colored::Colorize;

pub fn bundle(package_name: &String, packages: &Vec<cargo_metadata::Package>) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("The {} uses some third party libraries under their own license terms.\n", package_name));

    for package in packages {
        if &package.name == package_name {
            continue;
        }

        let mut package_line = bundle_a_package(package);

        lines.append(&mut package_line);
    }

    lines
}

fn bundle_a_package(package: &cargo_metadata::Package) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push("====================".to_string());
    lines.push(format!("  package name: {}", package.name));
    lines.push(format!("  authors: {}", package.authors.join(", ")));

    if let Some(repository) = &package.repository {
        lines.push(format!("  repository: {}", repository))
    }

    if let Some(license) = &package.license {
        lines.push(format!("  license: {}", license));
    }

    let license_files = match get_license_file_names(package) {
        Err(e) => {
            eprintln!("{}: {}", "ERROR".red(), e);
            return lines;
        },
        Ok(files) => files
    };

    let mut license_lines = load_license_lines(&license_files);

    lines.append(&mut license_lines);

    lines
}

fn get_license_file_names(package: &cargo_metadata::Package) -> Result<Vec<std::path::PathBuf>, String> {
    let path_dir = match package.manifest_path.parent() {
        None => return Ok(Vec::new()),
        Some(path) => path
    };

    let objects = path_dir.read_dir().map_err(|e| format!("failed to read diretory: path={}, err={}", path_dir.as_str(), e))?;
    let licenses = objects.filter_map(|f| f.map(|e| e.path()).ok()).filter(|f| f.is_file()).filter(|f| f.file_name().map_or_else(|| false, |g| g.to_string_lossy().to_lowercase().contains("license"))).collect();

    Ok(licenses)
}

fn load_license_lines(license_files: &Vec<std::path::PathBuf>) -> Vec<String> {
    let mut lines = Vec::new();

    for path in license_files {
        let text = match std::fs::read_to_string(path) {
            Err(e) => {
                eprintln!("{}: failed to read license file: path={}, err={}", "ERROR".red(), path.display(), e);
                continue
            },
            Ok(s) => s
        };
        lines.push("--------------------".to_string());
        lines.push(text);
    }

    lines
}
