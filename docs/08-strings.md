# Strings

Strings are a fundamental data type in jRust. This guide covers string creation, manipulation, and concatenation.

## String Basics

### Creating Strings

Declare a string with the `string` type:

```typescript
let greeting: string = "Hello, jRust!";
let name: string = "Alice";
let empty: string = "";
```

### String Literals

Use double quotes for string literals:

```typescript
let simple: string = "Hello";
let withNumbers: string = "Version 1.0";
let withSpaces: string = "Hello, World!";
```

## String Concatenation

### Using the + Operator

Concatenate strings with the `+` operator:

```typescript
let firstName: string = "John";
let lastName: string = "Doe";
let fullName: string = firstName + " " + lastName;
print(fullName);  // Output: John Doe
```

### Concatenating with Numbers

Numbers are automatically converted to strings:

```typescript
let count: number = 5;
let message: string = "You have " + count + " items";
print(message);  // Output: You have 5 items
```

### Building Complex Strings

```typescript
const APP_NAME: string = "Calculator";
const VERSION: number = 1;

let info: string = APP_NAME + " v" + VERSION;
print(info);  // Output: Calculator v1
```

### Multi-Step Concatenation

```typescript
let result: string = "Hello";
result = result + ", ";
result = result + "World";
result = result + "!";
print(result);  // Output: Hello, World!
```

## Escape Sequences

Use escape sequences to include special characters:

```typescript
let quoted: string = "She said \"Hello\"";
let newline: string = "Line 1\nLine 2";
let tab: string = "Column1\tColumn2";
let backslash: string = "Path: C:\\Users\\Alice";
```

### Common Escape Sequences

| Sequence | Meaning |
|----------|---------|
| `\"` | Double quote |
| `\\` | Backslash |
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |

## String Examples

### Logging with String Concatenation

```typescript
function greet(name: &string): void {
    let message: string = "Hello, " + name + "!";
    print(message);
}

function main(): void {
    greet("Alice");      // Output: Hello, Alice!
    greet("Bob");        // Output: Hello, Bob!
}
```

### Building Dynamic Messages

```typescript
const ERROR_PREFIX: string = "[ERROR] ";
const WARNING_PREFIX: string = "[WARNING] ";

function logError(message: &string): void {
    print(ERROR_PREFIX + message);
}

function logWarning(message: &string): void {
    print(WARNING_PREFIX + message);
}

function main(): void {
    logError("Database connection failed");
    logWarning("Retry attempt 1 of 3");
}
```

### String Building with Multiple Concatenations

```typescript
function buildReport(name: &string, status: &string, count: number): string {
    let report: string = "Report: ";
    report = report + name;
    report = report + " | Status: ";
    report = report + status;
    report = report + " | Count: ";
    report = report + count;
    return report;
}

function main(): void {
    let result: string = buildReport("Task1", "Complete", 42);
    print(result);
}
```

## String Borrowing

Use references to pass strings efficiently:

```typescript
function display(msg: &string): void {
    print(msg);
}

function main(): void {
    let text: string = "Important data";
    display(&text);
    display(&text);  // ✓ Can use multiple times
}
```

## Mutable String Operations

Modify strings through mutable references:

```typescript
function appendSuffix(s: &mut string): void {
    s = s + " [processed]";
}

function main(): void {
    let status: string = "In Progress";
    appendSuffix(&mut status);
    print(status);  // Output: In Progress [processed]
}
```

## Transpilation to Rust

**jRust:**
```typescript
const GREETING: string = "Hello";

function greetUser(name: &string): void {
    let message: string = GREETING + ", " + name + "!";
    print(message);
}

function main(): void {
    greetUser("Alice");
}
```

**Generated Rust:**
```rust
const GREETING: &str = "Hello";

fn greetUser(name: &String) {
    let mut message: String = format!("{}, {}!", GREETING, name);
    println!("{}", message);
}

fn main() {
    greetUser(&String::from("Alice"));
}
```

## String Best Practices

1. **Use concatenation for simple cases** — Clear and readable
   ```typescript
   let message: string = "Error: " + error + " at line " + line;
   ```

2. **Pass strings by reference** — Avoid unnecessary copies
   ```typescript
   function process(s: &string): void { }  // ✓ Efficient
   function process(s: string): void { }   // Slower
   ```

3. **Use constants for repeated strings** — Avoid duplication
   ```typescript
   const APP_NAME: string = "MyApp";
   const LOG_PREFIX: string = "[LOG] ";
   ```

4. **Escape special characters** — When needed in literals
   ```typescript
   let path: string = "C:\\Users\\Data\\file.txt";
   ```

5. **Keep mutable strings local** — Don't pass mutability when not needed
   ```typescript
   function modify(s: &mut string): void { }  // Only when necessary
   ```

## Complete Example

```typescript
const APP_NAME: string = "Logger";
const ERROR_TAG: string = "[ERROR]";
const INFO_TAG: string = "[INFO]";

function formatLog(tag: &string, message: &string): string {
    return tag + " " + message;
}

function logError(message: &string): void {
    let formatted: string = formatLog(&ERROR_TAG, message);
    print(formatted);
}

function logInfo(message: &string): void {
    let formatted: string = formatLog(&INFO_TAG, message);
    print(formatted);
}

function main(): void {
    let appStart: string = APP_NAME + " started";
    logInfo(&appStart);
    
    let errorMsg: string = "Connection timeout";
    logError(&errorMsg);
}
```

## Coming Soon

Future versions will support:
- String methods (length, substring, indexOf, etc.)
- String interpolation (template literals)
- Character operations
- UTF-8 validation

Next, learn about **[Output and Transpilation](09-output.md)**.
