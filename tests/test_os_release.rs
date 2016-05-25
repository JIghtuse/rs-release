extern crate rs_release;

use rs_release::{OsReleaseError, parse_os_release};

#[test]
fn fails_on_io_errors() {
    for file in ["", "/etc/non_existing_file", "/etc/shadow"].iter() {
        assert_eq!(Err(OsReleaseError::Io), parse_os_release(file));
    }
}

#[test]
fn fails_on_parse_errors() {
    for file in ["tests/data/os-release-malformed-no-equal"].iter() {
        assert_eq!(Err(OsReleaseError::ParseError), parse_os_release(file));
    }
}

#[test]
fn parses_ok() {
    let path = "tests/data/os-release-one-env";
    let os_release = parse_os_release(path);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(1, os_release.len());
    assert_eq!("Fedora", os_release["NAME"]);
}
