use std::fs;
use std::io;

use structopt::clap::crate_name;
use structopt::StructOpt;

use crate::cli;
use crate::error::Result;
use crate::jwt;

impl cli::ColorMode {
    fn use_color(&self, output: &cli::Output) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => match output {
                cli::Output::Stdout => {
                    colored_json::ColorMode::Auto(colored_json::Output::StdOut).use_color()
                }
                cli::Output::Path(_) => false,
            },
        }
    }
}

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
        cli::Args::Decode {
            input,
            output,
            color,
            format,
        } => {
            jwt::decode(
                &mut get_read(&input)?,
                &mut get_write(&output)?,
                color.use_color(&output),
                &format,
            )?;
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
