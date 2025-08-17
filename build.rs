fn main() {
    // Tell Cargo to rerun if linker scripts change
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link");
} 