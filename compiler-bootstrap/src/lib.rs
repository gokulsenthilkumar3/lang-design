pub mod ast;
pub mod cli;
pub mod parser;

pub fn run_from_env() -> Result<(), String> {
    cli::run(std::env::args())
}

