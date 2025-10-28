# Troubleshooting Guide

Having issues with jRust? This guide covers common problems and solutions.

## Common Issues

### Installation & Setup

#### "jrust: command not found"

**Problem:** The jRust CLI is not in your PATH.

**Solutions:**

Option 1: Reinstall with Cargo
```bash
cargo install --path crates/cli --bin jrust
```

Option 2: Use full path
```bash
./target/debug/jrust --version
```

Option 3: Ensure Cargo bin is in PATH
```bash
echo $PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

#### "Cargo not found"

**Problem:** Rust/Cargo is not installed.

**Solution:** Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then add to PATH:
```bash
source $HOME/.cargo/env
```

#### "Permission denied" when installing

**Problem:** Trying to install to system directory.

**Solution:** Use Cargo (handles permissions automatically)
```bash
cargo install --path crates/cli --bin jrust
```

### Project Creation & Running

#### "No such file or directory" when running jrust init

**Problem:** jRust binary is not in the correct location.

**Solutions:**

1. Check if jRust is installed:
```bash
which jrust
```

2. If not found, reinstall:
```bash
cargo install --path crates/cli --bin jrust
```

3. If still not working, use cargo run:
```bash
cargo run --bin jrust -- init my-project
```

#### Project won't run ("error: could not compile")

**Problem:** One of several compilation issues in transpiled Rust.

**Steps to debug:**

1. Check your jRust syntax:
```bash
jrust check  # If available, or try to transpile
```

2. Look at the generated Rust code:
```bash
jrust build  # This will show Rust compiler errors
```

3. Verify your code matches jRust syntax:
   - Use colons for type annotations: `let x: number = 10;`
   - Use `function` keyword for functions: `function foo(): void {}`
   - Use proper control flow syntax: `for (let i: number in range(0, 10)) {}`

4. Check the docs for feature availability:
   - See [Current Features](00-index.md#current-features-phase-31)
   - See [Advanced Topics](13-advanced.md) for coming soon features

### Type Errors

#### "Expected type X, found type Y"

**Problem:** Type mismatch in assignment or function call.

**Examples & Fixes:**

```typescript
// Wrong: assigning string to number
let x: number = "hello";
// Fix: use number
let x: number = 42;

// Wrong: returning wrong type
function getValue(): number {
    return "not a number";
}
// Fix: return the correct type
function getValue(): number {
    return 42;
}

// Wrong: number to string
let name: string = 123;
// Fix: use string
let name: string = "123";
```

#### "Type mismatch in array"

**Problem:** Array elements don't match declared type.

**Example & Fix:**

```typescript
// Wrong: declaring number array but adding string
let numbers: number[] = [1, 2, 3];
numbers = numbers.concat("four");  // Error!

// Fix: make sure all elements are correct type
let numbers: number[] = [1, 2, 3, 4];
```

#### "Cannot call method on type X"

**Problem:** Calling a method that doesn't exist or isn't available yet.

**Example & Fix:**

```typescript
// Wrong: trying to use toUpperCase() before Phase 3.2
let s: string = "hello";
let upper: string = s.toUpperCase();  // Error!

// For now: string concatenation works
let s: string = "hello";
```

**Note:** String methods are coming in Phase 3.2. See [Advanced Topics](13-advanced.md).

### Syntax Errors

#### "Unexpected token"

**Problem:** Invalid jRust syntax in your code.

**Common causes:**

1. Missing colons in type annotations:
```typescript
// Wrong
let x number = 10;
// Fix
let x: number = 10;
```

2. Wrong function syntax:
```typescript
// Wrong
fn greet(name) { print(name); }
// Fix
function greet(name: string): void {
    print(name);
}
```

3. Missing semicolons:
```typescript
// Wrong
let x: number = 10
let y: number = 20
// Fix
let x: number = 10;
let y: number = 20;
```

4. Wrong loop syntax:
```typescript
// Wrong
for (let i = 0; i < 10; i++) {
    print(i);
}
// Fix
for (let i: number in range(0, 10)) {
    print(i);
}
```

#### "Expected semicolon after statement"

**Problem:** Missing semicolon at end of statement.

**Solution:** Add semicolon

```typescript
// Wrong
print("Hello")
// Fix
print("Hello");
```

### Compilation Errors

#### "error: could not compile jRust due to previous error(s)"

**Problem:** Your jRust code transpiled successfully but the Rust code has errors.

**Steps to fix:**

1. Check your jRust syntax first
2. Look at generated Rust code (if visible in error)
3. Common causes:
   - Type mismatches
   - Incorrect borrowing
   - Wrong function signatures

#### Rust compiler error about "borrow checker"

**Problem:** Your code violates Rust's borrowing rules.

**Common scenario:**

```typescript
// Wrong: trying to use variable after moving it
let s: string = "hello";
let s2: string = s;  // s is moved
print(s);  // Error! s is no longer owned
```

**Fix: Use references**

```typescript
// Correct: using reference
let s: string = "hello";
let s2: &string = &s;  // borrow s
print(s);  // OK! s still owned
```

See [Ownership and Borrowing](07-ownership-and-borrowing.md) for details.

### Runtime Issues

#### "thread panicked" when running

**Problem:** Runtime error in generated Rust code.

**Common causes:**

1. Array index out of bounds:
```typescript
let arr: number[] = [1, 2, 3];
print(arr[5]);  // Panic! Index 5 doesn't exist
```

**Fix:** Check your array bounds
```typescript
let arr: number[] = [1, 2, 3];
print(arr[0]);  // OK: valid index
```

2. Integer overflow:
```typescript
let x: number = 2147483647;  // Max i32
x = x + 1;  // Could overflow
```

**Fix:** Use appropriate size or check before operating

#### Program produces no output

**Problem:** Your program runs but doesn't print anything.

**Causes & Fixes:**

1. Forgot to call `print()`:
```typescript
// No output:
let x: number = 42;

// Has output:
let x: number = 42;
print(x);
```

2. Logic error prevents execution:
```typescript
// Might not execute:
if (false) {
    print("Never prints");
}

// Will execute:
if (true) {
    print("Prints");
}
```

3. Using wrong print method:
```typescript
print("message");          // Prints to stdout
print.error("error");      // Prints to stderr (still visible)
print.warn("warning");     // Warning level
print.info("info");        // Info level
print.debug("debug");      // Debug level
```

## Debugging Strategies

### Check Your Syntax

1. Verify type annotations:
```typescript
// Each variable needs type
let name: string = "Alice";
let age: number = 30;
let active: boolean = true;
```

2. Verify function signatures:
```typescript
// Each function needs parameters and return type
function add(a: number, b: number): number {
    return a + b;
}
```

3. Verify control flow:
```typescript
// Correct if/else
if (condition) {
    // do something
} else {
    // do other thing
}

// Correct for loop
for (let i: number in range(0, 10)) {
    // loop body
}
```

### Enable Verbose Output

```bash
# Build with more detail
jrust build --verbose

# Run with debug info
jrust run --debug
```

### Check the Generated Rust

The transpiled Rust code is usually in a build directory:

```bash
# After running jrust build, look for target/ directory
cat target/generated.rs  # If available
```

### Isolate the Problem

1. Create a minimal reproduction:
```typescript
// Start with simplest possible code
let x: number = 10;
print(x);
```

2. Add features one at a time:
```typescript
// Add one feature
let x: number = 10;
print(x);

// Then add another
let y: number = 20;
print(y);
```

3. Test in isolation before integrating

### Read Error Messages Carefully

jRust compiler errors usually tell you exactly what's wrong:

```
error: expected type `number`, found `string`
  |
3 | let x: number = "hello";
  |                 ^^^^^^^
  | help: expected number literal
```

This tells you:
- What the error is (type mismatch)
- Where it is (line 3)
- What's wrong (assigned string instead of number)

## Frequently Asked Questions

### Q: Can I use Node.js libraries with jRust?

**A:** No. jRust compiles to Rust, not JavaScript. Use Rust crates instead.

### Q: Why do I get "module not found" errors?

**A:** The standard library features may not be implemented yet. See [Advanced Topics](13-advanced.md) for what's coming.

### Q: Can I mix jRust and Rust code?

**A:** Not directly in v1. File an issue on GitHub if this is important.

### Q: Why is compilation slow?

**A:** jRust transpiles to Rust, then Rust compiles to machine code. This involves two compilation steps. Rust compilation especially is thorough (for safety).

### Q: How do I optimize my jRust code?

**A:** Since jRust transpiles to Rust, use Rust optimization techniques:
- Use `jrust build --release` for optimized builds
- Check the generated Rust code for inefficiencies
- Consider performance-critical sections in native Rust

### Q: What happens if I have a bug in my generated Rust?

**A:** This should not happen. File a bug report with your jRust code and the error.

### Q: Can I use features from jRust that aren't documented?

**A:** No, stick to the documented features. Check [00-index.md](00-index.md) and [13-advanced.md](13-advanced.md) for what's available and coming soon.

## Getting More Help

### Documentation

- **[Documentation Index](00-index.md)** - All documentation
- **[Language Spec](../language-spec-and-architecture.md)** - Technical details
- **[Advanced Topics](13-advanced.md)** - Roadmap and features

### Community

- **GitHub Issues:** Report bugs and ask questions
- **GitHub Discussions:** General questions and ideas
- **Contribute:** Submit fixes and improvements

### Debugging Resources

1. Check existing issues: https://github.com/ifeora-emeka/jRust/issues
2. Search documentation for your issue
3. Create a minimal reproduction
4. File an issue with clear steps to reproduce

## Performance & Optimization

### Slow Compilation

**Cause:** Multiple compilation passes (jRust → Rust → machine code)

**Solutions:**
1. Use `jrust build` for development (faster debug build)
2. Use `jrust build --release` for production
3. Profile your code with Rust tools

### Slow Runtime

**Cause:** Unoptimized code or algorithm inefficiency

**Solutions:**
1. Check algorithm complexity
2. Use `--release` build flag
3. Profile with Rust profiling tools
4. Optimize hot paths in Rust (if needed)

### Large Binary Size

**Cause:** Debug build includes debug symbols

**Solution:** Use `--release` flag
```bash
jrust build --release
```

## Quick Reference

| Issue | Solution |
|-------|----------|
| "jrust: command not found" | Install: `cargo install --path crates/cli --bin jrust` |
| Type mismatch | Check type annotation: `: type` |
| Syntax error | Check syntax against docs |
| Borrow checker error | Use references properly: `&` and `&mut` |
| Runtime panic | Check array bounds and operations |
| No output | Add `print()` statement |
| Slow compilation | Use debug build for development |
| Large binary | Use `--release` flag |

## Still Having Issues?

1. Check the [Documentation Index](00-index.md)
2. Review the [Getting Started](02-get-started.md) guide
3. Look at similar examples in the [Arrays](11-arrays.md) or [Functions](05-functions.md) guides
4. Search [GitHub Issues](https://github.com/ifeora-emeka/jRust/issues)
5. Create a new issue with minimal reproduction

---

**Last Updated:** Phase 3.1
**For Issues:** Report on GitHub with minimal reproduction

