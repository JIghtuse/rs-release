extern crate rs_release;

use rs_release::get_os_release;

fn main() {
    match get_os_release() {
        Ok(os_release) => {
            println!("Parsed os-release:");
            for (k, v) in os_release {
                println!("{}={}", k, v);
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}
