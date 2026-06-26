use crate::parser;
use std::fs;

pub fn run<I>(mut args: I) -> Result<(), String>
where
    I: Iterator<Item = String>,
{
    let _program = args.next();
    match args.next().as_deref() {
        None => print_help(),
        Some("help") => print_help(),
        Some("version") => {
            println!("langc 0.1.0");
            Ok(())
        }
        Some("parse") => {
            let path = args.next().ok_or_else(|| "missing input file".to_string())?;
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {path}: {error}"))?;
            let module = parser::parse_module(&source);
            println!("{module:#?}");
            Ok(())
        }
        Some(command) => Err(format!("unknown command: {command}")),
    }
}

fn print_help() -> Result<(), String> {
    println!("langc - language bootstrap compiler");
    println!();
    println!("usage:");
    println!("  langc help");
    println!("  langc version");
    println!("  langc parse <file>");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_commands() {
        let result = run(vec!["langc".to_string(), "bogus".to_string()].into_iter());
        assert!(result.is_err());
    }
}
