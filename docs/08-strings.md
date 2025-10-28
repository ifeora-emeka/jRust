# Strings

Strings are a fundamental data type in jRust. This guide covers string creation, manipulation, methods, and best practices.

## String Basics

### Creating Strings

Declare a string with the `string` type keyword or use type inference:

```javascript
let greeting: string = "Hello, jRust!";
let name = "Alice";           // Type inferred as string
let empty: string = "";
```

**Compiles to:**

```rust
let mut greeting: String = "Hello, jRust!".to_string();
let mut name = "Alice";
let mut empty: String = "".to_string();
```
