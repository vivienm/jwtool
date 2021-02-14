use colored_json::to_colored_json;
use std::io;

use crate::cli::ColorMode;
use crate::error::Result;

impl From<&ColorMode> for colored_json::ColorMode {
    fn from(color: &ColorMode) -> Self {
        match color {
            ColorMode::Always => Self::On,
            ColorMode::Never => Self::Off,
            ColorMode::Auto => Self::Auto(colored_json::Output::StdOut),
        }
    }
}

pub fn decode<R: io::Read, W: io::Write>(
    input: &mut R,
    output: &mut W,
    color: &ColorMode,
) -> Result<()> {
    let mut token = String::new();
    input.read_to_string(&mut token)?;
    let value: serde_json::Value = jsonwebtoken::dangerous_insecure_decode(&token)?.claims;
    writeln!(output, "{}", to_colored_json(&value, color.into())?)?;
    Ok(())
}

pub fn encode<R: io::Read, W: io::Write>(input: &mut R, output: &mut W) -> Result<()> {
    let value: serde_json::Value = serde_json::from_reader(input)?;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &value,
        &jsonwebtoken::EncodingKey::from_secret(b""),
    )?;
    write!(output, "{}", token)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::cli::ColorMode;

    use super::{decode, encode};

    fn get_test_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
    }

    #[test]
    fn test_decode() {
        let test_dir = get_test_dir();
        let mut input = fs::File::open(test_dir.join("example.jwt")).unwrap();
        let expected = fs::read(test_dir.join("example.json")).unwrap();
        let mut output = Vec::new();
        decode(&mut input, &mut output, &ColorMode::Never).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn test_encode() {
        let test_dir = get_test_dir();
        let input = fs::read(test_dir.join("example.json")).unwrap();
        let mut encoded = Vec::new();
        encode(&mut &input[..], &mut encoded).unwrap();
        let mut output = Vec::new();
        decode(&mut &encoded[..], &mut output, &ColorMode::Never).unwrap();
        assert_eq!(input, output);
    }
}
