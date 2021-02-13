use std::path::PathBuf;

use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Encode and decode JSON web tokens
pub enum Args {
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
    /// Generates a completion file
    Completion {
        /// Shell to produce a completion file for
        shell: Shell,
        /// Output file
        #[structopt(default_value = "-")]
        output: PathBuf,
    },
}
