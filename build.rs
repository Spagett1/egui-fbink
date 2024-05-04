use std::{env, path::PathBuf};

fn main() {
    if env::var("DOCS_RS").unwrap_or_else(|_| "0".to_string()) == "0" {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        // println!("out_path: {:?}", out_path);
        // /home/build/qos/toolchains/armv7l-linux-musleabihf-cross/armv7l-linux-musleabihf/lib
        // Well that's stupid
        std::fs::copy("/home/build/qos/toolchains/armv7l-linux-musleabihf-cross/armv7l-linux-musleabihf/lib/libdl.a", out_path.join("libdl.a")).expect("Failed to copy libdl.a library");
        println!("cargo:rerun-if-changed=wrapper.h");
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=static=dl");  
    }
}