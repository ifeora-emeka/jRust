use jrust_transpiler_core::{lexer::Lexer, parser::Parser, codegen::Codegen};

#[test]
fn test_single_import_std() {
    let input = r#"
import File from "std::fs";
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::fs::File;"));
}

#[test]
fn test_multiple_imports_std() {
    let input = r#"
import {Read, Write, Seek} from "std::io";
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::io::{Read, Write, Seek};"));
}

#[test]
fn test_import_with_alias() {
    let input = r#"
import HashMap from "std::collections" as Map;
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::collections::HashMap as Map;"));
}

#[test]
fn test_external_crate_import() {
    let input = r#"
import {Serialize, Deserialize} from "serde";
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use serde::{Serialize, Deserialize};"));
}

#[test]
fn test_local_module_import() {
    let input = r#"
import {add, multiply} from "./utils/math";
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use utils::math::{add, multiply};"));
}

#[test]
fn test_export_function() {
    let input = r#"
export function add(a: number, b: number): number {
    return a + b;
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("pub fn add"));
}

#[test]
fn test_export_struct() {
    let input = r#"
export struct Point {
    x: number,
    y: number
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("pub struct Point"));
    assert!(output.contains("pub x: i32"));
    assert!(output.contains("pub y: i32"));
}

#[test]
fn test_export_enum() {
    let input = r#"
export enum Status {
    Pending,
    Active,
    Completed
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("pub enum Status"));
    assert!(output.contains("Pending"));
    assert!(output.contains("Active"));
    assert!(output.contains("Completed"));
}

#[test]
fn test_export_const() {
    let input = r#"
export const PI = 3;
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("pub const PI"));
}

#[test]
fn test_multiple_imports_mixed() {
    let input = r#"
import {File, Read} from "std::fs";
import HashMap from "std::collections" as Map;
import {add, multiply} from "./utils/math";
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::fs::{File, Read};"));
    assert!(output.contains("use std::collections::HashMap as Map;"));
    assert!(output.contains("use utils::math::{add, multiply};"));
}

#[test]
fn test_import_and_export_combined() {
    let input = r#"
import {Read, Write} from "std::io";

export function processFile(path: string): void {
    print("Processing file");
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::io::{Read, Write};"));
    assert!(output.contains("pub fn processFile"));
}

#[test]
fn test_complex_module_program() {
    let input = r#"
import {File, Read} from "std::fs";
import {HashMap} from "std::collections";

export struct User {
    name: string,
    age: number
}

export function createUser(name: string, age: number): User {
    return User {
        name: name,
        age: age
    };
}

export const MAX_USERS = 1000;

function main(): void {
    let user = createUser("Alice", 30);
    print("User created");
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut codegen = Codegen::new();
    let output = codegen.generate(&ast);
    
    assert!(output.contains("use std::fs::{File, Read};"));
    assert!(output.contains("use std::collections::HashMap;"));
    assert!(output.contains("pub struct User"));
    assert!(output.contains("pub fn createUser"));
    assert!(output.contains("pub const MAX_USERS"));
    assert!(output.contains("fn main()"));
}

