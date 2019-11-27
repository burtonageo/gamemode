#[cfg(target_os = "linux")]
fn main() {
    cc::Build::new()
        .file("./c_src/wrapper.c")
        .compile("gamemode_client");

    let bindings = bindgen::builder()
        .header("./c_src/wrapper.h")
        .ctypes_prefix("libc")
        .use_core()
        .generate()
        .map_err(|_| e("Could not generate bindings"))?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("gamemode_bindings.rs"))?;
}

#[cfg(not(target_os = "linux"))]
fn main() {
}
