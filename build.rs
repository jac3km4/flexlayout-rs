use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cdeps/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("cdeps/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .compiler("clang")
        .file("cdeps/FlexLayout/src/FlexLayout.c")
        .compile("FlexLayout.a");
}
