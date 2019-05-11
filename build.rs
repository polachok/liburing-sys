extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();

	Command::new("rm")
		.arg("-rf")
		.arg(format!("{}/liburing", out_dir.clone()))
		.status()
		.expect("failed to remove");
	Command::new("cp")
		.arg("-r")
		.arg("liburing")
		.arg(out_dir.clone())
		.status()
		.expect("failed to copy");
    Command::new("make")
		.arg("liburing.a")
		.current_dir(format!("{}/liburing/src", out_dir.clone()))
		.status()
		.expect("failed to execute make");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=static=uring");
    println!("cargo:rustc-link-search=native={}/liburing/src", out_dir.clone());


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
	.clang_arg("-D_GNU_SOURCE")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
