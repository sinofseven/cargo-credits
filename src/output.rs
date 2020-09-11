use crate::metadata::Package;
use crate::license_score::LicenseScorer;
use std::path::PathBuf;

pub fn create_credits(current_package_name: &String, packages: &Vec<Package>) -> String {
    let mut lines: Vec<String> = Vec::new();
    let scorer = LicenseScorer::new();
    for package in packages {
        if current_package_name.eq(&package.name) {
            continue
        }
        let name = &package.name;
        let repository = match &package.repository {
            Some(repo) => repo,
            None => ""
        };
        let manifest = &package.manifest_path;
        let license = match get_license_text(package, &scorer) {
            Ok(tex) => tex,
            Err(tex) => tex
        };
        let inner_lines = [
            format!("{}", name),
            format!("{}", repository),
            format!("----------------------------------------------------------------"),
            format!("{}", license),
            format!("================================================================\n")
        ];
        lines.push(inner_lines.join("\n"));
    }

    lines.join("\n")
}

fn get_license_text(package: &Package, scorer: &LicenseScorer) -> Result<String, String> {
    if let Some(text) = &package.license_text {
        return Ok(text.clone());
    }

    let path_manifest = std::path::Path::new(&package.manifest_path);
    let path_dir = path_manifest.parent().unwrap();

    let dir_entries = match path_dir.read_dir() {
        Ok(entries) => entries,
        Err(e) => return Err(format!("failed to read directory: {}", e))
    };

    let mut score: u8 = 0;
    let mut str_path_license: Option<PathBuf> = None;

    for entry in dir_entries {
        if let Ok(entry) = entry {
            let os_file_name = entry.file_name();
            let name = os_file_name.to_str().unwrap();
            let path = entry.path();
            if path.is_dir() { continue; }
            let current_score = scorer.get_score(name);
            if score < current_score {
                score  = current_score;
                str_path_license = Some(path);
            }
        }
    }

    match str_path_license {
        None => Ok("Not Found License File".to_string()),
        Some(path) => match std::fs::read_to_string(path) {
            Err(e) => Err(format!("failed to read license: {}", e)),
            Ok(tex) => Ok(tex)
        }
    }
}