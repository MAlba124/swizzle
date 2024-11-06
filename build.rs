#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=feature=\"nightly\"");
}

#[rustversion::not(nightly)]
fn main() {
}
