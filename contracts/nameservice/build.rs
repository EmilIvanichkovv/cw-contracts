fn main() {
    println!("cargo:rustc-link-arg=/path/to/my/o/file/add.o");
}

// Another approach could be:

// use std::process::Command;
// use std::env;
// use std::path::Path;

// fn main() {
//     let out_dir = env::var("OUT_DIR").unwrap();

//     Command::new("emar").args(&["rcs", "libadd.a", "/path/to/my/o/file/add.o"])
//                       .current_dir(&Path::new(&out_dir))
//                       .status().unwrap();
//     println!("cargo:rustc-link-search=native={}", out_dir);
//     println!("cargo:rustc-link-lib=add");
// }
