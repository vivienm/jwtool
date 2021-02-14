use std::ffi;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

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

#[derive(Debug)]
pub enum ColorMode {
    Always,
    Never,
    Auto,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Auto
    }
}

impl FromStr for ColorMode {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "auto" => Ok(Self::Auto),
            _ => Err("valid values: always, never, auto"),
        }
    }
}

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

impl ColorMode {
    pub fn variants() -> [&'static str; 3] {
        ["always", "never", "auto"]
    }
}

#[derive(Debug)]
pub enum JsonFormat {
    Pretty,
    Compact,
}

impl FromStr for JsonFormat {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pretty" => Ok(Self::Pretty),
            "compact" => Ok(Self::Compact),
            _ => Err("valid values: pretty, compact"),
        }
    }
}

impl fmt::Display for JsonFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pretty => write!(f, "pretty"),
            Self::Compact => write!(f, "compact"),
        }
    }
}

impl JsonFormat {
    pub fn variants() -> [&'static str; 2] {
        ["pretty", "compact"]
    }
}

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
/// Encode and decode JSON web tokens
pub enum Args {
    /// Decodes a JSON web token
    Decode {
        /// Color mode
        #[structopt(
            short = "c",
            long = "color",
            default_value = "auto",
            possible_values = &ColorMode::variants(),
        )]
        color: ColorMode,
        /// Formatting
        #[structopt(
            short = "f",
            long = "format",
            default_value = "pretty",
            possible_values = &JsonFormat::variants()
        )]
        format: JsonFormat,
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
        #[structopt(possible_values = &Shell::variants())]
        shell: Shell,
        /// Output file
        #[structopt(parse(from_os_str), default_value = "-")]
        output: Output,
    },
}
