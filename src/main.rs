mod app;
mod crossterm;
mod ui;

use crate::crossterm::run;
use argh::FromArgs;
use std::error::Error;

/// Missile Launcher
#[derive(Debug, FromArgs)]
struct Cli {
    /// correct code to launch missile
    #[argh(option, default = "String::from(\"NTNUCSIE\")")]
    code: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = argh::from_env();
    run(args.code)?;
    Ok(())
}
