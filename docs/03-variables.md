# Variables and Constants

Understanding variables and constants is essential to programming in jRust. This guide covers declarations, mutability, and the differences between them.

## Variables: The Basics

### Declaration

Variables in jRust are declared with the `let` keyword and require explicit type annotations:

```typescript
let x: number = 42;
let name: string = "Alice";
let isActive: boolean = true;
```

### Default Mutability

In jRust, **variables are mutable by default**. This means you can reassign them:

```typescript
let count: number = 0;
count = 5;
count = 10;
```

This transpiles to Rust's `let mut`:

```rust
let mut count: i32 = 0;
count = 5;
count = 10;
```

### Why Mutable by Default?

Unlike Rust where variables are immutable by default, jRust makes them mutable by default for ergonomic reasons. This is safer than allowing unlimited mutation because jRust still enforces **Rust's ownership and borrowing rules**.

## Immutable Variables (Coming Soon)

In future versions, you'll be able to explicitly declare immutable variables:

```typescript
let immutable const x: number = 42;
// x = 50; // Error: cannot reassign immutable variable
```

For now, use **constants** (see below) for values that shouldn't change.

## Constants

Constants are declared with the `const` keyword and **must use ALL_CAPS naming**:

```typescript
const MAX_CONNECTIONS: number = 100;
const APP_NAME: string = "MyApp";
const API_URL: string = "https://api.example.com";
const DB_TIMEOUT: number = 30000;
```

### Constants vs Variables

| Feature | Variable (`let`) | Constant (`const`) |
|---------|------------------|-------------------|
| Declaration | `let x: type = value;` | `const X: type = value;` |
| Mutability | Mutable by default | Always immutable |
| Naming | camelCase | UPPER_SNAKE_CASE |
| Scope | Function scope | Module or global scope |
| Transpilation | `let mut` in Rust | `const` in Rust |

### When to Use Constants

Use constants for values that:
- Never change during program execution
- Represent configuration or settings
- Are shared across functions or modules

```typescript
const API_KEY: string = "sk-12345";
const PORT: number = 8080;
const DATABASE_URL: string = "postgres://localhost/mydb";

function initServer(): void {
    print("Starting server on port: " + PORT);
}
```

### Constant Transpilation

**jRust:**
```typescript
const MAX_SIZE: number = 100;

function validateSize(size: number): void {
    print("Max size: " + MAX_SIZE);
}
```

**Generated Rust:**
```rust
const MAX_SIZE: i32 = 100;

fn validateSize(size: i32) {
    println!("Max size: {}", MAX_SIZE);
}
```

## Type Annotations

Type annotations are **required** in jRust (type inference coming in future versions):

```typescript
let x: number = 10;              // ✓ Correct
let y: string = "hello";         // ✓ Correct
let z: boolean = true;           // ✓ Correct
let a = 5;                       // ✗ Error: missing type annotation
```

## Scoping

Variables follow standard scoping rules:

```typescript
function outer(): void {
    let x: number = 10;
    
    {
        let y: number = 20;
        print(x);    // ✓ Can access outer scope
        print(y);    // ✓ Can access inner scope
    }
    
    // print(y);    // ✗ Error: y is out of scope
}
```

## Shadowing

jRust allows variable shadowing (reusing the same name in nested scopes):

```typescript
function example(): void {
    let x: number = 10;
    print(x);        // Output: 10
    
    {
        let x: string = "hello";
        print(x);    // Output: hello
    }
    
    print(x);        // Output: 10
}
```

## Initialization Requirements

Variables must be initialized when declared:

```typescript
let x: number = 0;           // ✓ Initialized
let name: string = "John";   // ✓ Initialized
// let y: number;            // ✗ Error: uninitialized variable
```

## Ownership and Mutability

Variables own the values they hold. When a variable goes out of scope, its value is dropped:

```typescript
function example(): void {
    let message: string = "Hello";
    print(message);    // ✓ message owns the string
    // message is dropped here
}
```

For complex types (strings, objects), you can borrow references instead of moving ownership:

```typescript
function printMessage(msg: &string): void {
    print(msg);
}

function main(): void {
    let message: string = "Hello, jRust!";
    printMessage(&message);
    print(message);    // ✓ message still valid after borrowing
}
```

## Mutable Borrowing

When a function needs to mutate a variable, use `&mut`:

```typescript
function append(msg: &mut string): void {
    msg = msg + " World";
}

function main(): void {
    let greeting: string = "Hello";
    append(&mut greeting);
    print(greeting);   // Output: Hello World
}
```

## Best Practices

1. **Use descriptive names** — `userName` is better than `u`
2. **Use constants for fixed values** — Config, limits, API keys
3. **Keep variables in narrow scopes** — Declare them where needed
4. **Prefer constants for safety** — They prevent accidental mutation
5. **Use borrowing for efficiency** — Avoid unnecessary copies

## Summary

- Variables declared with `let` are **mutable by default**
- Constants declared with `const` are **immutable and always uppercase**
- Type annotations are **required**
- Variables **own their values** and follow Rust's ownership rules
- Use **references** (`&`) to borrow instead of moving values

Next, learn about the **[Primitive Types](04-primitive-types.md)** jRust supports.
