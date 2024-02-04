use std::{env, io, ffi::OsStr};

use clap::{CommandFactory, Command};
use clap_complete::{generate_to, shells::{PowerShell, Elvish, Bash, Fish, Zsh}, Generator};

include!("src/cli.rs");

fn print_completions<G: Generator>(gen: G, cmd: &mut Command, outdir: &OsStr) -> Result<(), io::Error> {
    let path = generate_to(gen, cmd, cmd.get_name().to_string(), outdir)?;
    println!("cargo:warning=completion file is generated: {path:?}");
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => return Ok(()),
    };
    let mut cmd = Cli::command();
    print_completions(PowerShell, &mut cmd, outdir.as_os_str())?;
    print_completions(Bash, &mut cmd, outdir.as_os_str())?;
    print_completions(Elvish, &mut cmd, outdir.as_os_str())?;
    print_completions(Fish, &mut cmd, outdir.as_os_str())?;
    print_completions(Zsh, &mut cmd, outdir.as_os_str())?;
    Ok(())
}
