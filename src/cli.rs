use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Encode and decode JSON web tokens
pub enum Command {
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
