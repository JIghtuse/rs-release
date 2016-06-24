use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::borrow::Cow;

const PATHS: [&'static str; 2] = ["/etc/os-release", "/usr/lib/os-release"];
const QUOTES: [&'static str; 2] = ["\"", "'"];

const COMMON_KEYS: [&'static str; 15] = ["ANSI_COLOR",
                                         "BUG_REPORT_URL",
                                         "BUILD_ID",
                                         "CPE_NAME",
                                         "HOME_URL",
                                         "ID",
                                         "ID_LIKE",
                                         "NAME",
                                         "PRETTY_NAME",
                                         "PRIVACY_POLICY_URL",
                                         "SUPPORT_URL",
                                         "VARIANT",
                                         "VARIANT_ID",
                                         "VERSION",
                                         "VERSION_ID"];

#[derive(Debug, PartialEq)]
pub enum OsReleaseError {
    Io,
    NoFile,
    ParseError,
}

impl From<std::io::Error> for OsReleaseError {
    fn from(_: std::io::Error) -> OsReleaseError {
        OsReleaseError::Io
    }
}

pub type Result<T> = std::result::Result<T, OsReleaseError>;

fn trim_quotes(s: &str) -> &str {
    // TODO: is it malformed if we have only one quote?
    if QUOTES.iter().any(|q| s.starts_with(q) && s.ends_with(q)) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn extract_variable_and_value(s: &str) -> Result<(Cow<'static, str>, String)> {
    if let Some(equal) = s.chars().position(|c| c == '=') {
        let var = &s[..equal];
        let var = var.trim();
        let val = &s[equal + 1..];
        let val = trim_quotes(val.trim()).to_string();

        if let Some(key) = COMMON_KEYS.iter().find(|&k| *k == var) {
            Ok((Cow::Borrowed(key), val))
        } else {
            Ok((Cow::Owned(var.to_string()), val))
        }
    } else {
        Err(OsReleaseError::ParseError)
    }
}

pub fn parse_os_release<P: AsRef<Path>>(path: P) -> Result<HashMap<Cow<'static, str>, String>> {
    let mut os_release = HashMap::new();
    let file = try!(File::open(path));
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = try!(line);
        let line = line.trim();

        if line.starts_with('#') {
            continue;
        }
        let var_val = try!(extract_variable_and_value(line));
        os_release.insert(var_val.0, var_val.1);
    }
    Ok(os_release)
}

pub fn get_os_release() -> Result<HashMap<Cow<'static, str>, String>> {
    for file in &PATHS {
        if let Ok(os_release) = parse_os_release(file) {
            return Ok(os_release);
        }
    }
    Err(OsReleaseError::NoFile)
}
