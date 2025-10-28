# jRust

A Typescript-like programming language that compiles to Rust.

Use Rust to build jRust and its compiler, create a language server for IDE support, and build a VS Code extension for syntax highlighting. You'll also need a lexer, parser, AST, transpiler, and packaging tools.

## Quick Start

### Installation

```bash
cargo install --path crates/cli --bin jrust
```

### Create Your First Project

```bash
jrust init hello-world
cd hello-world
jrust run
```

### Learn More

- **[📚 Documentation Index](docs/00-index.md)** - Start here! Complete guide with learning paths
- **[Getting Started](docs/02-get-started.md)** - Installation and first program
- **[Language Features](docs/10-control-flow.md)** - Control flow guide (if/else, loops, break, continue)
- **[Arrays Guide](docs/11-arrays.md)** - Array operations and iteration
- **[Functions](docs/05-functions.md)** - Function declarations and calls
- **[Language Spec](language-spec-and-architecture.md)** - Architecture and design details
- **[Roadmap & Upcoming Features](docs/13-advanced.md)** - What's coming in future phases

### Development

```bash
make help          # Show development commands
make test          # Run all tests
make demo          # Run transpiler demo
make cli-help      # Show CLI help
```

---

# How To

Nice — great idea. Short, clear plan below.

# 1) Which implementation language?

**Use Rust.**
Reasons: fast, safe, great tooling (Cargo), easy to call `rustc`/`cargo` to produce OS executables, excellent crates for lexing/parsing/LSP, and you can reuse/emit idiomatic Rust as your backend target.
(Alternatives: **Go** for faster dev speed; **C++** for max control but more complexity; **TS/Node** for prototyping only.)

# 2) Language Server?

**Yes.** Build an LSP server (Diagnostics, Hover, Go-to-def, Completions, Formatting, References).
Implement it in Rust (e.g. `tower-lsp`) so it can reuse your compiler/typechecker and produce precise diagnostics.

# 3) VS Code syntax highlighting

You need:

- A **TextMate grammar** (`.tmLanguage.json`) _or_ a **Tree-sitter grammar** (recommended for robust highlighting and parsing).
- A small **VS Code extension** that registers the language, file extension(s), icons, snippets, and connects to your LSP server (language client).
  For semantic highlighting/errors you rely on the LSP.

# 4) Complete list of things to build (components + notes)

## Core language & compiler/transpiler

1. **Lexer** — tokenizes source. (Rust: `logos` or hand-rolled)
2. **Parser** — builds AST. (Rust: `lalrpop`, `pest`, or `nom`)
3. **AST representation** — typed Rust structs with source-span info.
4. **Name resolution / symbol table** — resolves imports, scopes.
5. **Type system / type checker** — strong typing rules. Choose:

   - Explicit static types first (easier), or
   - Gradual/Hindley–Milner type inference later.

6. **Semantic checks** — borrow/ownership rules (if you want Rust-like safety), lifetime checks, generics constraints.
7. **IR (optional)** — an intermediate form to simplify optimizations / codegen.
8. **Code generator to Rust** — map AST/IR → Rust source. Produce readable Rust with source maps/comments for debugging.
9. **Formatter / style** — either produce rustfmt-ready code or run `rustfmt` automatically on generated code.
10. **Interpreter / REPL** — interpreted runtime for fast feedback (interpreting AST or small bytecode VM). Useful for playground and tests.
11. **Runtime library (std)** — the standard library implemented in Rust (I/O, collections, concurrency, FFI wrappers) that your generated Rust links to.
12. **Foreign function interface** — expose C/Rust interop if you need native OS APIs.
13. **Error reporting & sourcemaps** — friendly errors tied to original jRust source.

## Tooling & Build

14. **Compiler CLI** (`jrustc`) — build, run, bundle flags (transpile → `cargo build --release`).
15. **Project scaffolding / templates** (`jrust init`)
16. **Package manager / dependency spec** (optional initially; could reuse Cargo by generating a `Cargo.toml`)
17. **Cross-compilation support / packaging** — use Cargo/cross/GHActions to produce target OS executables.
18. **Testing harness** — unit tests, integration tests, property tests (fuzzing).
19. **Benchmarking** — perf tests against generated Rust.

## IDE & Editor integration

20. **LSP server** (see above) — diagnostics, completion, go-to-def, refactorings.
21. **VSCode extension** — TextMate or Tree-sitter grammar, snippets, language client to your LSP, debugger support (DAP) later.
22. **Formatter plugin** — integrate `jrust fmt` or `rustfmt` for generated code.
23. **Language-aware formatter/linter** — static lints and style suggestions.

## Debugging & runtime introspection

24. **Source maps / mapping** from generated Rust back to jRust for stack traces.
25. **Debug Adapter** (DAP) to allow stepping through jRust code in VSCode (can initially map to Rust debug info).

## CI / Release / Docs / Community

26. **CI pipelines** (tests, builds for multiple platforms).
27. **Documentation site** with language spec, tutorials, examples.
28. **Examples & stdlib docs** — show how to interop with Rust and use std features.
29. **Package repo / registry** (optional later).
30. **License, contribution guide, security policy**.

## Quality / Safety / Maintenance

31. **Fuzzing & property tests** (e.g. `cargo-fuzz`).
32. **Benchmarks & regressions**.
33. **Telemetry (optional)** for error reports (respect privacy).

# Minimal **MVP** to start (priority)

1. Lexer + Parser → AST.
2. Simple typed language (explicit types) + type checker.
3. Codegen to Rust (emit `.rs` + `Cargo.toml`), and run `cargo build` to produce exe.
4. CLI (`jrust build` / `jrust run`) that automates rustfmt and cargo build.
5. Basic error reporting (source spans).
6. Small runtime stdlib in Rust.
7. VSCode: TextMate grammar + extension that recognizes files. (LSP later)

# Recommended crates / tools (Rust ecosystem)

- Lexer: `logos`
- Parser: `lalrpop` or `pest` or `nom` (choice depends on grammar complexity)
- Pretty-printing / codegen: `rustfmt` (call after emitting), `prettyplease` or `rowan` for syntax trees if needed
- LSP: `tower-lsp`
- Tree-sitter: for advanced highlighting (generate grammar)
- Testing: `insta` for snapshots, `cargo test`
- Fuzzing: `cargo-fuzz`
- CI: GitHub Actions (cross compilation), `cross` for cross builds

# Design advice / trade-offs

- **Emit Rust**: simple and lets the Rust toolchain optimize for you. Use generated Rust as the single source of truth for performance builds.
- **Interpreter + transpiler**: keep an interpreter for the REPL/playground (fast iteration) and a transpiler to Rust for production speed.
- **Borrow checker**: replicating Rust's ownership system is hard. For v1, implement simpler memory model + GC or reference counting in your runtime; later introduce ownership-inspired static checks.
- **Source maps**: invest early — debugging jRust will be unbearable without mapping to original source.

---

### jRust Syntax Overview

```ts
let x: number = 10;
const MY_VAR: number = 5;

function greet(name: string): void {
    print("Hello, " + name + "!");
}

let numbers: number[] = [1, 2, 3, 4, 5];
for (let n: number in numbers) {
    if (n % 2 == 0) {
        print("Even: " + n);
    }
}
```

### Current Features (Phase 3.1)

✅ **Language Features:**
- Variables and constants (`let`, `const`)
- All primitive types (number, string, boolean, void, any)
- User-defined functions with parameters and return types
- Conditionals (if/else with multiple conditions)
- Loops (for, while with full control)
- Loop control (break, continue)
- Arrays with indexing and iteration
- String operations and concatenation
- Comments (single and multi-line)

✅ **Memory Safety:**
- Ownership system
- Immutable references (`&`)
- Mutable references (`&mut`)
- Borrowing rules enforced at compile time

✅ **Tooling:**
- CLI (`jrust build`, `jrust run`)
- Project initialization (`jrust init`)
- Transpilation to Rust
- Automatic `rustfmt` formatting

🚀 **Coming Soon (Phase 3.2-3.3):**
- Type inference
- String methods (toUpperCase, toLowerCase, etc.)
- Array methods (map, filter, reduce, etc.)
- Struct types for custom data
- Enumerations
- Error handling (try/catch)
- Closures and lambda expressions

See [Roadmap & Upcoming Features](docs/13-advanced.md) for details.

### Variable & Type System

Variables:
- `let x: type = value;` — Mutable variable (generates `let mut x: RustType = value;`)
- `const MY_VAR: type = value;` — Immutable constant with UPPERCASE naming (generates `const MY_VAR: RustType = value;`)
- Note: jRust does not support the `mut` keyword. Mutability is determined by `const` (immutable) vs `let` (mutable)

