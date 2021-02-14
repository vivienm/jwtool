use std::ffi;
use std::fmt;
use std::path::PathBuf;

use structopt::clap::Shell;
use structopt::StructOpt;

#[inline]
fn is_dash(value: &ffi::OsStr) -> bool {
    value == "-"
}

#[derive(Debug)]
pub enum Input {
    Stdin,
    Path(PathBuf),
}

impl From<&ffi::OsStr> for Input {
    fn from(value: &ffi::OsStr) -> Self {
        if is_dash(value) {
            Self::Stdin
        } else {
            Self::Path(PathBuf::from(value))
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdin => write!(f, "<stdin>"),
            Self::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

#[derive(Debug)]
pub enum Output {
    Stdout,
    Path(PathBuf),
}

impl From<&ffi::OsStr> for Output {
    fn from(value: &ffi::OsStr) -> Self {
        if is_dash(value) {
            Self::Stdout
        } else {
            Self::Path(PathBuf::from(value))
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdout => write!(f, "<stdout>"),
            Self::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Encode and decode JSON web tokens
pub enum Args {
    /// Decodes a JSON web token
    Decode {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: Input,
        /// Output file
        #[structopt(parse(from_os_str), default_value = "-")]
        output: Output,
    },
    /// Encodes a JSON web token
    Encode {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: Input,
        /// Output file
        #[structopt(parse(from_os_str), default_value = "-")]
        output: Output,
    },
    /// Generates a completion file
    Completion {
        /// Shell to produce a completion file for
        shell: Shell,
        /// Output file
        #[structopt(parse(from_os_str), default_value = "-")]
        output: Output,
    },
}
