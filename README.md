# tencent-im-sys

# [Unofficial Crate]

## This crate is an unofficial crate created because the official Tencent IM SDK does not support Rust.

### Target Tencent IM SDK Version : TIM SDK 8.8.7354

## Build Requirement
https://rust-lang.github.io/rust-bindgen/requirements.html
~~~~~~~~
## How Use
After creating an sdk directory in the project's Manifest Dir, place TIM.lib and TIM.dll of Tencent IM SDK.

```Rust
// build.rs

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut sdk_path = PathBuf::new(manifest_dir);
    sdk_path.push("sdk");

    println!("cargo:rustc-link-search=native={}", sdk_path.display());
    println!("cargo:rustc-link-lib=static=ImSDK");

    println!("cargo:rerun-if_changed=build.rs");
    println!("cargo:rerun-if-changed=sdk/ImSDK.lib");

    // Copy dll
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../").join("ImSDK.dll");

    let dll_src = "sdk/ImSDK.dll";
    fs::copy(dll_src, dest_path).unwrap();
}

```