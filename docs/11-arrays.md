# Arrays

Arrays store multiple values of the same type. jRust supports both **dynamic arrays** (variable size) and **static arrays** (fixed size).

## Array Types

### Dynamic Arrays

Dynamic arrays can grow and shrink at runtime. Use `Type[]` syntax:

```javascript
let numbers: number[] = [1, 2, 3, 4, 5];
let names: string[] = ["Alice", "Bob", "Charlie"];
let flags: boolean[] = [true, false, true];
```

**Dynamic arrays compile to Rust's `Vec<T>` type**, providing heap allocation and dynamic sizing.

### Static Arrays

Static arrays have a fixed size determined at compile time. Use `Type[ElementType, Size]` syntax:

```javascript
let numbers: number[number, 5] = [1, 2, 3, 4, 5];
let names: string[string, 3] = ["Alice", "Bob", "Charlie"];
let flags: boolean[boolean, 2] = [true, false];
```

**Static arrays compile to Rust's `[T; N]` type**, providing stack allocation and compile-time size guarantees.

### When to Use Each Type

**Use Dynamic Arrays (`Type[]`) when:**
- The size is unknown at compile time
- You need to add or remove elements
- You're working with collections that change size
- Memory efficiency isn't critical

**Use Static Arrays (`Type[Type, Size]`) when:**
- The size is known and won't change
- You want stack allocation (faster)
- You need compile-time size guarantees
- You're working with fixed-size data (coordinates, RGB values, etc.)

## Creating Arrays

### Dynamic Array Declaration

Declare a dynamic array with the `[]` syntax:

```javascript
let numbers: number[] = [1, 2, 3, 4, 5];
let names: string[] = ["Alice", "Bob", "Charlie"];
let flags: boolean[] = [true, false, true];
```

### Static Array Declaration

Declare a static array with `[Type, Size]` syntax:

```javascript
let numbers: number[number, 5] = [1, 2, 3, 4, 5];
let coordinates: number[number, 3] = [10, 20, 30];
let rgb: number[number, 3] = [255, 128, 0];
```

**Note:** The size must match the number of elements exactly, or you'll get a compile error.

### Empty Arrays

```javascript
let empty: number[] = [];
```

**Note:** Empty static arrays don't make sense since size is fixed, so only use dynamic arrays for initially empty collections.

### Comparison Examples

```javascript
// Dynamic array (Vec<i32>) - can grow/shrink
let dynamic: number[] = [1, 2, 3];

// Static array ([i32; 3]) - fixed size
let static: number[number, 3] = [1, 2, 3];
```

**Generated Rust code:**

```rust
let mut dynamic: Vec<i32> = vec![1, 2, 3];
let mut static: [i32; 3] = [1, 2, 3];
```

## Accessing Elements

### Index Access

Access elements using zero-based indexing:

```typescript
let fruits: string[] = ["apple", "banana", "orange"];

let first: string = fruits[0];      // "apple"
let second: string = fruits[1];     // "banana"
let third: string = fruits[2];      // "orange"

print(first);   // Output: apple
```

### Bounds Checking

jRust will catch index out of bounds errors at compile time:

```typescript
let arr: number[] = [1, 2, 3];

// Valid indices: 0, 1, 2
print(arr[0]);  // ✓ Valid
// print(arr[5]);  // ✗ Error: index out of bounds
```

## Array Length (Coming Soon)

Get the length of an array:

```typescript
let items: number[] = [1, 2, 3, 4, 5];
let length: number = items.length;  // 5

print("Array length: " + length);
```

## Iterating Over Arrays

### for...in Loop

Use `for...in` to iterate over array elements:

```typescript
let numbers: number[] = [10, 20, 30, 40];

for num in numbers {
    print(num);
}
```

Output:
```
10
20
30
40
```

### With Conditions

```typescript
let scores: number[] = [85, 92, 78, 88, 95];

for score in scores {
    if (score >= 90) {
        print("Excellent: " + score);
    }
}
```

### Multiple Arrays

```typescript
let names: string[] = ["Alice", "Bob", "Charlie"];
let ages: number[] = [25, 30, 28];

for name in names {
    print("Name: " + name);
}

for age in ages {
    print("Age: " + age);
}
```

## Array Types

### Homogeneous Arrays

All elements must be the same type:

```typescript
let numbers: number[] = [1, 2, 3];           // ✓ All numbers
let mixed: any[] = [1, "two", true];         // ✓ Mixed using 'any'
// let bad: number[] = [1, "two"];           // ✗ Type mismatch
```

### Arrays of Strings

```typescript
let countries: string[] = ["USA", "Canada", "Mexico"];

for country in countries {
    print(country);
}
```

### Arrays of Numbers

```typescript
let temperatures: number[] = [72, 68, 75, 71];

for temp in temperatures {
    if (temp > 70) {
        print("Warm: " + temp);
    }
}
```

### Arrays of Booleans

```typescript
let states: boolean[] = [true, false, true, false];

for state in states {
    if (state) {
        print("Active");
    }
}
```

## Array Methods (Coming Soon)

Future versions will support:

```typescript
let arr: number[] = [1, 2, 3];

arr.push(4);                          // Add element
let last: number = arr.pop();         // Remove and return last
let length: number = arr.length;      // Get length

let doubled: number[] = arr.map(x => x * 2);
let evens: number[] = arr.filter(x => x % 2 == 0);
let sum: number = arr.reduce((a, b) => a + b, 0);
```

## Practical Examples

### Working with Static Arrays

Static arrays are perfect for fixed-size data:

```javascript
// RGB color representation
let red: number[number, 3] = [255, 0, 0];
let green: number[number, 3] = [0, 255, 0];
let blue: number[number, 3] = [0, 0, 255];

function printColor(name: string, rgb: number[number, 3]): void {
    print(name + " = RGB(" + rgb[0] + ", " + rgb[1] + ", " + rgb[2] + ")");
}

printColor("Red", red);
printColor("Green", green);
printColor("Blue", blue);
```

### 3D Coordinates

```javascript
let origin: number[number, 3] = [0, 0, 0];
let pointA: number[number, 3] = [10, 20, 30];
let pointB: number[number, 3] = [5, 15, 25];

function distance(p1: number[number, 3], p2: number[number, 3]): number {
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    let dz = p2[2] - p1[2];
    
    return dx * dx + dy * dy + dz * dz;
}
```

### Days of Week

```javascript
const DAYS: string[string, 7] = [
    "Monday",
    "Tuesday", 
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday"
];

function printWeek(): void {
    for day in DAYS {
        print(day);
    }
}
```

### Sum of Numbers (Dynamic Array)

```typescript
function sum(arr: number[]): number {
    let total: number = 0;
    
    for num in arr {
        total = total + num;
    }
    
    return total;
}

function main(): void {
    let numbers: number[] = [1, 2, 3, 4, 5];
    let result: number = sum(numbers);
    print("Sum: " + result);  // Output: Sum: 15
}
```

### Count Matches

```typescript
function countGreaterThan(arr: number[], threshold: number): number {
    let count: number = 0;
    
    for num in arr {
        if (num > threshold) {
            count = count + 1;
        }
    }
    
    return count;
}

function main(): void {
    let scores: number[] = [85, 92, 78, 88, 95];
    let high: number = countGreaterThan(scores, 90);
    print("Scores above 90: " + high);
}
```

### Find First Match

```typescript
function findFirst(arr: number[], target: number): number {
    for num in arr {
        if (num == target) {
            return num;
        }
    }
    return -1;
}

function main(): void {
    let numbers: number[] = [10, 20, 30, 40];
    let found: number = findFirst(numbers, 30);
    print("Found: " + found);
}
```

### Process Array Elements

```typescript
function processTemperatures(temps: number[]): void {
    print("Temperature Report:");
    
    for temp in temps {
        let status: string = "";
        
        if (temp > 80) {
            status = "Hot";
        } else if (temp > 60) {
            status = "Warm";
        } else if (temp > 40) {
            status = "Cool";
        } else {
            status = "Cold";
        }
        
        print("  " + temp + "°F - " + status);
    }
}

function main(): void {
    let temperatures: number[] = [85, 72, 50, 68, 92];
    processTemperatures(temperatures);
}
```

## Array Borrowing

Pass arrays by reference to avoid copies:

```typescript
function printArray(arr: &number[]): void {
    for num in arr {
        print(num);
    }
}

function main(): void {
    let nums: number[] = [1, 2, 3];
    printArray(&nums);
    print(nums[0]);  // ✓ Still valid
}
```

## Working with Multiple Arrays

```typescript
function compareArrays(arr1: &number[], arr2: &number[]): void {
    print("Array 1:");
    for num in arr1 {
        print("  " + num);
    }
    
    print("Array 2:");
    for num in arr2 {
        print("  " + num);
    }
}

function main(): void {
    let first: number[] = [1, 2, 3];
    let second: number[] = [4, 5, 6];
    
    compareArrays(&first, &second);
}
```

## Complete Example

```typescript
const MIN_SCORE: number = 60;
const EXCELLENT: number = 90;

function analyzeScores(scores: number[]): void {
    let pass: number = 0;
    let fail: number = 0;
    let excellent: number = 0;
    
    for score in scores {
        if (score >= EXCELLENT) {
            excellent = excellent + 1;
        } else if (score >= MIN_SCORE) {
            pass = pass + 1;
        } else {
            fail = fail + 1;
        }
    }
    
    print("Results:");
    print("  Excellent (90+): " + excellent);
    print("  Passed (60-89): " + pass);
    print("  Failed (<60): " + fail);
}

function main(): void {
    let scores: number[] = [95, 87, 52, 91, 78, 45, 88, 92];
    analyzeScores(scores);
}
```

## Best Practices

1. **Choose the right array type**
   ```javascript
   let dynamic: number[] = [1, 2, 3];              // ✓ When size changes
   let static: number[number, 3] = [1, 2, 3];     // ✓ When size is fixed
   ```

2. **Use static arrays for known sizes** - They're faster and safer
   ```javascript
   let rgb: number[number, 3] = [255, 128, 0];    // ✓ Always 3 elements
   let coords: number[number, 3] = [10, 20, 30];  // ✓ Always x, y, z
   ```

3. **Use descriptive names** for arrays
   ```javascript
   let studentScores: number[] = [85, 90];   // ✓ Clear
   let s: number[] = [85, 90];               // ✗ Unclear
   ```

4. **Check bounds** before accessing by index
   ```javascript
   if (index >= 0 && index < array.length) {
       print(array[index]);
   }
   ```

5. **Use for loops** for iteration
   ```javascript
   for item in items {
       process(item);
   }
   ```

6. **Pass arrays by reference** for efficiency
   ```javascript
   function process(arr: &number[]): void { }
   ```

7. **Keep arrays homogeneous** - same type for all elements
   ```javascript
   let numbers: number[] = [1, 2, 3];  // ✓ All numbers
   ```

## Coming Soon

Future versions will support:
- Array methods (map, filter, reduce, etc.)
- Array slicing
- Multi-dimensional arrays
- Destructuring
- Spread operator

## Next Steps

You've learned about arrays! Explore:
- **[Control Flow](10-control-flow.md)** — Revisit loops for more patterns
- **[Functions](05-functions.md)** — Pass arrays to functions
- **[Advanced Topics](13-advanced.md)** — Structs, enums, methods

