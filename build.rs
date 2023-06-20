fn main() {
    // Compiling and building the GO FFI library and linking it to the Rust library
    #[cfg(feature = "multichain-impl")]
    {
        const LIB_PATH: &str = "multichain-cgo";
        extern crate bindgen;

        use std::env;
        use std::path::PathBuf;
        use std::process::Command;

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        let mut go_build = Command::new("go");
        go_build
            .arg("build")
            .arg("-buildmode=c-archive")
            .arg("-o")
            .arg(out_path.join(format!("lib{LIB_PATH}.a")))
            .arg(format!("./{LIB_PATH}/lib.go"));

        go_build.status().expect("Go build failed");

        let bindings = bindgen::Builder::default()
            .header(out_path.join(format!("lib{LIB_PATH}.h")).to_str().unwrap())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        println!("cargo:rerun-if-changed={LIB_PATH}/lib.go");
        println!(
            "cargo:rustc-link-search=native={}",
            out_path.to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=static={LIB_PATH}");
    }
}
