use clap::ArgMatches;
use cargo::util::Config;
use cargo::util::command_prelude::ArgMatchesExt;
use crate::exit;
use cargo::CliError;
use cargo::ops::OutputMetadataOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub manifest_path: String,
    pub repository: Option<String>,
    pub license_text: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct ExportInfo {
    packages: Vec<Package>
}

pub fn get_metadata(args: &ArgMatches, config: &mut Config) -> Vec<Package> {
    let ws = match args.workspace(config) {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("failed to analyze workspace.");
            exit::exit_from_error(CliError::from(e))
        }
    };

    let option = OutputMetadataOptions {
        features: vec![],
        no_default_features: false,
        all_features: false,
        no_deps: false,
        version: 1,
        filter_platforms: vec![]
    };

    let result = match cargo::ops::output_metadata(&ws, &option) {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("failed to get metadata.");
            exit::exit_from_error(CliError::from(e))
        }
    };
    let text = match serde_json::to_string(&result) {
        Ok(text) => text,
        Err(_) => {
            eprintln!("failed to analyze metadata.");
            exit::exit(1)
        }
    };

    let export_info: ExportInfo = match serde_json::from_str(&text) {
        Err(e) => {
            eprintln!("aaa");
            exit::exit(1)
        },
        Ok(info) => info
    };
    export_info.packages
}