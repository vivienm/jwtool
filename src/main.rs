use structopt::StructOpt;

mod app;
mod cli;
mod error;
mod jwt;

use crate::error::Result;

fn main() -> Result<()> {
    app::main(cli::Command::from_args())
}
