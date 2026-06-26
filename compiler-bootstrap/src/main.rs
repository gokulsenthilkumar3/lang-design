fn main() {
    if let Err(error) = langc::run_from_env() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
