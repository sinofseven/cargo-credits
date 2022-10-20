use clap::{arg, command, Command};


#[derive(Debug, Clone)]
pub struct CliInput {
    pub manifest_path: Option<String>,
    pub output_file: Option<String>
}

pub fn get_cli_input() -> Result<CliInput, String> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("credits")
            .about("display licenses of dependencies")
            .arg(arg!(-m <MANIFEST_PATH> "path to Cargo.toml").long("manifest-path"))
            .arg(arg!(-o <OUTPUT_FILE> "path to output file (if this option is not specified, print to standard output)").long("output-file"))
        )
        .get_matches();
    if let Some((_, args)) = matches.subcommand() {
        Ok(CliInput {
            manifest_path: args.get_one::<String>("MANIFEST_PATH").map(|s| s.to_string()),
            output_file: args.get_one::<String>("OUTPUT_FILE").map(|s| s.to_string()),
        })
    } else {
        Err("".to_string())
    }
}