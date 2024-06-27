fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/call.cpp") // Ändern Sie dies von call.hpp zu call.cpp
        .flag_if_supported("-std=c++14")
        .compile("call");

    println!("cargo:rerun-if-changed=src/call.hpp");
    println!("cargo:rerun-if-changed=src/call.cpp"); // Fügen Sie dies hinzu
    println!("cargo:rerun-if-changed=src/lib.rs");
}