fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg("Isdk/include")
        .layout_tests(false)
        .allowlist_function("TIM.*")
        .allowlist_type("TIM.*")
        .allowlist_var("TIM.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}