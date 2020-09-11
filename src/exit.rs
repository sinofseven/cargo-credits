use cargo::CliError;
use cargo::core::Shell;

pub fn exit(code: i32) -> ! {
    let e = CliError::code(code);
    let mut shell = Shell::new();
    cargo::exit_with_error(e, &mut shell)
}

pub fn exit_from_error(e: CliError) -> ! {
    let mut shell = Shell::new();
    cargo::exit_with_error(e, &mut shell)
}