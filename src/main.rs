use std::path::{Path, PathBuf};
use std::{fs, io};

use structopt::StructOpt;

mod error;
mod jwt;

use crate::error::Result;

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Encode and decode JSON web tokens
enum Command {
    /// Decodes a JSON web token
    Decode {
        /// Input file
        input: PathBuf,
        /// Output file
        #[structopt(default_value = "-")]
        output: PathBuf,
    },
    /// Encodes a JSON web token
    Encode {
        /// Input file
        input: PathBuf,
        /// Output file
        #[structopt(default_value = "-")]
        output: PathBuf,
    },
}

fn get_read<P: AsRef<Path>>(input: P) -> Result<Box<dyn io::Read>> {
    Ok(if input.as_ref() == Path::new("-") {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    })
}

fn get_write<P: AsRef<Path>>(output: P) -> Result<Box<dyn io::Write>> {
    Ok(if output.as_ref() == Path::new("-") {
        Box::new(io::stdout())
    } else {
        Box::new(fs::File::create(output)?)
    })
}

fn main() -> Result<()> {
    let command = Command::from_args();
    match command {
        Command::Decode { input, output } => {
            jwt::decode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
        Command::Encode { input, output } => {
            jwt::encode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
    }
    Ok(())
}
