# jRust Variable Semantics

## Variable Declaration Rules

jRust enforces strong immutability and type safety at compile time.

### const - Immutable Constants

- Must be declared with UPPERCASE naming convention
- Immutable for the entire program lifetime
- Value is fixed at compile time
- Type cannot be changed

```typescript
const MAX_SIZE: number = 100;
const API_KEY: string = "secret_key";
const DEBUG_MODE: boolean = true;
```

Transpiles to Rust:
```rust
const MAX_SIZE: i32 = 100;
const API_KEY: String = "secret_key";
const DEBUG_MODE: bool = true;
```

### let - Mutable Variables

- Can use any naming convention (camelCase, snake_case, etc.)
- Mutable - value can be changed after initialization
- Type is fixed at declaration - cannot be changed
- Shadowing is not allowed in the current MVP

```typescript
let count: number = 0;
let name: string = "Alice";
let active: boolean = true;

count = 5;
name = "Bob";
```

Transpiles to Rust:
```rust
let mut count: i32 = 0;
let mut name: String = "Alice";
let mut active: bool = true;

count = 5;
name = "Bob";
```

## Type Safety

Once a variable is declared with a type, it cannot be changed:

```typescript
let x: number = 42;
x = "hello";  // ❌ ERROR: Type mismatch (number vs string)
```

## No `mut` Keyword

The `mut` keyword is **not supported** in jRust. Mutability is determined by the declaration style:

- `const` = immutable
- `let` = mutable

## Best Practices

1. **Use `const` for values that never change** - promotes clarity and safety
2. **Use UPPERCASE for const names** - makes immutability obvious in code
3. **Use lowercase for let variables** - keeps them visually distinct
4. **Initialize on declaration** - all variables must be initialized when declared

## Summary

| Feature | const | let |
|---------|-------|-----|
| Declaration | `const NAME` | `let name` |
| Mutability | Immutable ✅ | Mutable ✅ |
| Type Change | ❌ Not allowed | ❌ Not allowed |
| Naming | UPPERCASE | Any |
| Value at compile time | Required | Optional |
| Use case | Configuration, constants | State, counters |
