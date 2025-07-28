use cmake::Config;

fn main() {
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

    let mut debug_suffix = "";

    let target = std::env::var("TARGET").unwrap();
    let libname = if target.contains("windows") && target.contains("msvc") {
        if std::env::var("OPT_LEVEL").unwrap() == "0" {
            debug_suffix = "d";
        }
        "zlibstatic"
    } else {
        "z"
    };

    println!("cargo:rustc-link-lib=static=z{}", debug_suffix);

    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-search=native={}/bin", dst.display());

    println!("cargo::rustc-link-lib=static=freetype");

    println!("cargo::rerun-if-changed=zlib");
    println!("cargo::rerun-if-changed=freetype");
}
