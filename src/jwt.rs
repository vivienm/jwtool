use std::io;

use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde::Serialize;

use crate::cli::JsonFormat;
use crate::error::Result;

fn serialize<F: serde_json::ser::Formatter, W: io::Write>(
    value: &serde_json::Value,
    formatter: F,
    output: &mut W,
) -> Result<()> {
    let mut serializer = serde_json::Serializer::with_formatter(output, formatter);
    value.serialize(&mut serializer)?;
    writeln!(serializer.into_inner())?;
    Ok(())
}

pub fn decode<R: io::Read, W: io::Write>(
    input: &mut R,
    output: &mut W,
    color: bool,
    format: &JsonFormat,
) -> Result<()> {
    let mut token = String::new();
    input.read_to_string(&mut token)?;
    let value: serde_json::Value = jsonwebtoken::dangerous_insecure_decode(&token)?.claims;
    match (color, format) {
        (false, JsonFormat::Compact) => {
            let formatter = CompactFormatter {};
            serialize(&value, formatter, output)
        }
        (false, JsonFormat::Pretty) => {
            let formatter = PrettyFormatter::new();
            serialize(&value, formatter, output)
        }
        (true, JsonFormat::Compact) => {
            let formatter = ColoredFormatter::new(CompactFormatter {});
            serialize(&value, formatter, output)
        }
        (true, JsonFormat::Pretty) => {
            let formatter = ColoredFormatter::new(PrettyFormatter::new());
            serialize(&value, formatter, output)
        }
    }
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

    use crate::cli::JsonFormat;

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
        decode(&mut input, &mut output, false, &JsonFormat::Pretty).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn test_encode() {
        let test_dir = get_test_dir();
        let input = fs::read(test_dir.join("example.json")).unwrap();
        let mut encoded = Vec::new();
        encode(&mut &input[..], &mut encoded).unwrap();
        let mut output = Vec::new();
        decode(&mut &encoded[..], &mut output, false, &JsonFormat::Pretty).unwrap();
        assert_eq!(input, output);
    }
}
