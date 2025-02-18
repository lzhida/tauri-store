mod command;
mod manifest;
mod path;
mod target;

use anyhow::Result;
use clap::Parser;
use command::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "tauri-store-cli")]
enum Cli {
  Codegen(Codegen),
  Docs(Docs),
}

fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Codegen(cmd) => cmd.run(),
    Cli::Docs(cmd) => cmd.run(),
  }
}
