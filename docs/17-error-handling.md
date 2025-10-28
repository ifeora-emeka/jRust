# Error Handling in jRust

jRust provides try/catch blocks for error handling, similar to JavaScript/TypeScript, which compile to Rust's Result-based error handling.

## Basic Try/Catch

### Syntax

```typescript
try {
    // Code that might fail
} catch (error) {
    // Handle the error
}
```

### Example

```typescript
try {
    print("Attempting operation...");
    throw "Something went wrong!";
    print("This won't execute");
} catch (e) {
    print("Error caught: ");
    print(e);
}
```

### Rust Output

```rust
match (|| -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Attempting operation...");
    panic!("{}", "Something went wrong!");
    println!("{}", "This won't execute");
    Ok(())
})() {
    Ok(_) => {},
    Err(e) => {
        println!("{}", "Error caught: ");
        println!("{}", e);
    }
}
```

## Throwing Errors

### throw Statement

```typescript
throw "Error message";
throw "File not found";
```

**Rust Output:**
```rust
panic!("{}", "Error message");
panic!("{}", "File not found");
```

## Catch Parameter

### With Named Parameter

```typescript
try {
    throw "Operation failed";
} catch (error) {
    print(error);
}
```

### Without Parameter

```typescript
try {
    throw "Some error";
} catch {
    print("An error occurred");
}
```

## Complete Examples

### File Operation Simulation

```typescript
function readFile(filename: string): string {
    if filename == "missing.txt" {
        throw "File not found";
    }
    return "File contents";
}

try {
    let content = readFile("missing.txt");
    print(content);
} catch (e) {
    print("Error reading file: ");
    print(e);
}
```

### Validation with Error Handling

```typescript
function validateAge(age: number): void {
    if age < 0 {
        throw "Age cannot be negative";
    }
    if age > 150 {
        throw "Age seems unrealistic";
    }
    print("Age is valid");
}

let userAge = 25;

try {
    validateAge(userAge);
    print("Validation passed");
} catch (error) {
    print("Validation failed: ");
    print(error);
}
```

### Multiple Operations with Recovery

```typescript
let results = [0, 0, 0];
let index = 0;

function processValue(val: number): number {
    if val < 0 {
        throw "Negative value not allowed";
    }
    return val * 2;
}

for num in [5, -10, 15] {
    try {
        let result = processValue(num);
        results[index] = result;
        print("Processed: ");
        print(result);
    } catch (e) {
        print("Skipped invalid value");
        results[index] = 0;
    }
    let index = index + 1;
}
```

### Nested Try/Catch

```typescript
try {
    print("Outer try block");
    
    try {
        print("Inner try block");
        throw "Inner error";
    } catch (inner) {
        print("Caught inner: ");
        print(inner);
        throw "Propagated error";
    }
} catch (outer) {
    print("Caught outer: ");
    print(outer);
}
```

## Error Handling Patterns

### Graceful Degradation

```typescript
function loadData(): string {
    let data = "";
    
    try {
        data = fetchFromServer();
    } catch (e) {
        print("Server unavailable, using cache");
        data = loadFromCache();
    }
    
    return data;
}
```

### Retry Logic

```typescript
let attempts = 0;
let success = false;

while attempts < 3 && !success {
    try {
        performOperation();
        success = true;
        print("Operation succeeded");
    } catch (e) {
        let attempts = attempts + 1;
        print("Attempt failed, retrying...");
    }
}

if !success {
    print("Operation failed after 3 attempts");
}
```

### Validation Chain

```typescript
function validateUser(name: string, age: number): void {
    if name.length == 0 {
        throw "Name is required";
    }
    if age < 18 {
        throw "Must be 18 or older";
    }
    print("User is valid");
}

try {
    validateUser("", 20);
} catch (e) {
    print("Validation error: ");
    print(e);
}
```

## Best Practices

1. **Catch specific errors** - Use descriptive error messages
2. **Don't catch silently** - Always log or handle errors appropriately
3. **Clean up resources** - Ensure cleanup happens even on error
4. **Fail fast** - Throw errors early when validation fails
5. **Provide context** - Include relevant information in error messages

## Error Message Guidelines

### Good Error Messages

```typescript
throw "Invalid email format: expected name@domain.com";
throw "File not found: config.json";
throw "Array index out of bounds: index 10, length 5";
```

### Avoid

```typescript
throw "Error";
throw "Failed";
throw "Something went wrong";
```

## Common Patterns

### Input Validation

```typescript
function divide(a: number, b: number): number {
    if b == 0 {
        throw "Division by zero";
    }
    return a / b;
}

try {
    let result = divide(10, 0);
    print(result);
} catch (e) {
    print("Math error: ");
    print(e);
}
```

### Resource Management

```typescript
try {
    openConnection();
    performWork();
} catch (e) {
    print("Work failed: ");
    print(e);
}
closeConnection();
```

### Optional Operations

```typescript
let optionalData = "";

try {
    optionalData = loadOptionalFeature();
} catch (e) {
    print("Optional feature not available");
    optionalData = "";
}

continueWithWork(optionalData);
```

## Limitations

Current implementation:
- Error types are always strings
- No finally block (coming in future phases)
- No custom error types yet
- No error chaining

Future enhancements will include:
- Custom error structs
- Finally blocks
- Error type hierarchies
- Async error handling

## Comparison with Rust

jRust's try/catch compiles to Rust patterns:

**jRust:**
```typescript
try {
    doSomething();
} catch (e) {
    handleError(e);
}
```

**Rust:**
```rust
match (|| -> Result<(), Box<dyn std::error::Error>> {
    doSomething();
    Ok(())
})() {
    Ok(_) => {},
    Err(e) => {
        handleError(e);
    }
}
```

## See Also

- [Functions](05-functions.md) - Function definitions
- [Control Flow](10-control-flow.md) - If statements and loops
- [Advanced Types](12-advanced-types.md) - Custom types
