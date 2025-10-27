# Getting Started with jRust

Let's get you up and running with jRust in just a few minutes.

## Installation

### Prerequisites

- **Rust toolchain** (1.75.0 or later) — [Install Rust](https://rustup.rs/)
- **Node.js** (optional, for some tools)

### Install jRust

```bash
# Using jnet (jRust's package manager)
jnet install jrust

# Or download from GitHub
git clone https://github.com/ifeora-emeka/jRust.git
cd jRust
cargo install --path crates/cli
```

Verify the installation:

```bash
jrust --version
```

## Your First Program

Create a new file called `hello.jr`:

```typescript
function main(): void {
    print("Hello, jRust!");
}
```

Run it:

```bash
jrust run hello.jr
```

Output:
```
Hello, jRust!
```

## Project Structure

### Using jnet

jRust uses **jnet** as its package manager. Create a new project:

```bash
jnet init my-app
cd my-app
```

This creates a project with this structure:

```
my-app/
├── jrust.json          # Project configuration
├── src/
│   └── main.jr         # Entry point
├── jrust_modules.toml  # Dependencies (transpiles to Cargo.toml)
└── build/              # Output directory (contains generated Rust)
```

### jrust.json Configuration

The `jrust.json` file specifies your project configuration:

```json
{
  "name": "my-app",
  "version": "0.1.0",
  "entry": "src/main.jr",
  "output": "build/main.rs",
  "target": "bin"
}
```

**Key fields:**
- `entry` — Entry point for your application (what to compile)
- `output` — Where the transpiled Rust code is written (typically `build/main.rs`)
- `target` — `bin` for executables, `lib` for libraries

The entry point transpiles to `/build/main.rs` for the underlying Rust project.

## Project Commands

### jnet Commands

```bash
# Create a new project
jnet init <project-name>

# Build the project (transpile + compile)
jnet build

# Run the project
jnet start
# or
jnet run

# Install dependencies
jnet install <package-name>

# Format code
jnet fmt

# Run tests
jnet test
```

### Manual Compilation

If you're not using jnet, you can use `jrust` directly:

```bash
# Transpile and run
jrust run program.jr

# Build (transpile + compile)
jrust build program.jr

# Just transpile (output Rust code)
jrust transpile program.jr
```

## Project Layout Example

```
my-calculator/
├── jrust.json
├── src/
│   ├── main.jr
│   ├── math.jr
│   └── utils.jr
├── tests/
│   └── math_test.jr
└── examples/
    └── basic-operations.jr
```

Your `src/main.jr` might look like:

```typescript
function add(a: number, b: number): number {
    return a + b;
}

function main(): void {
    let result: number = add(5, 3);
    print("Result: " + result);
}
```

## Transpilation to Rust

When you run `jnet build` or `jrust build`, jRust transpiles your code to Rust. The generated Rust is placed in the `build/` directory.

**jRust:**
```typescript
let x: number = 42;
const APP_NAME: string = "Calculator";

function greet(name: &string): void {
    print("Hello, " + name);
}

function main(): void {
    let message: string = "World";
    greet(&message);
}
```

**Generated Rust (build/main.rs):**
```rust
let mut x: i32 = 42;
const APP_NAME: &str = "Calculator";

fn greet(name: &String) {
    println!("Hello, {}", name);
}

fn main() {
    let mut message: String = String::from("World");
    greet(&message);
}
```

## Dependencies

Manage dependencies using `jrust_modules.toml`, which transpiles to Rust's `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

Run `jnet install` to fetch dependencies.

## Next Steps

Now that you're set up, learn about:
- **[Variables and Constants](03-variables.md)** — Declaring and managing state
- **[Primitive Types](04-primitive-types.md)** — number, string, boolean, and more
- **[Functions](05-functions.md)** — Writing modular, reusable code

Happy coding! 🚀
