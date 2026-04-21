fn main() {
    if let Err(error) = cliptype::run() {
        eprintln!("cliptype failed: {error}");
        std::process::exit(1);
    }
}
