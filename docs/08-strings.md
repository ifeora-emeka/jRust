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

# String Methods in jRust

jRust provides comprehensive string manipulation methods that compile to efficient Rust string operations. These methods are familiar to JavaScript/TypeScript developers while leveraging Rust's string handling.

## String Creation

```typescript
let message = "Hello, World!";
let name = "Alice";
let multiline = "Line 1
Line 2
Line 3";
```

## String Properties

### length Property

```typescript
let text = "Hello";
let len = text.length;
```

**Rust Output:**
```rust
let mut len = text.len() as i32;
```

## Case Conversion

### toUpperCase() - Convert to Uppercase

```typescript
let upper = message.toUpperCase();
```

**Rust Output:**
```rust
let mut upper = message.to_uppercase();
```

### toLowerCase() - Convert to Lowercase

```typescript
let lower = message.toLowerCase();
```

**Rust Output:**
```rust
let mut lower = message.to_lowercase();
```

## Substring Operations

### substring(start, end?) - Extract Substring

```typescript
let sub = text.substring(0, 5);
let tail = text.substring(7);
```

**Rust Output:**
```rust
let mut sub = text.chars().skip(0 as usize).take((5 - 0) as usize).collect::<String>();
let mut tail = text.chars().skip(7 as usize).take(usize::MAX).collect::<String>();
```

### charAt(index) - Get Character at Position

```typescript
let ch = text.charAt(0);
```

**Rust Output:**
```rust
let mut ch = text.chars().nth(0 as usize).unwrap_or('\0');
```

## Searching Methods

### indexOf(substring) - Find Position

```typescript
let pos = text.indexOf("World");
```

**Rust Output:**
```rust
let mut pos = text.find("World").map(|i| i as i32).unwrap_or(-1);
```

Returns -1 if not found, otherwise the index position.

## Trimming Methods

### trim() - Remove Whitespace

```typescript
let trimmed = "  hello  ".trim();
```

**Rust Output:**
```rust
let mut trimmed = "  hello  ".trim().to_string();
```

## String Splitting and Joining

### split(delimiter) - Split into Array

```typescript
let words = "apple,banana,orange".split(",");
```

**Rust Output:**
```rust
let mut words = "apple,banana,orange".split(",").map(|s| s.to_string()).collect::<Vec<String>>();
```

### join(delimiter) - Join Array Elements

```typescript
let arr = ["a", "b", "c"];
let joined = arr.join("-");
```

**Rust Output:**
```rust
let mut joined = arr.join("-");
```

## Checking Methods

### contains(substring) - Check if Contains

```typescript
let hasWorld = text.contains("World");
```

**Rust Output:**
```rust
let mut hasWorld = text.contains("World");
```

## Complete Examples

### Text Processing

```typescript
let input = "  Hello, World!  ";
let cleaned = input.trim();
let upper = cleaned.toUpperCase();
let lower = cleaned.toLowerCase();

print(upper);
print(lower);

let len = cleaned.length;
print("Length: ");
print(len);
```

### String Analysis

```typescript
let sentence = "The quick brown fox jumps";
let words = sentence.split(" ");
let wordCount = words.length;

print("Word count: ");
print(wordCount);

if sentence.contains("fox") {
    print("Found fox!");
}
```

### String Manipulation

```typescript
let original = "JavaScript";
let first = original.charAt(0);
let sub = original.substring(0, 4);
let upper = sub.toUpperCase();

print(upper);
```

### Search and Extract

```typescript
let email = "user@example.com";
let atPos = email.indexOf("@");

if atPos > 0 {
    let username = email.substring(0, atPos);
    print("Username: ");
    print(username);
}
```

## Method Reference Table

| Method | Description | Returns | Example |
|--------|-------------|---------|---------|
| `length` | String length | number | `"hello".length` → 5 |
| `toUpperCase()` | Convert to uppercase | string | `"hello".toUpperCase()` → "HELLO" |
| `toLowerCase()` | Convert to lowercase | string | `"HELLO".toLowerCase()` → "hello" |
| `substring(start, end?)` | Extract substring | string | `"hello".substring(0, 2)` → "he" |
| `charAt(index)` | Get character | string | `"hello".charAt(0)` → "h" |
| `indexOf(substring)` | Find position | number | `"hello".indexOf("ll")` → 2 |
| `trim()` | Remove whitespace | string | `" hi ".trim()` → "hi" |
| `split(delimiter)` | Split into array | string[] | `"a,b".split(",")` → ["a", "b"] |
| `contains(substring)` | Check if contains | boolean | `"hello".contains("ll")` → true |

## Advanced String Operations

### String Concatenation

```typescript
let first = "Hello";
let second = "World";
let combined = first + " " + second;
```

### Template-like Strings

```typescript
let name = "Alice";
let age = 25;
let message = "Name: " + name + ", Age: " + age;
print(message);
```

## Best Practices

1. **Use `length` for string size** - Don't manually count characters
2. **Check with `indexOf()` before extracting** - Avoid out-of-bounds errors
3. **Use `trim()` on user input** - Remove unwanted whitespace
4. **Prefer `contains()` for existence checks** - More readable than indexOf
5. **Be mindful of Unicode** - Rust handles Unicode correctly

## Performance Notes

- String concatenation with `+` creates new strings
- `charAt()` and `substring()` work with Unicode characters
- `indexOf()` returns -1 if not found (check before using result)
- `split()` creates a new array
- Case conversion creates new strings

## Common Patterns

### Email Validation (Basic)

```typescript
let email = "user@example.com";
if email.contains("@") {
    let atPos = email.indexOf("@");
    if atPos > 0 {
        print("Valid email format");
    }
}
```

### Text Formatting

```typescript
let input = "  mixed CASE text  ";
let formatted = input.trim().toLowerCase();
print(formatted);
```

### Word Processing

```typescript
let text = "one two three";
let words = text.split(" ");
for word in words {
    let capitalized = word.charAt(0).toUpperCase() + word.substring(1);
    print(capitalized);
}
```

## See Also

- [Arrays Guide](11-arrays.md) - Array operations
- [Array Methods](15-array-methods.md) - Array manipulation
- [Variables](03-variables.md) - Variable declarations
- [Primitive Types](04-primitive-types.md) - Basic types
