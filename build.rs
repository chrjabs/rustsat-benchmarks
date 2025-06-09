use std::{env, path::PathBuf};

fn main() {
    let src_files = [
        "open-wbo-capi/open-wbo.cpp",
        "open-wbo/encodings/Encodings.cc",
        "open-wbo/encodings/Enc_GTE.cc",
        "open-wbo/encodings/Enc_Adder.cc",
        "open-wbo/encodings/Enc_Totalizer.cc",
    ];

    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .define("NSPACE", "Glucose")
        .include(".")
        .include("open-wbo")
        .include("open-wbo/solvers/glucose4.1")
        .files(src_files);

    if env::var("PROFILE").unwrap() == "debug" {
        builder.opt_level(0).debug(true);
    } else {
        builder.opt_level(3).define("NDEBUG", None).warnings(false);
    }

    builder.compile("openwbo");

    bindgen::Builder::default()
        .clang_arg("-Iopen-wbo")
        .header("open-wbo-capi/open-wbo.h")
        .allowlist_file("open-wbo-capi/open-wbo.h")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=open-wbo-capi/");
    println!("cargo:rerun-if-changed=open-wbo/");
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=openwbo");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
