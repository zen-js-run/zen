use std::process::Command;
use std::io::{Error, ErrorKind};
use std::fs::metadata;

pub struct Formatter;

impl Formatter {
    pub fn new() -> Result<Self, Error> {
        Self::install_dprint()?;
        Ok(Formatter)
    }

    fn install_dprint() -> Result<(), Error> {
        let output = Command::new("cargo")
            .arg("install")
            .arg("dprint")
            .output()?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ));
        }

        Ok(())
    }

    pub fn format_js(&self, file_path: &str) -> Result<(), Error> {
        if metadata(file_path).is_err() {
            return Err(Error::new(ErrorKind::NotFound, format!("File not found: {}", file_path)));
        }

        let output = Command::new("dprint")
            .arg("format")
            .arg(file_path)
            .output()?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ));
        }

        Ok(())
    }
}