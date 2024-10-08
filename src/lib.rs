//! os-release parser
//!
//! # Usage example
//!
//! ```
//! use rs_release::parse_os_release;
//!
//! let os_release_path = "/etc/os-release";
//! if let Ok(os_release) = parse_os_release(os_release_path) {
//!     println!("Parsed os-release:");
//!     for (k, v) in os_release {
//!         println!("{}={}", k, v);
//!     }
//! } else {
//!     eprintln!("Cannot parse {}", os_release_path);
//! }
//! ```
#![deny(missing_docs)]

use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const PATHS: [&str; 2] = ["/etc/os-release", "/usr/lib/os-release"];
const QUOTES: [&str; 2] = ["\"", "'"];

const COMMON_KEYS: [&str; 30] = [
    "ANSI_COLOR",
    "ARCHITECTURE",
    "BUG_REPORT_URL",
    "BUILD_ID",
    "CONFEXT_LEVEL",
    "CONFEXT_SCOPE",
    "CPE_NAME",
    "DEFAULT_HOSTNAME",
    "DOCUMENTATION_URL",
    "HOME_URL",
    "ID",
    "ID_LIKE",
    "IMAGE_ID",
    "IMAGE_VERSION",
    "LOGO",
    "NAME",
    "PORTABLE_PREFIXES",
    "PRETTY_NAME",
    "PRIVACY_POLICY_URL",
    "SUPPORT_END",
    "SUPPORT_URL",
    "SYSEXT_LEVEL",
    "SYSEXT_SCOPE",
    "VARIANT",
    "VARIANT_ID",
    "VENDOR_NAME",
    "VENDOR_URL",
    "VERSION",
    "VERSION_CODENAME",
    "VERSION_ID",
];

/// Represents possible errors when parsing os-release file/string
#[derive(Debug)]
pub enum OsReleaseError {
    /// Input-Output error (failed to read file)
    Io(std::io::Error),
    /// Failed to find os-release file in standard paths
    NoFile,
    /// File is malformed
    ParseError,
}

impl PartialEq for OsReleaseError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (&Self::Io(_), &Self::Io(_))
                | (&Self::NoFile, &Self::NoFile)
                | (&Self::ParseError, &Self::ParseError)
        )
    }
}

impl fmt::Display for OsReleaseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Io(ref inner) => inner.fmt(fmt),
            Self::NoFile => write!(fmt, "Failed to find os-release file"),
            Self::ParseError => write!(fmt, "File is malformed"),
        }
    }
}

impl Error for OsReleaseError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            Self::Io(ref err) => Some(err),
            Self::NoFile | Self::ParseError => None,
        }
    }
}

impl From<std::io::Error> for OsReleaseError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// A specialized `Result` type for os-release parsing operations.
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
    s.chars().position(|c| c == '=').map_or_else(
        || Err(OsReleaseError::ParseError),
        |equal| {
            let variable = &s[..equal];
            let variable = variable.trim();
            let value = &s[equal + 1..];
            let value = trim_quotes(value.trim()).to_string();

            if let Ok(index) = COMMON_KEYS.binary_search(&variable) {
                Ok((Cow::Borrowed(COMMON_KEYS[index]), value))
            } else {
                Ok((Cow::Owned(variable.to_string()), value))
            }
        },
    )
}

fn parse_impl<S, L>(lines: L) -> Result<HashMap<Cow<'static, str>, String>>
where
    S: AsRef<str>,
    L: Iterator<Item = S>,
{
    let mut os_release = HashMap::new();
    for line in lines {
        let line = line.as_ref().trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let var_val = extract_variable_and_value(line)?;
        os_release.insert(var_val.0, var_val.1);
    }
    Ok(os_release)
}

/// Mapping of os-release variable names to values
type OsReleaseVariables = HashMap<Cow<'static, str>, String>;

/// Parses key-value pairs from `path`
pub fn parse_os_release<P: AsRef<Path>>(path: P) -> Result<OsReleaseVariables> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_impl(reader.lines().map(std::result::Result::unwrap_or_default))
}

/// Parses key-value pairs from `data` string
pub fn parse_os_release_str(data: &str) -> Result<OsReleaseVariables> {
    parse_impl(data.split('\n'))
}

/// Tries to find and parse os-release in common paths. Stops on success.
pub fn get_os_release() -> Result<OsReleaseVariables> {
    for file in &PATHS {
        if let Ok(os_release) = parse_os_release(file) {
            return Ok(os_release);
        }
    }
    Err(OsReleaseError::NoFile)
}
