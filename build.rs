// build.rs

fn main() {
    // Tell cargo to rerun if the linker script changes
    println!("cargo:rerun-if-changed=linker.ld");
}
