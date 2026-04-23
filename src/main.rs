#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = cliptype::run() {
        eprintln!("cliptype failed: {error}");
        std::process::exit(1);
    }
}
