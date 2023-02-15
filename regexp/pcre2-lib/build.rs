fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib/");
    println!("cargo:rustc-link-lib=dylib=pcre2-8");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
