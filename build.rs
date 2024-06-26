use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dir = "./vendor";

    // Find the source files we need to build and sort them by C and C++.
    let (cfiles, cxxfiles): (Vec<_>, Vec<_>) = fs::read_dir(format!("{}/lib", lib_dir))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let ext = path.extension().and_then(|e| e.to_str());
            ext == Some("c") || ext == Some("cc")
        })
        .partition(|path| {
            let ext = path.extension().and_then(|e| e.to_str());
            ext == Some("c")
        });

    // Build a dummy library so the C files are compiled to objects, we need to link with these in
    // the final build.
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-fno-exceptions")
        .define("DEFAULT_HARDWARE", "\"regular\"")
        .include(format!("{}/lib", lib_dir))
        .include(format!("{}/include", lib_dir))
        .files(&cfiles)
        .files(&cxxfiles)
        .compile("librgbmatrix.a");

    bindgen::builder()
        .header(format!("{}/include/led-matrix-c.h", lib_dir))
        .derive_debug(true)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("librgbmatrix.rs"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=rgbmatrix");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
