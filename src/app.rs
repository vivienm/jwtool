use std::fs;
use std::io;
use std::path::Path;

use structopt::clap::crate_name;
use structopt::StructOpt;

use crate::cli;
use crate::error::Result;
use crate::jwt;

fn path_is_dash<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().as_os_str() == "-"
}

fn get_read<P: AsRef<Path>>(input: P) -> Result<Box<dyn io::Read>> {
    Ok(if path_is_dash(&input) {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(&input)?)
    })
}

fn get_write<P: AsRef<Path>>(output: P) -> Result<Box<dyn io::Write>> {
    Ok(if path_is_dash(&output) {
        Box::new(io::stdout())
    } else {
        Box::new(fs::File::create(&output)?)
    })
}

pub fn main(args: cli::Args) -> Result<()> {
    match args {
        cli::Args::Decode { input, output } => {
            jwt::decode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
        cli::Args::Encode { input, output } => {
            jwt::encode(&mut get_read(&input)?, &mut get_write(output)?)?;
        }
        cli::Args::Completion { shell, output } => {
            cli::Args::clap().gen_completions_to(crate_name!(), shell, &mut get_write(output)?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::path_is_dash;

    #[test]
    fn test_path_is_dash() {
        assert!(path_is_dash(Path::new("-")));
        assert!(!path_is_dash(Path::new("--")));
        assert!(!path_is_dash(Path::new(".")));
        assert!(!path_is_dash(Path::new("/tmp/foo")));
    }
}
