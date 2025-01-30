fn main() {
    println!("cargo:rustc-link-search=native=C:\\Npcap-SDK\\Lib\\x64");
    println!("cargo:rustc-link-lib=static=Packet");
    println!("cargo:rustc-link-lib=static=wpcap");
}
