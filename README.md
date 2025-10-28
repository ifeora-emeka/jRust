# jRust

**A simple, elegant way to write high-performance Rust code.**

## Why jRust?

### The Problem

Rust is an incredibly powerful language with unmatched performance and memory safety guarantees. However, its learning curve can be steep. Concepts like ownership, borrowing, lifetimes, and verbose syntax can feel overwhelming when you just want to build something fast.

Meanwhile, many developers are familiar with clean, expressive syntax found in modern programming languages but need better performance, type safety, and the ability to compile to native code for systems programming, CLI tools, or performance-critical applications.

The barrier to entry for Rust can prevent developers from leveraging its power for:
- **High-performance CLI tools** that need to be fast and portable
- **Systems programming** where native code is essential
- **Performance-critical applications** that require blazing speed
- **Cross-platform utilities** that compile to standalone binaries

### The Solution

**jRust bridges this gap.** It's a strongly-typed language that:

- **Uses familiar, readable syntax** - Clean, expressive code that's easy to understand and write
- **Compiles to idiomatic Rust** - Get all the performance and safety benefits of Rust without writing Rust directly
- **Generates native executables** - Your jRust code becomes fast, standalone binaries via the Rust toolchain
- **Simplifies complexity** - No manual memory management, ownership annotations, or lifetime specifiers in your source code
- **Enables rapid development** - Write less boilerplate, iterate faster, and focus on your application logic

### Who Is This For?

- **JavaScript/Node.js developers** who want native performance without learning Rust's complexity
- **Python developers** looking for compiled, high-performance alternatives for production systems
- **Rapid prototypers** who need fast, type-safe code without wrestling with ownership rules
- **Systems programmers** who want a simpler syntax for building CLI tools and utilities
- **Educators** teaching programming concepts with performance and safety in mind
- **Teams** that need Rust's benefits but want faster onboarding for new developers

### What Makes jRust Different?

jRust isn't just a syntax sugar layer - it's a carefully designed transpilation pipeline that:

1. **Transpiles to clean, idiomatic Rust code** (not FFI wrappers or runtime hacks)
2. **Provides familiar features**: type inference, structs, enums, array/string methods
3. **Generates production-ready binaries** via `cargo` with full optimization
4. **Maintains safety guarantees** through strong static typing
5. **Supports Rust interop** - use any Rust crate directly in your jRust code

---

## Quick Start

### Installation

#### Prerequisites

Before installing jRust, ensure you have the following installed on your system:

1. **Rust Toolchain** (version 1.75.0 or later)
   ```bash
   # Install via rustup (recommended)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Or on Windows, download from: https://rustup.rs/
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Git** (for cloning the repository)
   ```bash
   # Verify git is installed
   git --version
   ```

#### Installing jRust

```bash
# Clone the repository
git clone https://github.com/ifeora-emeka/jRust.git
cd jRust

# Build and install the jRust CLI
cargo install --path crates/cli --bin jrust

# Verify installation
jrust --version
```

### Your First jRust Program

```bash
# Create a new project
jrust init hello-world
cd hello-world

# Run your program
jrust run
```

This creates a project with a sample `src/index.jr` file that demonstrates jRust features.

---

## jRust CLI Commands

The jRust CLI provides four core commands for managing your projects:

### `jrust init <name>`

**Initialize a new jRust project**

Creates a complete project structure with all necessary configuration files.

```bash
jrust init my-app
```

**Creates:**
- `my-app/` - Project root directory
- `src/index.jr` - Main entry point with example code
- `jrust.toml` - Project configuration
- `.gitignore` - Git ignore rules

### `jrust check [path]`

**Check syntax and types without building**

Validates your jRust code for syntax errors and type correctness without generating Rust code or compiling.

```bash
# Check the default entry point (src/index.jr)
jrust check

# Check a specific file
jrust check src/custom.jr
```

**Output:**
- ✅ Lexical analysis passed
- ✅ Syntax parsing passed
- ✅ All checks passed!

### `jrust build [path]`

**Transpile and compile to native executable**

Converts your jRust code to Rust, then compiles it to an optimized native binary.

```bash
# Build the default entry point
jrust build

# Build a specific file
jrust build src/custom.jr
```

**Process:**
1. Lexical analysis (tokenization)
2. Syntax parsing (AST generation)
3. Code generation (Rust output)
4. Rust compilation (via `cargo`)

**Output:** `generated/target/release/` - Optimized executable

### `jrust run [path]`

**Build and execute your program**

Combines `build` and execution in one command - perfect for development.

```bash
# Run the default entry point
jrust run

# Run a specific file
jrust run src/custom.jr
```

---

## Language Features

jRust supports a rich set of features with familiar syntax from modern programming languages:

### Type System

```javascript
// Type inference
let x = 42;                    // Inferred as number
let name = "Alice";            // Inferred as string
let active = true;             // Inferred as boolean

// Explicit types
let age: number = 30;
let message: string = "Hello";
let items: number[] = [1, 2, 3];

// Any type (flexible typing)
let flexible: any = "can be anything";
```

### Variables and Constants

```javascript
// Mutable variables
let counter: number = 0;
counter = counter + 1;

// Immutable constants (MUST be UPPERCASE)
const MAX_SIZE: number = 100;
const API_URL: string = "https://api.example.com";
```

### Functions

```javascript
// Function with parameters and return type
function add(a: number, b: number): number {
    return a + b;
}

// Void functions (no return value)
function greet(name: string): void {
    print("Hello, " + name + "!");
}

// Call functions
let result: number = add(10, 20);
greet("World");
```

### Structs (Record Types)

```javascript
// Define a struct
struct User {
    name: string,
    age: number,
    active: boolean
}

// Create struct instances
let alice = User { 
    name: "Alice", 
    age: 30, 
    active: true 
};
```

### Enums

```javascript
// Simple enum
enum Status {
    Active,
    Inactive,
    Pending
}

// Enum with associated data
enum Result {
    Success(string),
    Error(string)
}
```

### Control Flow

```javascript
// If-else statements
if x > 10 {
    print("Greater than 10");
} else {
    print("Less than or equal to 10");
}

// For loops
for item in [1, 2, 3, 4, 5] {
    print(item);
}

// While loops
let count = 0;
while count < 5 {
    print(count);
    count = count + 1;
}

// Break and continue
for n in [1, 2, 3, 4, 5] {
    if n == 2 {
        continue;  // Skip 2
    }
    if n == 4 {
        break;     // Stop at 4
    }
    print(n);      // Prints: 1, 3
}
```

### Arrays

```javascript
// Array declaration
let numbers: number[] = [1, 2, 3, 4, 5];
let names: string[] = ["Alice", "Bob", "Charlie"];

// Array indexing
let first: number = numbers[0];

// Array methods
let length: number = numbers.length;
```

### String Methods

```javascript
let text: string = "Hello, World!";

// String methods
let upper = text.toUpperCase();      // "HELLO, WORLD!"
let lower = text.toLowerCase();      // "hello, world!"
let sub = text.substring(0, 5);      // "Hello"
let char = text.charAt(7);           // "W"
let index = text.indexOf("World");   // 7
```

### Output

```javascript
// Print to console
print("Hello, World!");
print(42);
print(variable);
```

---

## Development Setup

### For Contributors

If you want to contribute to jRust development:

#### 1. Clone and Build

```bash
git clone https://github.com/ifeora-emeka/jRust.git
cd jRust

# Build all crates
cargo build --all
```

#### 2. Project Structure

```
jRust/
├── crates/
│   ├── transpiler_core/   # Lexer, parser, AST, codegen
│   ├── cli/              # Command-line interface
│   ├── runtime/          # Runtime library
│   └── std/              # Standard library
├── docs/                 # Documentation
├── tests/               # Integration tests
└── examples/            # Example programs
```

#### 3. Development Commands

The project includes a `Makefile` for common development tasks:

```bash
# Run all tests (50+ tests)
make test

# Run specific test suites
make test-lexer      # Lexer tests only
make test-parser     # Parser tests only
make test-codegen    # Code generation tests only

# Build the project
make build

# Run the demo
make demo

# Code quality checks
make fmt             # Format code
make clippy          # Run linter

# Clean build artifacts
make clean
```

#### 4. Running Tests

```bash
# All tests
cargo test --all

# Specific crate tests
cargo test -p jrust_transpiler_core
cargo test -p jrust                    # CLI tests

# Specific test
cargo test test_parse_function_decl

# With output
cargo test -- --nocapture
```

#### 5. Development Workflow

1. **Make changes** to the relevant crate (`transpiler_core`, `cli`, etc.)
2. **Run tests** to ensure nothing breaks: `make test`
3. **Format code**: `make fmt`
4. **Run clippy**: `make clippy`
5. **Test manually**: `make demo` or create a test `.jr` file
6. **Commit changes** following conventional commits

#### 6. Testing Your Changes

```bash
# Create a test jRust file
cat > test.jr << 'EOF'
let x: number = 42;
print("The answer is:");
print(x);
EOF

# Check syntax
cargo run --bin jrust -- check test.jr

# Build and run
cargo run --bin jrust -- run test.jr
```

---

## Documentation

- **[📚 Full Documentation](docs/00-index.md)** - Complete language reference
- **[🚀 Getting Started Guide](docs/02-get-started.md)** - Your first jRust program
- **[📖 Language Specification](language-spec-and-architecture.md)** - Design and architecture
- **[🏗️ Architecture Overview](ARCHITECTURE.md)** - Transpilation pipeline details

### Key Documentation Topics

- **Variables** - [Variable declarations and types](docs/03-variables.md)
- **Functions** - [Function syntax and usage](docs/05-functions.md)
- **Control Flow** - [If/else, loops, break/continue](docs/10-control-flow.md)
- **Arrays** - [Array operations and methods](docs/11-arrays.md)
- **Structs & Enums** - [Advanced types](docs/12-advanced-types.md)
- **Array Methods** - [Comprehensive array API](docs/15-array-methods.md)
- **String Methods** - [String manipulation](docs/16-string-methods.md)

---

## Example: Complete Program

Here's a complete jRust program showcasing multiple features:

```javascript
// Define data structures
struct User {
    name: string,
    age: number,
    active: boolean
}

enum Role {
    Admin,
    User,
    Guest
}

// Create user instance
let alice = User { 
    name: "Alice", 
    age: 30, 
    active: true 
};

// Function to process data
function processUser(user: User): void {
    print("Processing user: " + user.name);
    
    if user.active {
        print("User is active");
    } else {
        print("User is inactive");
    }
}

// Array operations
let numbers: number[] = [1, 2, 3, 4, 5];
let total = 0;

for n in numbers {
    total = total + n;
}

print("Total: ");
print(total);

// String methods
let message: string = "Hello, jRust!";
let upper = message.toUpperCase();
print(upper);

// Call function
processUser(alice);
```

**This generates optimized Rust code and compiles to a native executable!**

---

## Roadmap

### Current Status (Phase 3.2) ✅

- ✅ Variables and constants
- ✅ Functions with parameters and returns
- ✅ Control flow (if/else, for, while, break, continue)
- ✅ Arrays and array methods
- ✅ String methods
- ✅ Type inference
- ✅ Structs (record types)
- ✅ Enums with variants
- ✅ Print statements

### Coming Soon

- 🚧 Classes and methods
- 🚧 Generics
- 🚧 Pattern matching
- 🚧 Module system
- 🚧 Error handling (try/catch)
- 🚧 Async/await support
- 🚧 FFI for calling Rust crates
- 🚧 LSP for IDE support
- 🚧 VS Code extension

See [Advanced Topics](docs/13-advanced.md) for the complete roadmap.

---

## Contributing

We welcome contributions! Whether it's:

- 🐛 Bug reports
- 💡 Feature suggestions
- 📝 Documentation improvements
- 🔧 Code contributions

**Steps to contribute:**

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and test thoroughly
4. Commit your changes: `git commit -m 'Add amazing feature'`
5. Push to the branch: `git push origin feature/amazing-feature`
6. Open a Pull Request

**Before submitting:**
- Run `make test` to ensure all tests pass
- Run `make fmt` to format code
- Run `make clippy` to check for common issues
- Add tests for new features

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Designed with developer-friendly syntax for rapid adoption
- Built with Rust's powerful toolchain for maximum performance
- Thanks to all contributors who help make jRust better!

---

## Support

- **Issues**: [GitHub Issues](https://github.com/ifeora-emeka/jRust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ifeora-emeka/jRust/discussions)
- **Documentation**: [docs/00-index.md](docs/00-index.md)

---

**Start writing simpler, faster code today with jRust!** 🚀
