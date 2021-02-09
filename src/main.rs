use structopt::StructOpt;

mod app;
mod error;
mod jwt;

use crate::error::Result;

fn main() -> Result<()> {
    let command = app::Command::from_args();
    app::main(command)
}
