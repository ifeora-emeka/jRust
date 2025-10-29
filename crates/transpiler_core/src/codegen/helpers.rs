use crate::ast::{Expression, BinaryOp};

pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;
    let mut chars = name.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if prev_is_lower || (result.len() > 0 && chars.peek().map_or(false, |c| c.is_lowercase())) {
                if result.len() > 0 {
                    result.push('_');
                }
            }
            result.push(ch.to_lowercase().next().unwrap());
            prev_is_lower = false;
        } else {
            result.push(ch);
            prev_is_lower = ch.is_lowercase();
        }
    }
    
    result
}

pub fn convert_name(name: &str) -> String {
    if name.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric()) {
        name.to_uppercase()
    } else {
        to_snake_case(name)
    }
}

pub fn collect_string_parts<'a>(expr: &'a Expression, parts: &mut Vec<&'a Expression>) {
    match expr {
        Expression::BinaryOp(left, BinaryOp::Add, right) => {
            let left_is_string = matches!(**left, Expression::StringLiteral(_));
            let right_is_string = matches!(**right, Expression::StringLiteral(_));
            
            if left_is_string || right_is_string {
                collect_string_parts(left, parts);
                collect_string_parts(right, parts);
            } else {
                parts.push(expr);
            }
        }
        _ => {
            parts.push(expr);
        }
    }
}