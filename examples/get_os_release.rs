extern crate rs_release;

use rs_release::{get_os_release, parse_os_release};
use std::env;

fn main() {
    let args = env::args();

    let os_release = if let Some(os_release_path) = args.skip(1).next() {
        parse_os_release(os_release_path)
    } else {
        get_os_release()
    };

    match os_release {
        Ok(os_release) => {
            println!("Parsed os-release:");
            for (k, v) in os_release {
                println!("{}={}", k, v);
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}
