extern crate cc;

fn main() {
    #[cfg(feature = "generate")]
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let i = std::fs::read_dir("libsofa_c/")
        .expect("Could not read folder")
        .filter_map(|entry| {
            if let Ok(e) = entry {
                let path = e.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "c" {
                            if let Some(stem) = path.file_stem() {
                                if stem != "t_sofa_c" {
                                    return Some(path);
                                }
                            }
                        }
                    }
                }
            }
            None
        });

    cc::Build::new()
        .files(i)
        .include("libsofa_c/")
        .compile("sofa_c");

    #[cfg(feature = "generate")]
    bindgen::Builder::default()
        .header("libsofa_c/sofa.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("iau.*")
        .whitelist_type("iau.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("sofa_c.rs"))
        .expect("Unable to save bindings");
}
