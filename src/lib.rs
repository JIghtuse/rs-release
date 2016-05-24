use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum OsReleaseError {
    Io,
    NoFile,
}

impl From<std::io::Error> for OsReleaseError {
    fn from(_: std::io::Error) -> OsReleaseError {
        OsReleaseError::Io
    }
}

pub type Result<T> = std::result::Result<T, OsReleaseError>;

pub fn parse_os_release<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
    let mut os_release = HashMap::new();
    let file = try!(File::open(path));
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = try!(line);
        if let Some(equal) = line.chars().position(|c| c == '=') {
            let variable = &line[..equal];
            let value = &line[equal + 1..];
            os_release.insert(variable.to_string(), value.to_string());
        }
    }
    Ok(os_release)
}

pub fn get_os_release() -> Result<HashMap<String, String>> {
    if let Ok(os_release) = parse_os_release("/etc/os-release") {
        Ok(os_release)
    } else if let Ok(os_release) = parse_os_release("/usr/lib/os-release") {
        Ok(os_release)
    } else {
        Err(OsReleaseError::NoFile)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_os_release;

    #[test]
    fn it_works() {
        // TODO: Add some files to test
        let result = parse_os_release("/etc/os-release").unwrap();
        for (k, v) in result {
            println!("{} {}", k, v);
        }
    }
}
