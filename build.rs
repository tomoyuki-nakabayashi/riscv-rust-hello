extern crate cc;

use std::error::Error;
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `asm.s` file
    Build::new()
        .file("boot.s")
        .flag("-mabi=ilp32")
        .compile("asm");

    Ok(())
}
