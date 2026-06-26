use crate::ast::{Expression, Function, Item, Module, Statement};

pub fn parse_module(source: &str) -> Module {
    let items = source
        .lines()
        .filter_map(parse_line)
        .collect::<Vec<_>>();

    Module {
        name: None,
        items,
    }
}

fn parse_line(line: &str) -> Option<Item> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix("fn ") {
        let name = rest
            .split(|character| character == '(' || character == ' ' || character == '{')
            .next()
            .unwrap_or("anonymous")
            .to_string();
        return Some(Item::Function(Function {
            name,
            body: vec![Statement::Expression(Expression::Literal(trimmed.to_string()))],
        }));
    }

    Some(Item::Unknown(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_function_lines() {
        let module = parse_module("fn hello() {}\n");
        assert_eq!(module.items.len(), 1);
        match &module.items[0] {
            Item::Function(function) => assert_eq!(function.name, "hello"),
            other => panic!("expected function item, got {other:?}"),
        }
    }

    #[test]
    fn preserves_unknown_lines() {
        let module = parse_module("let x = 1\n");
        assert_eq!(module.items, vec![Item::Unknown("let x = 1".to_string())]);
    }
}
