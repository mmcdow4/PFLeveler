fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=SLINT_INCLUDE_GENERATED={}", out_dir);

    // slint_build::compile("ui\\summary_page.slint").unwrap();
    slint_build::compile("ui\\main_window.slint").unwrap();
}