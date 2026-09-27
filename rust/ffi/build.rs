fn main() {
    // Go finds the bundled dylib through -rpath, which requires an @rpath install name.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libseedfaker_ffi.dylib");
    }
}
