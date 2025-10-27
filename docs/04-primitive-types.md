# Primitive Types

jRust provides several primitive types for working with basic data. All types must be explicitly annotated.

## number

The `number` type represents numeric values. It transpiles to Rust's `i32` by default:

```typescript
let age: number = 25;
let temperature: number = -10;
let pi: number = 314;  // Note: currently integers only, floats coming soon
```

**Transpilation to Rust:**
```rust
let mut age: i32 = 25;
let mut temperature: i32 = -10;
let mut pi: i32 = 314;
```

### Number Literals

```typescript
let positive: number = 42;
let negative: number = -100;
let zero: number = 0;
```

### Number Operations

```typescript
let x: number = 10;
let y: number = 20;
let sum: number = x + y;           // Addition
let difference: number = y - x;    // Subtraction
let product: number = x * y;       // Multiplication
let quotient: number = y / x;      // Division
let remainder: number = y % x;     // Modulo
```

### Future: Float Support

In future versions:
```typescript
let float_pi: f32 = 3.14;
let double_pi: f64 = 3.14159265;
```

## string

The `string` type represents text. It transpiles to Rust's `String`:

```typescript
let greeting: string = "Hello, World!";
let name: string = "Alice";
let empty: string = "";
```

**Transpilation to Rust:**
```rust
let mut greeting: String = String::from("Hello, World!");
let mut name: String = String::from("Alice");
let mut empty: String = String::from("");
```

### String Literals

```typescript
let simple: string = "Hello";
let withNumbers: string = "Version 1.0";
let withSymbols: string = "Price: $99.99";
```

### Escape Sequences

```typescript
let quoted: string = "He said \"Hello\"";
let newline: string = "Line1\nLine2";
let tab: string = "Column1\tColumn2";
```

### String Concatenation

Use the `+` operator to concatenate strings:

```typescript
let firstName: string = "John";
let lastName: string = "Doe";
let fullName: string = firstName + " " + lastName;
print(fullName);  // Output: John Doe
```

You can also concatenate with numbers (numbers convert to strings):

```typescript
let count: number = 5;
let message: string = "Items: " + count;
print(message);  // Output: Items: 5
```

## boolean

The `boolean` type represents true or false values. It transpiles to Rust's `bool`:

```typescript
let isActive: boolean = true;
let hasErrors: boolean = false;
```

**Transpilation to Rust:**
```rust
let mut isActive: bool = true;
let mut hasErrors: bool = false;
```

### Boolean Literals

```typescript
let t: boolean = true;
let f: boolean = false;
```

### Boolean Operations

```typescript
let a: boolean = true;
let b: boolean = false;

// Logical AND
let and_result: boolean = a && b;  // false

// Logical OR
let or_result: boolean = a || b;   // true

// Logical NOT
let not_a: boolean = !a;           // false
```

### Boolean Comparisons

```typescript
let x: number = 10;
let y: number = 20;

let equal: boolean = x == y;              // false
let notEqual: boolean = x != y;           // true
let lessThan: boolean = x < y;            // true
let lessThanOrEqual: boolean = x <= y;    // true
let greaterThan: boolean = x > y;         // false
let greaterThanOrEqual: boolean = x >= y; // false
```

## void

The `void` type represents the absence of a value. Use it as a function return type when the function doesn't return anything:

```typescript
function greet(name: string): void {
    print("Hello, " + name + "!");
}

function main(): void {
    greet("Alice");
}
```

**Transpilation to Rust:**
```rust
fn greet(name: String) {
    println!("Hello, {}", name);
}

fn main() {
    greet(String::from("Alice"));
}
```

`void` is not used for variables — you can't create a `void` value.

## Type Conversions (Future)

In future versions:

```typescript
let num_str: string = "42";
let num: number = number(num_str);  // String to number

let str: string = string(42);       // Number to string
```

## Type Summary

| Type | Rust | Usage | Example |
|------|------|-------|---------|
| `number` | `i32` | Integers | `let age: number = 25;` |
| `string` | `String` | Text | `let name: string = "Alice";` |
| `boolean` | `bool` | True/false | `let active: boolean = true;` |
| `void` | `()` | No value | `function f(): void {}` |

## Best Practices

1. **Be explicit with types** — Always annotate variable types
2. **Use string concatenation** — For simple text building
3. **Keep numbers as numbers** — Don't convert unnecessarily
4. **Use booleans in conditions** — For control flow

## Complete Example

```typescript
const APP_NAME: string = "Calculator";
const MAX_INPUTS: number = 100;
const DEBUG: boolean = true;

function calculate(a: number, b: number): number {
    return a + b;
}

function main(): void {
    const appLabel: string = APP_NAME + " v1.0";
    const result: number = calculate(10, 20);
    const isValid: boolean = result > 0;
    
    print(appLabel);
    print("Result: " + result);
    print("Valid: " + isValid);
}
```

Next, learn how to write **[Functions](05-functions.md)** to organize your code.
