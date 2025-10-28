# Array Methods in jRust

jRust provides a rich set of array methods that compile to efficient Rust code. These methods follow JavaScript/TypeScript conventions but leverage Rust's powerful Vec operations.

## Array Creation

```typescript
let numbers = [1, 2, 3, 4, 5];
let names = ["Alice", "Bob", "Charlie"];
let empty: number[] = [];
```

## Accessing Elements

### Index Access

```typescript
let first = numbers[0];
let last = numbers[4];
```

**Rust Output:**
```rust
let mut first = numbers[0 as usize];
let mut last = numbers[4 as usize];
```

### Length Property

```typescript
let len = numbers.length;
```

**Rust Output:**
```rust
let mut len = numbers.len() as i32;
```

## Mutating Methods

### push() - Add Element to End

```typescript
let nums = [1, 2, 3];
nums.push(4);
```

**Rust Output:**
```rust
nums.push(4);
```

### pop() - Remove Last Element

```typescript
let last = nums.pop();
```

**Rust Output:**
```rust
let mut last = nums.pop();
```

### unshift() - Add Element to Beginning

```typescript
nums.unshift(0);
```

**Rust Output:**
```rust
nums.insert(0, 0);
```

### shift() - Remove First Element

```typescript
let first = nums.shift();
```

**Rust Output:**
```rust
let mut first = nums.remove(0);
```

## Non-Mutating Methods

### slice() - Extract Portion

```typescript
let sub = numbers.slice(1, 3);
let tail = numbers.slice(2);
```

**Rust Output:**
```rust
let mut sub = numbers[1 as usize..3 as usize].to_vec();
let mut tail = numbers[2 as usize..].to_vec();
```

### reverse() - Reverse Array

```typescript
let reversed = numbers.reverse();
```

**Rust Output:**
```rust
let mut reversed = numbers.iter().rev().cloned().collect::<Vec<_>>();
```

### sort() - Sort Array

```typescript
nums.sort();
```

**Rust Output:**
```rust
nums.sort();
```

## Searching Methods

### contains() / includes() - Check if Element Exists

```typescript
let hasThree = numbers.contains(3);
let includesFive = numbers.includes(5);
```

**Rust Output:**
```rust
let mut hasThree = numbers.contains(&3);
let mut includesFive = numbers.contains(&5);
```

## Higher-Order Methods

### map() - Transform Elements

```typescript
let doubled = numbers.map(x => x * 2);
```

**Note:** Full lambda support coming in Phase 5. Currently requires function references.

### filter() - Select Elements

```typescript
let evens = numbers.filter(x => x % 2 == 0);
```

**Note:** Full lambda support coming in Phase 5. Currently requires function references.

## Complete Examples

### Working with Numbers

```typescript
let scores = [85, 92, 78, 95, 88];

let total = 0;
for score in scores {
    total = total + score;
}

let firstTwo = scores.slice(0, 2);
let len = scores.length;

print("Total scores: ");
print(total);
print("First two: ");
print(firstTwo[0]);
```

### Array Manipulation

```typescript
let fruits = ["apple", "banana", "orange"];

fruits.push("grape");
fruits.unshift("mango");

let last = fruits.pop();
let first = fruits.shift();

print(fruits.length);
```

### Searching and Checking

```typescript
let numbers = [1, 2, 3, 4, 5];

if numbers.contains(3) {
    print("Found 3!");
}

let hasEven = false;
for n in numbers {
    if n % 2 == 0 {
        hasEven = true;
        break;
    }
}
```

## Method Reference Table

| Method | Description | Returns | Mutates |
|--------|-------------|---------|---------|
| `length` | Get array size | number | No |
| `push(item)` | Add to end | void | Yes |
| `pop()` | Remove from end | item? | Yes |
| `shift()` | Remove from start | item? | Yes |
| `unshift(item)` | Add to start | void | Yes |
| `slice(start, end?)` | Extract portion | array | No |
| `reverse()` | Reverse order | array | No |
| `sort()` | Sort array | void | Yes |
| `contains(item)` | Check existence | boolean | No |
| `includes(item)` | Check existence | boolean | No |
| `map(fn)` | Transform elements | array | No |
| `filter(fn)` | Select elements | array | No |

## Best Practices

1. **Use `length` for array size** - More readable than manual counting
2. **Prefer `slice()` for immutability** - Creates a copy instead of modifying
3. **Use `contains()` for existence checks** - More efficient than manual loops
4. **Chain methods when appropriate** - But keep readability in mind

## Performance Notes

- Index access is O(1)
- `push()` and `pop()` are O(1)
- `shift()` and `unshift()` are O(n)
- `slice()` creates a copy - O(n)
- `sort()` is O(n log n)

## See Also

- [Arrays Guide](11-arrays.md) - Array basics and iteration
- [String Methods](16-string-methods.md) - String operations
- [Variables](03-variables.md) - Variable declarations
