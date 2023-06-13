fn main() {
    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        .file("src/main.cc")
        .flag_if_supported("-std=c++20")
        .compile("rslibcc");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
