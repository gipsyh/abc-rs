use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let src_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Environmental variable `CARGO_MANIFEST_DIR` not defined.".to_string())?;

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(src_dir).display()
    );
    println!("cargo:rustc-link-lib=static=abc");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
