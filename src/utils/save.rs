use std::fs::{File, canonicalize};
use std::io::Write;
use std::path::PathBuf;

pub fn save(html: String, filename: &str) -> Result<PathBuf, std::io::Error> {
    let mut f = File::create(filename)?;
    f.write_all(html.as_bytes())?;
    let path = canonicalize(filename)?;
    Ok(path)
}
