extern crate cmake;

fn main() {
    let dst = cmake::build("wrappers");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=llvm-wrappers");
}
