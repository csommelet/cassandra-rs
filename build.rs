fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }

    println!("cargo:rustc-flags=-l dylib=cassandra");
    println!("cargo:rustc-flags=-l dylib=crypto");
    println!("cargo:rustc-flags=-l dylib=ssl");
    println!("cargo:rustc-flags=-l dylib=uv");
}
