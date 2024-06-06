extern crate rs_release;

use rs_release::{get_os_release, parse_os_release, parse_os_release_str};
use std::env;

fn main() {
    let mut args = env::args();

    let os_release = args.nth(1).map_or_else(get_os_release, parse_os_release);

    match os_release {
        Ok(os_release) => {
            println!("Parsed os-release:");
            for (k, v) in os_release {
                println!("{}={}", k, v);
            }
        }
        Err(e) => eprintln!("ERROR: {e}"),
    }
    println!();

    // You could also parse data from a string
    if let Ok(os_release) = parse_os_release_str("NAME = Fedora") {
        println!("Parsed os-release from &str:");
        for (k, v) in os_release {
            println!("{}={}", k, v);
        }
    }
}
