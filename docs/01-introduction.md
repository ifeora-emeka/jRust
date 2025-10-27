# Introduction to jRust

Welcome to **jRust** — a TypeScript-like programming language that brings the simplicity and familiarity of JavaScript to the power and safety of Rust.

## What is jRust?

jRust is a strongly-typed, interpreted language that transpiles to Rust. It combines:
- **TypeScript-like syntax** — familiar to JavaScript developers
- **Rust's performance** — compiled to native executables
- **Rust's safety** — memory safety without garbage collection
- **Low-level capabilities** — without the complexity

## Why jRust?

### For JavaScript Developers
If you know TypeScript, you already know jRust syntax. Write low-level systems code without struggling with Rust's borrow checker learning curve. jRust enforces Rust's safety rules while maintaining familiar semantics.

### For Systems Programming
Get the speed and safety of Rust with a more approachable syntax. No need to master Rust's complex ownership rules explicitly — jRust guides you toward safe code patterns automatically.

## Design Philosophy

**jRust** is built on these core principles:

- **Performance-first:** Emit idiomatic Rust code and leverage the Rust toolchain for optimized native builds
- **Safety:** Strong static typing and memory safety guarantees inherited from Rust
- **Simplicity:** High-level, concise syntax focused on readability and rapid iteration
- **Compatibility:** Seamless interoperability with Rust crates and native code
- **Developer Experience:** Fast iteration with LSP support and VS Code integration

## Key Features

- **Explicit static typing** with type inference (coming in future versions)
- **Variables are mutable by default** — transpile to `let mut` for ergonomic mutation
- **Constants with ALL_CAPS naming** — immutable and guaranteed at compile time
- **Ownership and borrowing rules** — enforced at compile time, just like Rust
- **String concatenation** — use `+` operator like in TypeScript
- **Multiple logging levels** — `print()`, `print.error()`, `print.info()`, `print.warn()`, `print.debug()`
- **Direct Rust code generation** — inspect the generated Rust for debugging or optimization

## jRust vs Rust

| Feature | jRust | Rust |
|---------|-------|------|
| Syntax | TypeScript-like | Unique |
| Learning curve | Gentle | Steep |
| Safety | Full | Full |
| Performance | Native | Native |
| Compilation | Transpile + compile | Direct compile |
| Borrow checker | Enforced implicitly | Explicit |

## Your Journey

This documentation will guide you through:

1. **Installation and Setup** — Get jRust running on your machine
2. **Variables and Types** — Understanding jRust's type system
3. **Functions** — Writing and organizing code
4. **Ownership and Borrowing** — Memory safety patterns
5. **Output and Debugging** — Console logging and transpilation

Ready to get started? Head to **[Get Started](02-get-started.md)**.

## Module System

jRust's module system is similar to TypeScript's:
- Projects are organized with a `jrust.json` configuration file
- Namespaces follow TypeScript conventions
- The `jnet` package manager handles dependencies
- Transpiled code maintains the module structure in the generated Rust

## Transpilation

Every jRust program compiles through this pipeline:

```
jRust source (.jr)
    ↓
Lexer (tokenization)
    ↓
Parser (AST construction)
    ↓
Type Checker (validation)
    ↓
Code Generator
    ↓
Rust source (.rs)
    ↓
Rust compiler
    ↓
Native executable
```

## Next Steps

- Read **[Variables and Constants](03-variables.md)** to learn about jRust's type system
- Explore **[Functions](05-functions.md)** to see how to write modular code
- Dive into **[Ownership and Borrowing](07-ownership-and-borrowing.md)** for advanced memory patterns
