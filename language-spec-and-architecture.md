# jRust — Language Overview & Project Architecture

## Short description

**jRust** is a small, interpreted, strongly-typed language that compiles/bundles to Rust. It aims to give developers the **performance and safety of Rust** while offering a **simple, readable high-level syntax**. jRust programs are easy to write and reason about, but compile to idiomatic Rust and then to native executables — and they can directly depend on Rust crates and use the jRust standard library.

## Goals & design principles

* **Performance-first:** Emit Rust code and reuse the Rust toolchain for optimized native builds.
* **Safety:** Strong static typing and sound runtime/stdlib guarantees; optional ownership-inspired checks initially.
* **Simplicity:** High-level, concise syntax geared toward readability and rapid iteration.
* **Interop:** Seamless ability to import and use Rust crates and call into native code.
* **Developer ergonomics:** Fast REPL/interpretation for development; proper tooling (LSP, VS Code extension).

## Key features (target)

* Explicit, strongly-typed surface language with type inference in later phases
* REPL / interpreted mode for rapid feedback
* Transpilation pipeline: jRust → idiomatic Rust → `cargo build` → native executable
* First-class support for Rust crates and FFI
* Standard library implemented as Rust crates (bundled + optional source)
* LSP for IDE features and a VS Code extension for syntax highlighting and integration

---

# High-level project architecture

The project is organized as a Rust *workspace* with several crates and companion folders for editor tooling, docs, and examples. The architecture intentionally separates concerns so parts (LSP, CLI, transpiler core, runtime) can be developed and released independently.

**Top-level components:**

* **crates/transpiler_core** — lexer, parser, AST, type checker, semantic analysis, IR, and Rust code generator.
* **crates/runtime** — small runtime helpers (memory model if needed, basic GC/ARC, runtime panics, OS bindings) implemented in Rust.
* **crates/std** — jRust standard library implemented as Rust crates (I/O, collections, concurrency, primitives).
* **crates/cli** — `jrust` CLI (build, run, init, fmt, test) that orchestrates transpile → cargo.
* **crates/lsp** — Language Server (LSP) implemented in Rust (reuses transpiler_core for diagnostics & features).
* **editor/vscode** — VS Code extension (TextMate or Tree-sitter grammar, language client that connects to the LSP, snippets, icons).
* **examples/** — sample projects and benchmarks.
* **docs/** — language spec, tutorials, design notes.
* **tools/** — codegen helpers, scripts for packaging, cross-compilation, `rustfmt`/`cargo` wrappers.

---

# Recommended file structure (Markdown tree)

```
jrust/                        # repo root (Cargo workspace)
├── Cargo.toml                 # workspace manifest
├── README.md
├── LICENSE
├── rust-toolchain             # pinned rust toolchain for reproducible builds
├── crates/
│   ├── transpiler_core/         # core transpiler crate
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── lexer.rs
│   │   │   ├── parser.rs
│   │   │   ├── ast.rs
│   │   │   ├── types.rs
│   │   │   ├── resolver.rs
│   │   │   ├── ir.rs
│   │   │   └── codegen_rust.rs
│   │   └── tests/
│   ├── runtime/               # runtime helpers used by generated code
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── mem.rs
│   ├── std/                   # standard library (may be multiple crates/modules)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── cli/                   # jrust CLI (jrust build/run/init)
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── lsp/                   # LSP server (tower-lsp)
│       ├── Cargo.toml
│       └── src/
├── editor/
│   └── vscode/
│       ├── package.json       # VS Code extension manifest
│       ├── syntaxes/          # TextMate grammar OR tree-sitter grammar
│       ├── client/            # language client code (TS) that spawns LSP
│       └── README.md
├── examples/
│   ├── hello-world.jr
│   └── cli-tool/              # example jRust project with Cargo integration
├── tools/
│   ├── gen_std.rs             # helper to generate std bindings
│   └── build_release.sh
├── docs/
│   ├── spec.md
│   └── tutorials/
└── ci/
    └── github-actions.yml
```

---

# Short descriptions of important crates/folders

* **transpiler_core**: The brain. Implement lexing, parsing, AST with accurate source spans, name resolution, type checking, semantic analysis, and a Rust code generator. Keep the public API small so LSP and CLI can call into it easily.

* **runtime**: Small Rust library that ships with jRust programs when necessary. It provides any required memory/runtime features not expressible in pure generated Rust (e.g., a lightweight GC, panic hooks, platform shims).

* **std**: The jRust standard library. Implement core modules in Rust and expose them as `jr_std` crate(s). Generated jRust projects link against these crates.

* **cli**: Orchestrates workflows: `jrust build` (transpile + `cargo build`), `jrust run` (build + run), `jrust fmt` (format or call rustfmt on generated code), `jrust init` (scaffold project), `jrust test`.

* **lsp**: Implements LSP endpoints (hover, completions, diagnostics, go-to-def) and reuses transpiler_core for authoritative results.

* **editor/vscode**: Contains grammar for syntax highlighting, snippets, and the lightweight client that launches the LSP server.

---

# Minimal MVP roadmap (short)

1. Design a small, well-scoped grammar (expressions, functions, structs, modules, imports, basic control flow).
2. Implement lexer + parser + AST in `transpiler_core`.
3. Implement type checker with explicit types (simple inference later).
4. Implement Rust code generator (emit `.rs` + `Cargo.toml`).
5. Implement CLI to transpile and call `cargo build` / `cargo run`.
6. Provide a tiny `std` and `runtime` crate.
7. Create a VS Code extension with TextMate grammar for highlighting and a basic `jrust` language configuration. Add LSP later.

---

# Suggested crates & tools (for the Rust side)

* `logos` — tokenizer (fast, ergonomic)
* `lalrpop` / `pest` / `nom` — parser choices depending on grammar complexity
* `tower-lsp` — build the Language Server
* `cargo` / `rustfmt` integration for build & formatting
* `insta` / `cargo-test` for snapshot and unit tests
* `cargo-fuzz` for fuzz testing later

---

# Notes on Rust interop & packaging

* Generated projects should create a `Cargo.toml` listing `jr_std` and `runtime` where appropriate. This leverages Cargo as your package manager initially.
* Allow `use crate::extern_crate("some_rust_crate")` syntax or an import macro to make adding Rust crates ergonomic.
* Provide source maps / metadata that map generated Rust back to jRust lines for debugging.

