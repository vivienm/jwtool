use std::path::Path;
use std::{fs, io};

use crate::cli;
use crate::error::Result;
use crate::jwt;

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

pub fn main(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::Decode { input, output } => {
            jwt::decode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
        cli::Command::Encode { input, output } => {
            jwt::encode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
    }
    Ok(())
}
