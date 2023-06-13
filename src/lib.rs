#[cxx::bridge]
mod ffi {
    // Any shared structs, whose fields will be visible to both languages.
    #[derive(Debug)]
    struct SomeNum {
        num: i64,
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {}
}
