# jRust MVP Development Plan

## Goal
Create a minimal viable product that supports basic jRust programs with functions, variables, and console output.

## Target Features
- Variable declarations with explicit types (`number`, `string`)
- Function definitions with parameters and return types
- Console output via `print()` function
- String concatenation
- Transpilation to valid Rust code

## Development Steps (In Order)

### Step 1: Lexer (Tokenization) ✓ COMPLETED
**Goal:** Convert jRust source code into tokens

**Tasks:**
- ✓ Implement token types: `let`, `function`, identifiers, literals (numbers, strings), operators (`+`, `=`, `:`, `;`), keywords (`return`, `void`)
- ✓ Handle whitespace and comments
- ✓ Create token struct with position info for error reporting
- ✓ Write tests for valid and invalid tokens

**Input:** `let x: number = 42;`  
**Output:** `[LET, IDENT("x"), COLON, NumberType, EQUALS, NUMBER(42), SEMICOLON]`

**Files created:**
- `crates/transpiler_core/src/lexer.rs` - Full lexer implementation with 13 tests
- `crates/transpiler_core/src/token.rs` - Token types and struct

**Implementation notes:**
- Supports keywords: `let`, `const`, `function`, `return`, `void`, `print`, `mut`
- Supports type keywords: `number`, `string`, `boolean`
- Handles single-line comments (`// ...`)
- Supports all required operators: arithmetic (`+`, `-`, `*`, `/`, `%`), comparison (`==`, `!=`, `>`, `>=`, `<`, `<=`), logical (`&&`, `||`, `!`)
- String escape sequences: `\n`, `\t`, `\r`, `\\`, `\"`
- Line/column tracking for error reporting
- All 13 lexer tests passing

---

### Step 2: Parser (AST Construction)
**Goal:** Build an Abstract Syntax Tree from tokens

**Tasks:**
- Define AST node types: `Program`, `FunctionDecl`, `VariableDecl`, `ReturnStmt`, `CallExpr`, `BinaryExpr`, `Identifier`, `Literal`
- Implement recursive descent parser for:
  - Variable declarations: `let name: type = value;`
  - Function declarations: `function name(params): returnType { body }`
  - Return statements: `return expr;`
  - Function calls: `print("text")`
  - Binary expressions: `a + b`
- Add error recovery and helpful error messages
- Write comprehensive parser tests

**Input:** Token stream  
**Output:** AST representing the program structure

**Files to create:**
- `crates/transpiler_core/src/parser.rs`
- `crates/transpiler_core/src/ast.rs`

---

### Step 3: Type Checker
**Goal:** Validate types and ensure type safety

**Tasks:**
- Implement type system with: `number` (i32), `string` (String), `void` (unit type)
- Build symbol table to track variable and function declarations
- Validate:
  - Variable types match their assigned values
  - Function parameters match call arguments
  - Function return types match return statements
  - No use of undeclared variables
- Generate clear type error messages with source locations
- Write type checking tests (valid and invalid programs)

**Input:** AST  
**Output:** Type-checked AST or type errors

**Files to create:**
- `crates/transpiler_core/src/types.rs`
- `crates/transpiler_core/src/type_checker.rs`

---

### Step 4: Code Generator (Rust Emission)
**Goal:** Generate valid, idiomatic Rust code from type-checked AST

**Tasks:**
- Implement Rust code emitter that maps:
  - jRust types → Rust types (`number` → `i32`, `string` → `String`, `void` → `()`)
  - jRust functions → Rust functions (`function` → `fn`)
  - jRust variables → Rust let bindings
  - jRust `print()` → Rust `println!()` macro
  - String concatenation → Rust `format!()` or `.to_string() + &str`
- Generate proper Rust syntax with correct formatting
- Add source comments to link back to jRust source
- Write codegen tests to verify output

**Input:** Type-checked AST  
**Output:** Valid Rust source code as String

**Files to create:**
- `crates/transpiler_core/src/codegen.rs`

---

### Step 5: CLI Integration
**Goal:** Complete end-to-end workflow from jRust source to running executable

**Tasks:**
- Update CLI to:
  - Read `.jr` files from disk
  - Call lexer → parser → type checker → codegen pipeline
  - Write generated Rust to temporary `.rs` file
  - Invoke `cargo` to compile Rust code
  - Execute the resulting binary
- Implement `jrust run <file.jr>` command
- Add proper error handling and user-friendly messages
- Create example `.jr` programs in `examples/` directory
- Write integration tests

**Commands to implement:**
- `jrust run program.jr` - Transpile and run
- `jrust build program.jr` - Transpile to Rust (optional: compile)

**Files to update:**
- `crates/cli/src/main.rs`

**Files to create:**
- `examples/hello.jr`
- `examples/functions.jr`
- `examples/variables.jr`

---

## Success Criteria

The MVP is complete when you can:

1. Write this jRust program (`hello.jr`):
```typescript
function greet(name: string): void {
    print("Hello, " + name + "!");
}

function main(): void {
    let x: number = 42;
    greet("World");
    print("The answer is: " + x);
}
```

2. Run: `jrust run hello.jr`

3. See output:
```
Hello, World!
The answer is: 42
```

4. Generated Rust code is valid and idiomatic

5. All tests pass (lexer, parser, type checker, codegen, integration)

---

## Testing Strategy

Each step must have:
- Unit tests for core functionality
- Error case tests
- Integration tests (where applicable)

Run tests after each step: `cargo test`

---

## Dependencies to Add

Add to `workspace.dependencies` in root `Cargo.toml`:
- Consider: `logos` for lexer (fast tokenization)
- Consider: `miette` for beautiful error messages
- Consider: `insta` for snapshot testing

---

## Future Enhancements (Post-MVP)
- Control flow: `if`/`else`, `while`, `for`
- More types: `boolean`, arrays, objects/structs
- Type inference
- REPL/interactive mode
- LSP server for IDE support
- VS Code extension
