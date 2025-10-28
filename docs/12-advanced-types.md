# Advanced Types in jRust

jRust supports advanced type features including custom structs, enumerations, and type inference.

## Structs (Records)

Structs allow you to create custom data types with named fields.

### Syntax

```typescript
struct StructName {
    fieldName: type,
    anotherField: type
}
```

### Example

```typescript
struct Point {
    x: number,
    y: number
}

struct User {
    name: string,
    age: number,
    active: boolean
}

let p = Point { x: 10, y: 20 };
let user = User { name: "Alice", age: 30, active: true };

print(p.x);
print(user.name);
```

### Rust Output

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: i32,
    active: bool,
}
```

## Enumerations

Enums define a type that can be one of several variants.

### Syntax

```typescript
enum EnumName {
    VariantOne,
    VariantTwo,
    VariantWithData(type)
}
```

### Example

```typescript
enum Status {
    Active,
    Inactive,
    Pending(string)
}

enum Color {
    Red,
    Green,
    Blue,
    Custom(number, number, number)
}
```

### Rust Output

```rust
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(i32, i32, i32),
}
```

## Type Inference

jRust supports automatic type detection, allowing you to omit type annotations when the type can be inferred from the value.

### Without Type Inference

```typescript
let x: number = 42;
let name: string = "Alice";
let active: boolean = true;
```

### With Type Inference

```typescript
let x = 42;
let name = "Alice";
let active = true;
```

Both produce the same Rust code:

```rust
let mut x = 42;
let mut name = "Alice".to_string();
let mut active = true;
```

### When to Use Type Inference

✅ **Use type inference when:**
- The type is obvious from the value
- You're initializing with a literal value
- The code remains clear and readable

❌ **Avoid type inference when:**
- Function parameters (required for clarity)
- Return types (required for clarity)
- The type might not be obvious to readers
- Working with `any` type

### Example

```typescript
let age = 25;
let name = "Bob";
let scores = [95, 87, 92];

function greet(name: string): void {
    print("Hello, " + name);
}
```

## Custom Types

You can use your custom struct and enum names as types:

```typescript
struct Point {
    x: number,
    y: number
}

function distance(p1: Point, p2: Point): number {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    return dx * dx + dy * dy;
}

let p1 = Point { x: 0, y: 0 };
let p2 = Point { x: 3, y: 4 };
let dist = distance(p1, p2);
```

## Best Practices

1. **Use descriptive names** for structs and enums
2. **Group related data** in structs
3. **Use enums** for types with a fixed set of values
4. **Leverage type inference** for cleaner code
5. **Add type annotations** when clarity is needed

## Examples

### Complete Struct Example

```typescript
struct Rectangle {
    width: number,
    height: number
}

function area(rect: Rectangle): number {
    return rect.width * rect.height;
}

let r = Rectangle { width: 10, height: 20 };
let a = area(r);
print(a);
```

### Complete Enum Example

```typescript
enum HttpStatus {
    Ok,
    NotFound,
    Error(string)
}

let status = HttpStatus.Ok;
```

## See Also

- [Variables](03-variables.md) - Variable declarations
- [Functions](05-functions.md) - Function definitions
- [Primitive Types](04-primitive-types.md) - Basic types
