use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoToml {
    package: Package,
}

pub fn get_package_name() -> Result<String, String> {
    let text = match std::fs::read_to_string("./Cargo.toml") {
        Ok(text) => text,
        Err(e) => {
            return Err(format!(
                "failed to read Cargo.toml in current directory ({})",
                e
            ))
        }
    };
    let resp: CargoToml = match toml::from_str(&text) {
        Err(e) => {
            return Err(format!(
                "failed to parse Cargo.toml in current directory ({})",
                e
            ))
        }
        Ok(resp) => resp,
    };

    Ok(resp.package.name)
}
