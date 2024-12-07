use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .expect("Failed to update submodules.");
    println!("cargo:rerun-if-changed=./abc");
    let mut cfg = cmake::Config::new("abc");
    cfg.build_target("libabc");
    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=static=abc");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
