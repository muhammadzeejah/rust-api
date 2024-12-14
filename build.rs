fn main() {
    println!("cargo:rustc-link-search=C:\\Program Files\\PostgreSQL\\17\\lib");
    println!("cargo:rustc-link-lib=libpq");
}