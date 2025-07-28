use cmake::Config;

fn main() {
    let dst = cmake::build("./zlib");

    Config::new("./freetype")
        .define("DISABLE_FORCE_DEBUG_POSTFIX", "TRUE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("FT_DISABLE_HARFBUZZ", "TRUE")
        .define("FT_DISABLE_BROTLI", "TRUE")
        .define("FT_DISABLE_PNG", "TRUE")
        .define("FT_DISABLE_BZIP2", "TRUE")
        .build();

    println!("cargo::rustc-link-search=native={}/lib", dst.display());

    println!("cargo::rustc-link-lib=static=freetype");
    println!("cargo::rustc-link-lib=static=z");

    println!("cargo::rerun-if-changed=build.rs");
}
