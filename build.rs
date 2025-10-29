use std::path::PathBuf;

use cmake::Config;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    let bindings_path = PathBuf::from(&out_dir).join("bindings.rs");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/wrapper.h");
    println!("cargo::rerun-if-changed=zlib/CMakeLists.txt");
    println!("cargo::rerun-if-changed=freetype/CMakeLists.txt");

    let zlib_dst = Config::new("zlib")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("CMAKE_DEBUG_POSTFIX ", "")
        .out_dir(out_path.join("z"))
        .build();

    let freetype_dst = Config::new("freetype")
        .define("DISABLE_FORCE_DEBUG_POSTFIX", "TRUE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("FT_DISABLE_HARFBUZZ", "TRUE")
        .define("FT_DISABLE_BROTLI", "TRUE")
        .define("FT_DISABLE_PNG", "TRUE")
        .define("FT_DISABLE_BZIP2", "TRUE")
        .out_dir(out_path.join("freetype"))
        .build();

    let debug_suffix = if out_dir.contains("windows")
        && out_dir.contains("msvc")
        && std::env::var("OPT_LEVEL").unwrap() == "0"
    {
        "d"
    } else {
        ""
    };

    println!("cargo::rustc-link-search=native={}/lib", zlib_dst.display());
    println!(
        "cargo::rustc-link-search=native={}/lib",
        freetype_dst.display()
    );

    println!("cargo:rustc-link-lib=static=z{}", debug_suffix);
    println!("cargo::rustc-link-lib=static=freetype");

    let binding = bindgen::builder()
        .allowlist_item("^FT_.*$")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}/include/freetype2", freetype_dst.display()))
        .header("src/wrapper.h")
        .generate()
        .unwrap();

    binding.write_to_file(bindings_path).unwrap();
}
