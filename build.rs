use cmake::Config;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    let dst = Config::new("./zlib")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("CMAKE_DEBUG_POSTFIX ", "")
        .build();

    Config::new("./freetype")
        .define("DISABLE_FORCE_DEBUG_POSTFIX", "TRUE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("FT_DISABLE_HARFBUZZ", "TRUE")
        .define("FT_DISABLE_BROTLI", "TRUE")
        .define("FT_DISABLE_PNG", "TRUE")
        .define("FT_DISABLE_BZIP2", "TRUE")
        .build();

    let debug_suffix = if target.contains("windows")
        && target.contains("msvc")
        && std::env::var("OPT_LEVEL").unwrap() == "0"
    {
        "d"
    } else {
        ""
    };

    println!("cargo:rustc-link-lib=static=z{}", debug_suffix);

    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-search=native={}/bin", dst.display());

    println!("cargo::rustc-link-lib=static=freetype");

    println!("cargo::rerun-if-changed=zlib");
    println!("cargo::rerun-if-changed=freetype");
}
