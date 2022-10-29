extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=wintun/wintun_functions.h");

    let bindings = bindgen::Builder::default()
        .allowlist_function("Wintun.*")
        .allowlist_type("WINTUN_.*")
        .dynamic_link_require_all(true)
        .dynamic_library_name("wintun")
        .header("wintun/wintun_functions.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/wintun_raw.rs")
        .expect("Couldn't write bindings!");
}
