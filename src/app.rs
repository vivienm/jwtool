use std::fs;
use std::io;

use structopt::clap::crate_name;
use structopt::StructOpt;

use crate::cli;
use crate::error::Result;
use crate::jwt;

fn get_read(input: &cli::Input) -> Result<Box<dyn io::Read>> {
    Ok(match input {
        cli::Input::Stdin => Box::new(io::stdin()),
        cli::Input::Path(path) => Box::new(fs::File::open(path)?),
    })
}

fn get_write(output: &cli::Output) -> Result<Box<dyn io::Write>> {
    Ok(match output {
        cli::Output::Stdout => Box::new(io::stdout()),
        cli::Output::Path(path) => Box::new(fs::File::create(path)?),
    })
}

pub fn main(args: cli::Args) -> Result<()> {
    match args {
        cli::Args::Decode { input, output } => {
            jwt::decode(&mut get_read(&input)?, &mut get_write(&output)?)?;
        }
        cli::Args::Encode { input, output } => {
            jwt::encode(&mut get_read(&input)?, &mut get_write(&output)?)?;
        }
        cli::Args::Completion { shell, output } => {
            cli::Args::clap().gen_completions_to(crate_name!(), shell, &mut get_write(&output)?);
        }
    }
    Ok(())
}
