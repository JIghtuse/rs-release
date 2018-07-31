extern crate rs_release;

use rs_release::{OsReleaseError, parse_os_release, parse_os_release_str};

#[test]
fn fails_on_io_errors() {
    for file in &["", "/etc/non_existing_file", "/etc/shadow"] {
        match parse_os_release(file) {
            Err(OsReleaseError::Io(_)) => {}
            err => panic!("Expected OsReleaseError::Io, but instead got {:?}", err),
        }
    }
}

#[test]
fn fails_on_parse_errors() {
    for file in &["tests/data/os-release-malformed-no-equal"] {
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

#[test]
fn trims_quotes() {
    let path = "tests/data/os-release-quotes-two-env";
    let os_release = parse_os_release(path);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(2, os_release.len());
    assert_eq!("Fedora 24 (Workstation Edition)", os_release["PRETTY_NAME"]);
    assert_eq!("cpe:/o:fedoraproject:fedora:24", os_release["CPE_NAME"]);
}

#[test]
fn unescapes_characters() {
    let path = "tests/data/os-release-escaped-three-env";
    let os_release = parse_os_release(path);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(3, os_release.len());
    assert_eq!("Multi-line\n\"Linux\"", os_release["NAME"]);
    assert_eq!(r#"0.1\n-same-line"#, os_release["VERSION"]);
    assert_eq!(
        "To escape \\t use double-quotes: \t",
        os_release["WEIRD_QUOTES"]
    );
}

#[test]
fn ignores_comments() {
    let path = "tests/data/os-release-comment";
    let os_release = parse_os_release(path);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(0, os_release.len());
}

#[test]
fn trims_whitespace() {
    let path = "tests/data/os-release-whitespace";
    let os_release = parse_os_release(path);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(2, os_release.len());
    assert_eq!("Fedora 24 (Workstation Edition)", os_release["PRETTY_NAME"]);
    assert_eq!("cpe:/o:fedoraproject:fedora:24", os_release["CPE_NAME"]);
}

#[test]
fn parses_from_str() {
    let data = r"

        # comment

        QUOTED_NAME = 'Fedora 24 (Workstation Edition)'

    PRETTY_NAME     =   Fedora 24 (Workstation Edition)

CPE_NAME=        cpe:/o:fedoraproject:fedora:24   ";
    let os_release = parse_os_release_str(data);
    assert!(os_release.is_ok());
    let os_release = os_release.unwrap();
    assert_eq!(3, os_release.len());
    assert_eq!("Fedora 24 (Workstation Edition)", os_release["PRETTY_NAME"]);
    assert_eq!("Fedora 24 (Workstation Edition)", os_release["QUOTED_NAME"]);
    assert_eq!("cpe:/o:fedoraproject:fedora:24", os_release["CPE_NAME"]);

    let os_release_malformed = parse_os_release_str("SOMETHING");
    assert_eq!(Err(OsReleaseError::ParseError), os_release_malformed);

    let os_release_empty = parse_os_release_str("");
    assert!(os_release_empty.is_ok());
    let os_release_empty = os_release_empty.unwrap();
    assert_eq!(0, os_release_empty.len());
}
