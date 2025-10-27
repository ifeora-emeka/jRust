# Ownership and Borrowing

Ownership and borrowing are core concepts in jRust, inherited from Rust. They ensure memory safety without garbage collection. This guide explains how they work.

## Ownership

Each value in jRust has one owner. When the owner goes out of scope, the value is cleaned up.

### The Three Ownership Rules

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped (freed)
3. You can transfer ownership to another owner (moving)

### Basic Ownership

```typescript
function example(): void {
    let s: string = "hello";
    print(s);
    // s is dropped here (the string is freed)
}
```

**In Rust:**
```rust
fn example() {
    let s: String = String::from("hello");
    println!("{}", s);
    // s is dropped here
}
```

### Moving Ownership

When you assign a value to another variable, ownership moves:

```typescript
function example(): void {
    let s1: string = "hello";
    let s2: string = s1;
    
    print(s1);  // ✗ Error: s1 no longer owns the string!
    print(s2);  // ✓ s2 owns it now
}
```

This prevents double-freeing (two owners trying to free the same memory).

## References and Borrowing

Instead of moving ownership, you can **borrow** a reference to a value. References let you use a value without taking ownership.

### Immutable References (`&`)

Use `&` to borrow an immutable reference:

```typescript
function printString(s: &string): void {
    print(s);
}

function main(): void {
    let message: string = "Hello";
    printString(&message);
    print(message);  // ✓ message is still valid!
}
```

**Key points:**
- The function borrows `message` (doesn't take ownership)
- The function cannot modify `message`
- Multiple immutable references are allowed

```typescript
function process(s: &string): void {
    print(s);
}

function main(): void {
    let data: string = "test";
    process(&data);
    process(&data);  // ✓ Can borrow multiple times
    process(&data);
}
```

### Mutable References (`&mut`)

Use `&mut` when a function needs to modify a borrowed value:

```typescript
function append(s: &mut string): void {
    s = s + " World";
}

function main(): void {
    let greeting: string = "Hello";
    append(&mut greeting);
    print(greeting);  // Output: Hello World
}
```

**Key points:**
- Only one mutable reference at a time
- Mutable and immutable references cannot coexist
- The original variable can be modified through the reference

### The Borrowing Rules

```typescript
// ✓ Multiple immutable references
let data: string = "test";
let ref1: &string = &data;
let ref2: &string = &data;
let ref3: &string = &data;

// ✓ One mutable reference (alone)
let data: string = "test";
let ref_mut: &mut string = &mut data;

// ✗ Cannot mix immutable and mutable references
let data: string = "test";
let ref_imm: &string = &data;
let ref_mut: &mut string = &mut data;  // Error!
```

## Practical Examples

### Borrowing to Avoid Copies

```typescript
// Without borrowing (moves ownership)
function expensive(s: string): void {
    print(s);
}

function main(): void {
    let data: string = "large data";
    expensive(data);
    print(data);  // ✗ Error: data was moved
}
```

```typescript
// With borrowing (efficient, no move)
function efficient(s: &string): void {
    print(s);
}

function main(): void {
    let data: string = "large data";
    efficient(&data);
    print(data);  // ✓ data still valid
}
```

### Modification Through References

```typescript
function updateString(s: &mut string): void {
    s = s + " updated";
}

function main(): void {
    let message: string = "Original";
    updateString(&mut message);
    print(message);  // Output: Original updated
}
```

### Multiple Borrows

```typescript
function read1(s: &string): void {
    print("Read 1: " + s);
}

function read2(s: &string): void {
    print("Read 2: " + s);
}

function main(): void {
    let text: string = "Hello";
    read1(&text);
    read2(&text);
    print(text);  // ✓ All work fine
}
```

## Transpilation to Rust

**jRust:**
```typescript
function processData(data: &string): void {
    print("Processing: " + data);
}

function modifyData(data: &mut string): void {
    data = data + " modified";
}

function main(): void {
    let info: string = "original";
    processData(&info);
    modifyData(&mut info);
    print(info);
}
```

**Generated Rust:**
```rust
fn processData(data: &String) {
    println!("Processing: {}", data);
}

fn modifyData(data: &mut String) {
    *data = data.clone() + " modified";
}

fn main() {
    let mut info: String = String::from("original");
    processData(&info);
    modifyData(&mut info);
    println!("{}", info);
}
```

## Ownership vs Borrowing: When to Use What

| Scenario | Use |
|----------|-----|
| Function just reads a value | `&T` (immutable reference) |
| Function modifies a value | `&mut T` (mutable reference) |
| Function takes full ownership | `T` (move ownership) |
| You're done with the value | Move (transfer ownership) |
| You need the value later | Borrow (keep ownership) |

## Common Patterns

### Reading Without Moving

```typescript
function printLength(s: &string): void {
    print("Length check");
}

function main(): void {
    let text: string = "Hello";
    printLength(&text);
    printLength(&text);  // ✓ Can use multiple times
}
```

### Modifying and Returning

```typescript
function processData(data: &mut string): void {
    data = data + " processed";
}

function main(): void {
    let info: string = "raw data";
    processData(&mut info);
    print(info);  // Shows processed version
}
```

## Best Practices

1. **Default to borrowing** — Use `&T` and `&mut T` instead of moving
2. **Be explicit about mutability** — Use `&mut` only when needed
3. **Keep borrows short** — Release them as soon as possible
4. **Avoid deep nesting of references** — Keep one level deep usually

## Memory Safety Guaranteed

By enforcing these rules, jRust guarantees:
- ✓ No use-after-free
- ✓ No double-free
- ✓ No data races
- ✓ No null pointer dereferences
- ✓ Memory safety at compile time

All without garbage collection!

Next, learn about **[Strings](08-strings.md)** and string manipulation.
