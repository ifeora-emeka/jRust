# Variables and Constants

Understanding variables and constants is essential to programming in jRust. This guide covers declarations, mutability, type inference, and best practices.

## Variables: The Basics

### Declaration with Type Annotation

Variables in jRust are declared with the `let` keyword with optional type annotations:

```javascript
let x: number = 42;
let name: string = "Alice";
let isActive: boolean = true;
```

**Transpiles to:**
```rust
let mut x: i32 = 42;
let mut name: String = "Alice".to_string();
let mut isActive: bool = true;
```

### Type Inference

jRust supports **type inference** - you can omit the type annotation and let the compiler infer it:

```javascript
let x = 42;           // Inferred as number
let name = "Alice";   // Inferred as string
let active = true;    // Inferred as boolean
```

**Transpiles to:**
```rust
let mut x = 42;
let mut name = "Alice";
let mut active = true;
```

### Default Mutability

In jRust, **all `let` variables are mutable by default**. This means you can reassign them:

```javascript
let count = 0;
count = 5;
count = 10;
print(count);  // Output: 10
```

**Transpiles to:**
```rust
let mut count = 0;
count = 5;
count = 10;
println!("{}", count);
```

### Why Mutable by Default?

Unlike Rust where variables are immutable by default, jRust makes them mutable by default for developer ergonomics. You get easy mutability while still benefiting from **Rust's ownership and borrowing rules** at the compiled level.

## Constants

Constants are declared with the `const` keyword and **must use ALL_CAPS naming**:

```javascript
const MAX_CONNECTIONS: number = 100;
const APP_NAME: string = "MyApp";
const API_URL: string = "https://api.example.com";
const DB_TIMEOUT: number = 30000;
```

**Transpiles to:**

```rust
const MAX_CONNECTIONS: i32 = 100;
const APP_NAME: &str = "MyApp";
const API_URL: &str = "https://api.example.com";
const DB_TIMEOUT: i32 = 30000;
```

### Constants vs Variables

| Feature | Variable (`let`) | Constant (`const`) |
|---------|------------------|-------------------|
| Declaration | `let x = value;` or `let x: type = value;` | `const X: type = value;` |
| Mutability | Mutable (can be reassigned) | Immutable (cannot be reassigned) |
| Naming | camelCase or any style | UPPER_SNAKE_CASE (required) |
| Scope | Block/function scope | Global or module scope |
| Compiled as | `let mut` in Rust | `const` in Rust |
| Type annotation | Optional (inferred) | Required |

### When to Use Constants

Use constants for values that:

- Never change during program execution
- Represent configuration or settings
- Are shared across functions or modules
- Have compile-time known values

```javascript
const API_KEY: string = "sk-12345";
const PORT: number = 8080;
const MAX_RETRIES: number = 3;

function initServer(): void {
    print("Starting server on port: " + PORT);
}
```

### Constant Examples with Transpilation

**Example 1: Numeric Constants**

jRust:

```javascript
const MAX_SIZE: number = 100;
const MIN_SIZE: number = 10;

function validateSize(size: number): void {
    if size > MAX_SIZE {
        print("Too large");
    }
}
```

Generated Rust:

```rust
const MAX_SIZE: i32 = 100;
const MIN_SIZE: i32 = 10;

fn validateSize(size: i32) {
    if size > MAX_SIZE {
        println!("Too large");
    }
}
```

**Example 2: String Constants**

jRust:

```javascript
const GREETING: string = "Hello";
const ERROR_PREFIX: string = "[ERROR]";

function greet(name: string): void {
    print(GREETING + ", " + name);
}
```

Generated Rust:

```rust
const GREETING: &str = "Hello";
const ERROR_PREFIX: &str = "[ERROR]";

fn greet(name: String) {
    println!("{}, {}", GREETING, name);
}
```

Note: `const` strings compile to `&str` (string slices) in Rust for efficiency, while `let` strings compile to `String` (heap-allocated).

## Type Annotations vs Type Inference

### With Type Annotations (Explicit)

```javascript
let x: number = 10;              // ✓ Explicit type
let y: string = "hello";         // ✓ Explicit type
let z: boolean = true;           // ✓ Explicit type
```

### With Type Inference (Implicit)

```javascript
let x = 10;                      // ✓ Inferred as number
let y = "hello";                 // ✓ Inferred as string
let z = true;                    // ✓ Inferred as boolean
```

Both approaches work! Use type annotations when:

- You want explicit documentation
- The type isn't obvious from the value
- You're working with complex types

Use type inference when:

- The type is obvious from the value
- You want concise code
- You're prototyping

## Scoping

Variables follow standard block scoping rules:

```javascript
function outer(): void {
    let x = 10;
    
    {
        let y = 20;
        print(x);    // ✓ Can access outer scope
        print(y);    // ✓ Can access inner scope
    }
    
    // print(y);    // ✗ Error: y is out of scope
}
```

**Transpiles to:**

```rust
fn outer() {
    let mut x = 10;
    
    {
        let mut y = 20;
        println!("{}", x);
        println!("{}", y);
    }
    
    // println!("{}", y); // Error: y is out of scope
}
```

## Variable Shadowing

jRust allows variable shadowing (reusing the same name in nested scopes):

```javascript
function example(): void {
    let x = 10;
    print(x);        // Output: 10
    
    {
        let x = "hello";
        print(x);    // Output: hello
    }
    
    print(x);        // Output: 10
}
```

**Transpiles to:**

```rust
fn example() {
    let mut x = 10;
    println!("{}", x);
    
    {
        let mut x = "hello";
        println!("{}", x);
    }
    
    println!("{}", x);
}
```

## Initialization Requirements

Variables must be initialized when declared:

```javascript
let x: number = 0;           // ✓ Initialized
let name: string = "John";   // ✓ Initialized
let count = 42;              // ✓ Initialized with inference
// let y: number;            // ✗ Error: uninitialized variable
```

## Common Errors

### Error 1: Attempting to Reassign Constants

```javascript
const MAX: number = 100;
MAX = 200;  // ✗ Error: cannot reassign constant
```

**Error message:** Cannot reassign constant value

### Error 2: Lowercase Constant Names

```javascript
const maxSize: number = 100;  // ✗ Error: const must be UPPERCASE
```

**Error message:** Constant names must be UPPER_SNAKE_CASE

**Fix:**

```javascript
const MAX_SIZE: number = 100;  // ✓ Correct
```

### Error 3: Type Mismatch

```javascript
let x: number = 42;
x = "hello";  // ✗ Error: cannot assign string to number
```

**Error message:** Type mismatch: expected number, found string

### Error 4: Missing Initialization

```javascript
let x: number;  // ✗ Error: variable not initialized
print(x);
```

**Error message:** Variable must be initialized when declared

**Fix:**

```javascript
let x: number = 0;  // ✓ Correct
print(x);
```

## Ownership and Mutability

Variables own the values they hold. When a variable goes out of scope, its value is dropped:

```javascript
function example(): void {
    let message = "Hello";
    print(message);    // ✓ message owns the string
}  // message is dropped here
```

**Transpiles to:**

```rust
fn example() {
    let mut message = "Hello";
    println!("{}", message);
}  // message is dropped here
```

For complex types (strings, arrays, objects), jRust handles ownership automatically in the generated Rust code.

## Working with Arrays

jRust supports both dynamic and static arrays:

### Dynamic Arrays (Variable Length)

```javascript
let numbers: number[] = [1, 2, 3, 4, 5];
let names: string[] = ["Alice", "Bob"];
```

**Transpiles to:**

```rust
let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
let mut names: Vec<String> = vec!["Alice".to_string(), "Bob".to_string()];
```

### Static Arrays (Fixed Length)

```javascript
let coords: number[number, 3] = [10, 20, 30];
let rgb: number[number, 3] = [255, 128, 0];
```

**Transpiles to:**

```rust
let mut coords: [i32; 3] = [10, 20, 30];
let mut rgb: [i32; 3] = [255, 128, 0];
```

See **[Arrays](11-arrays.md)** for more details.

## Best Practices

1. **Use descriptive names** — `userName` is better than `u`
   ```javascript
   let userName = "Alice";        // ✓ Good
   let u = "Alice";               // ✗ Bad
   ```

2. **Use constants for fixed values** — Configuration, limits, API keys
   ```javascript
   const MAX_RETRIES: number = 3;  // ✓ Good
   let maxRetries = 3;             // ✗ Less clear intent
   ```

3. **Use type inference for obvious types** — Less verbose
   ```javascript
   let count = 0;                  // ✓ Good (obviously number)
   let count: number = 0;          // ✓ Also fine but verbose
   ```

4. **Use type annotations for clarity** — When type isn't obvious
   ```javascript
   let result: boolean = checkStatus();  // ✓ Good (return type unclear)
   let result = checkStatus();           // ✗ What type is result?
   ```

5. **Keep variables in narrow scopes** — Declare them where needed
   ```javascript
   function process(): void {
       if condition {
           let temp = compute();    // ✓ Good (temp only needed here)
           print(temp);
       }
   }
   ```

6. **Prefer constants for safety** — They prevent accidental mutation
   ```javascript
   const API_URL: string = "https://api.example.com";  // ✓ Cannot be changed
   let apiUrl = "https://api.example.com";             // ✗ Could be changed accidentally
   ```

## Complete Example

Here's a complete example showing variables, constants, type inference, and scoping:

```javascript
const MAX_USERS: number = 100;
const APP_NAME: string = "MyApp";

function processUsers(): void {
    let currentCount = 0;
    let users: string[] = ["Alice", "Bob", "Charlie"];
    
    for user in users {
        currentCount = currentCount + 1;
        
        if currentCount > MAX_USERS {
            print("Maximum users reached");
            break;
        }
        
        let message = APP_NAME + ": Processing " + user;
        print(message);
    }
    
    print("Total processed: " + currentCount);
}
```

**Transpiles to:**

```rust
const MAX_USERS: i32 = 100;
const APP_NAME: &str = "MyApp";

fn processUsers() {
    let mut currentCount = 0;
    let mut users: Vec<String> = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
    
    for user in users {
        currentCount = currentCount + 1;
        
        if currentCount > MAX_USERS {
            println!("Maximum users reached");
            break;
        }
        
        let mut message = format!("{}: Processing {}", APP_NAME, user);
        println!("{}", message);
    }
    
    println!("Total processed: {}", currentCount);
}
```

## Summary

- Variables declared with `let` are **mutable by default**
- **Type annotations are optional** - use type inference when obvious
- Constants declared with `const` are **immutable and always UPPER_CASE**
- Variables **own their values** and follow Rust's ownership rules
- **Shadowing is allowed** in nested scopes
- **Both static and dynamic arrays** are supported

## See Also

- **[Primitive Types](04-primitive-types.md)** - Available data types
- **[Strings](08-strings.md)** - Working with strings
- **[Arrays](11-arrays.md)** - Array operations
- **[Functions](05-functions.md)** - Function declarations
