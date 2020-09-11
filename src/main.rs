// I used the following code as a guide.
//   - https://github.com/Songmu/gocredits/blob/cb6a1f54c6/gocredits.go

#[macro_use]
extern crate clap;
#[cfg(test)]
use rstest::rstest;

use cargo::util::Config;
use clap::{App, ArgMatches, SubCommand};

mod cargo_toml;
mod exit;
mod license_score;
mod metadata;
mod output;
mod rust_license;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("credits").about(crate_description!()))
        .get_matches();

    let mut config = match Config::default() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("failed to load config: {}", e);
            exit::exit_from_error(e.into())
        }
    };

    match matches.subcommand() {
        ("credits", Some(args)) => run(args, &mut config),
        _ => {
            eprintln!("no exist subcommand");
            exit::exit(1);
        }
    }
}

fn run(args: &ArgMatches, config: &mut Config) {
    let current_package_name = match cargo_toml::get_package_name() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("{}", e);
            exit::exit(1);
        }
    };
    let rust_package = match rust_license::get_rust_license() {
        Ok(package) => package,
        Err(e) => {
            eprintln!("{}", e);
            exit::exit(1)
        }
    };
    let mut packages = metadata::get_metadata(args, config);
    packages.insert(0, rust_package);
    let text = output::create_credits(&current_package_name, &packages);
    match std::fs::write("./CREDITS", text) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("failed to write CREDITS: {}", e);
            exit::exit(1)
        }
    }
}
