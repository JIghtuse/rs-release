extern crate rs_release;

use std::process::Command;

#[derive(Debug)]
enum Error {
    UnknownOs,
    ReadError,
}

fn get_os_id() -> Result<String, Error> {
    rs_release::get_os_release().map_or(Err(Error::ReadError), |mut os_release| {
        os_release.remove("ID").ok_or(Error::UnknownOs)
    })
}

// https://blog.tinned-software.net/show-installed-yum-packages-by-size/
fn show_rpm_based_os_packages() {
    let mut command = Command::new("rpm");

    command.arg("--query");
    command.arg("--all");
    command.arg("--queryformat");
    command.arg("%10{size} - %-25{name} \t %{version}\n");

    if let Err(e) = command.spawn() {
        eprintln!("ERROR running rpm: {:?}", e);
    }
}

// http://www.commandlinefu.com/commands/view/3842/list-your-largest-installed-packages-on-debianubuntu
fn show_debian_packages() {
    let mut command = Command::new("dpkg-query");

    command.arg("--show");
    command.arg("--showformat");
    command.arg("${Installed-Size}\t${Package}\n");

    if let Err(e) = command.spawn() {
        eprintln!("ERROR running dpkg-query: {:?}", e);
    }
}

fn main() {
    match get_os_id() {
        Ok(id) => match id.as_str() {
            "fedora" | "opensuse-tumbleweed" => show_rpm_based_os_packages(),
            "debian" | "ubuntu" => show_debian_packages(),
            _ => eprintln!("ERROR: {:?} {}", Error::UnknownOs, id),
        },
        Err(e) => eprintln!("ERROR: {:?}", e),
    }
}
