use std::io;

use crate::error::Result;

pub fn decode<R: io::Read, W: io::Write>(input: &mut R, output: &mut W) -> Result<()> {
    let mut token = String::new();
    input.read_to_string(&mut token)?;
    let value: serde_json::Value = jsonwebtoken::dangerous_insecure_decode(&token)?.claims;
    writeln!(output, "{}", serde_json::to_string_pretty(&value)?)?;
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
