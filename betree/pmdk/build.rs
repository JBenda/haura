extern crate bindgen;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=pmem");

    let bindings = bindgen::Builder::default()
        .blocklist_function("qfcvt.*")
        .blocklist_function("qgcvt.*")
        .blocklist_function("qecvt.*")
        .blocklist_function("strtold")
        .blocklist_function("gcvt")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
