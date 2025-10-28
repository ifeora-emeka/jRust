# Advanced Topics

This document covers features that are planned for upcoming phases of jRust development.

## Planned Features

### Type Inference

Automatic type detection so you don't always need explicit type annotations:

```typescript
let x = 42;                 // Inferred as number
let name = "Alice";         // Inferred as string
let active = true;          // Inferred as boolean
let arr = [1, 2, 3];       // Inferred as number[]
```

**Status:** Planning complete, ready for implementation

### String Methods

Direct string manipulation with familiar syntax:

```typescript
let s: string = "Hello World";

let upper: string = s.toUpperCase();    // HELLO WORLD
let lower: string = s.toLowerCase();    // hello world
let len: number = s.length;             // 11
let index: number = s.indexOf("World"); // 6
let has: boolean = s.contains("World"); // true
let sub: string = s.substring(0, 5);    // Hello

let parts: string[] = s.split(" ");     // ["Hello", "World"]
let replaced: string = s.replace("World", "Rust");
```

**Available Methods:**
- `length` — String length
- `toUpperCase()` — Convert to uppercase
- `toLowerCase()` — Convert to lowercase
- `trim()` — Remove whitespace
- `substring(start, end)` — Extract substring
- `indexOf(search)` — Find first occurrence
- `contains(search)` — Check if contains
- `startsWith(prefix)` — Check prefix
- `endsWith(suffix)` — Check suffix
- `split(delimiter)` — Split into array
- `replace(search, replacement)` — Replace text

**Status:** Design complete, implementation phase ready

### Array Methods

Functional programming support with familiar array methods:

```typescript
let numbers: number[] = [1, 2, 3, 4, 5];

let doubled: number[] = numbers.map(x => x * 2);        // [2, 4, 6, 8, 10]
let evens: number[] = numbers.filter(x => x % 2 == 0); // [2, 4]
let sum: number = numbers.reduce((a, b) => a + b, 0);  // 15

let found: number = numbers.find(x => x > 3);          // 4
let has: boolean = numbers.contains(3);                // true

let reversed: number[] = numbers.reverse();            // [5, 4, 3, 2, 1]
let sliced: number[] = numbers.slice(1, 4);           // [2, 3, 4]
let joined: string = numbers.join(", ");              // "1, 2, 3, 4, 5"
```

**Available Methods:**
- `length` — Array length
- `push(element)` — Add element
- `pop()` — Remove last element
- `map(callback)` — Transform elements
- `filter(predicate)` — Select elements
- `reduce(callback, initial)` — Reduce to single value
- `find(predicate)` — Find first match
- `contains(element)` — Check if contains
- `slice(start, end)` — Extract subset
- `reverse()` — Reverse order
- `join(separator)` — Join to string

**Requires:** Closure/lambda support

**Status:** Design complete, requires closure implementation

### Closures and Lambda Expressions

Anonymous functions for use with array methods:

```typescript
let numbers: number[] = [1, 2, 3, 4, 5];

numbers.map(x => x * 2);
numbers.filter(x => x > 2);
numbers.reduce((acc, x) => acc + x, 0);

let add: (number, number) => number = (a, b) => a + b;
let result: number = add(5, 3);
```

**Syntax Options:**
```typescript
(x) => x * 2              // Arrow function
|x| x * 2                 // Rust pipe style
(x: number) => x * 2      // With type annotation
```

**Status:** Design phase

### Struct Types

Define custom data structures:

```typescript
struct Point {
    x: number,
    y: number
}

struct User {
    name: string,
    age: number,
    email: string
}

let p: Point = {x: 10, y: 20};
print(p.x);      // 10

let u: User = {name: "Alice", age: 30, email: "alice@example.com"};
print(u.name);   // Alice

p.x = 15;        // Mutate field
```

**Features:**
- Field access with dot notation
- Field mutation
- Type checking
- Struct as function parameters
- Struct literals with field shorthand

**Status:** Design complete, implementation ready

### Enumerations

Define enumerated types:

```typescript
enum Color {
    Red,
    Green,
    Blue
}

enum Status {
    Active,
    Inactive,
    Pending
}

let color: Color = Color.Red;
let status: Status = Status.Active;

if (color == Color.Red) {
    print("It's red!");
}
```

**Features:**
- Enum variant definitions
- Enum value creation
- Enum comparisons
- Type checking

**Status:** Design complete, implementation ready

### Error Handling

try/catch blocks for error management:

```typescript
try {
    let value: number = parseInt("123");
    print("Parsed: " + value);
} catch (e) {
    print("Error: " + e);
}

function divide(a: number, b: number): number {
    if (b == 0) {
        throw "Division by zero";
    }
    return a / b;
}

try {
    let result: number = divide(10, 0);
} catch (error) {
    print("Caught: " + error);
}
```

**Features:**
- try blocks for risky code
- catch blocks for error handling
- throw statements for raising errors
- Error message capture
- Multiple catch blocks (future)

**Status:** Design complete, implementation ready

## Feature Roadmap

### Phase 3.2 (Upcoming)
- Type Inference
- String Methods (11 methods)
- Array Methods (requires closures)

### Phase 3.3
- Struct Types
- Enumerations
- Error Handling

### Phase 4 (Future)
- Pattern Matching
- Advanced Pattern Matching
- Trait System
- Generics
- Lifetime Management

### Phase 5 (Future)
- Module System Improvements
- Async/Await
- Macro System
- FFI Enhancements

## Implementation Status

| Feature | Status | Complexity | Effort |
|---------|--------|-----------|--------|
| Type Inference | Ready | Low | 2-3h |
| String Methods | Ready | Low | 3-4h |
| Array Methods | Ready | Medium | 4-5h |
| Closures | Planning | Medium | 3-4h |
| Struct Types | Ready | High | 5-6h |
| Enumerations | Ready | High | 3-4h |
| Error Handling | Ready | Medium | 3-4h |

## How to Use These Features (When Available)

### String Manipulation

```typescript
function formatName(first: string, last: string): string {
    let full: string = first + " " + last;
    return full.toUpperCase();
}

let name: string = formatName("john", "doe");
print(name);  // JOHN DOE
```

### Data Processing

```typescript
function analyzeNumbers(nums: number[]): number {
    return nums
        .filter(n => n > 5)
        .map(n => n * 2)
        .reduce((a, b) => a + b, 0);
}

let data: number[] = [1, 6, 3, 8, 4, 9];
let total: number = analyzeNumbers(data);
print("Total: " + total);
```

### Struct Usage

```typescript
struct Config {
    host: string,
    port: number,
    debug: boolean
}

function initServer(config: Config): void {
    print("Server at " + config.host + ":" + config.port);
    if (config.debug) {
        print("Debug mode enabled");
    }
}

let cfg: Config = {host: "localhost", port: 8080, debug: true};
initServer(cfg);
```

### Error Handling

```typescript
function safeProcess(data: string): void {
    try {
        let value: number = parseInt(data);
        process(value);
    } catch (e) {
        print.error("Failed to process: " + e);
    }
}
```

## Learning Path

1. **Master Current Features**
   - Variables and constants
   - Primitive types
   - Functions and parameters
   - Control flow (if/else, loops)
   - Arrays
   - Ownership and borrowing
   - Strings (basic concatenation)

2. **Learn Type Inference** (Phase 3.2)
   - Reduces type annotation boilerplate
   - Makes code more concise

3. **Master String Methods** (Phase 3.2)
   - Text manipulation
   - String searching and replacing

4. **Learn Array Methods** (Phase 3.2)
   - Functional programming
   - Data transformation

5. **Build Custom Types** (Phase 3.3)
   - Struct types for data organization
   - Enumerations for state management

6. **Handle Errors** (Phase 3.3)
   - Safe error recovery
   - Robust applications

## Best Practices (Preview)

### When Using String Methods
```typescript
let s: string = "Hello World";
let cleaned: string = s.trim().toLowerCase();
print(cleaned);  // hello world
```

### When Using Array Methods
```typescript
let data: number[] = [1, 2, 3, 4, 5];
let result: number[] = data
    .filter(x => x % 2 == 0)
    .map(x => x * 2);
print(result);  // [4, 8]
```

### When Using Structs
```typescript
struct Person {
    name: string,
    age: number
}

function celebrateBirthday(person: &mut Person): void {
    person.age = person.age + 1;
    print(person.name + " is now " + person.age);
}
```

### When Using Error Handling
```typescript
function safeDivide(a: number, b: number): void {
    try {
        if (b == 0) {
            throw "Cannot divide by zero";
        }
        let result: number = a / b;
        print("Result: " + result);
    } catch (error) {
        print("Error: " + error);
    }
}
```

## Stay Updated

Watch the project repository for updates on these features:
- GitHub: https://github.com/ifeora-emeka/jRust
- Check CHANGELOG for feature announcements
- Review documentation for new features

## Questions and Discussion

For questions about upcoming features:
- Check the GitHub issues
- Review project roadmap
- See CONTRIBUTING guidelines

---

**Next Steps:**

When these features become available, check back for updated documentation and examples.

For now, master the current features and build great applications!

