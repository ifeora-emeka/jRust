# Control Flow

Control flow statements direct how your program executes. jRust provides `if/else`, `for` loops, `while` loops, and loop control with `break` and `continue`.

## Conditionals: if/else

### Basic if

```typescript
let age: number = 18;

if (age >= 18) {
    print("You are an adult");
}
```

### if/else

```typescript
let score: number = 75;

if (score >= 80) {
    print("You passed!");
} else {
    print("You failed!");
}
```

### Multiple Conditions

```typescript
let temperature: number = 25;

if (temperature < 0) {
    print("Freezing");
} else if (temperature < 15) {
    print("Cold");
} else if (temperature < 25) {
    print("Cool");
} else {
    print("Hot");
}
```

### Conditions with Logical Operators

Use `&&` (AND) and `||` (OR):

```typescript
let age: number = 25;
let hasLicense: boolean = true;

if (age >= 18 && hasLicense) {
    print("Can drive");
}

let isWeekend: boolean = true;
let isHoliday: boolean = false;

if (isWeekend || isHoliday) {
    print("Day off!");
}
```

### Negation with !

```typescript
let isRaining: boolean = false;

if (!isRaining) {
    print("Good weather!");
}
```

## Loops: for

Iterate over arrays with `for...in`:

```typescript
let numbers: number[] = [1, 2, 3, 4, 5];

for num in numbers {
    print(num);
}
```

### Loop with Conditions

```typescript
let items: number[] = [10, 20, 30, 40, 50];

for item in items {
    if (item > 25) {
        print("Large: " + item);
    }
}
```

### Nested Loops

```typescript
let matrix: number[][] = [[1, 2], [3, 4]];

for row in matrix {
    for col in row {
        print(col);
    }
}
```

## Loops: while

Repeat while a condition is true:

```typescript
let count: number = 0;

while (count < 5) {
    print(count);
    count = count + 1;
}
```

### Countdown

```typescript
let seconds: number = 10;

while (seconds > 0) {
    print(seconds);
    seconds = seconds - 1;
}

print("Blast off!");
```

## Loop Control

### break: Exit Early

Use `break` to exit a loop immediately:

```typescript
let found: boolean = false;

for item in [1, 2, 3, 4, 5] {
    if (item == 3) {
        found = true;
        break;
    }
}

print("Found: " + found);
```

### continue: Skip Iteration

Use `continue` to skip to the next iteration:

```typescript
for num in [1, 2, 3, 4, 5] {
    if (num == 3) {
        continue;
    }
    print(num);
}
```

Output:
```
1
2
4
5
```

## Practical Examples

### Search in Array

```typescript
function findIndex(arr: number[], target: number): number {
    let result: number = -1;
    
    for i in arr {
        if (i == target) {
            result = 1;
            break;
        }
    }
    
    return result;
}
```

### Validate Input

```typescript
function isValid(value: number): boolean {
    if (value < 0) {
        return false;
    }
    if (value > 100) {
        return false;
    }
    return true;
}
```

### Process Data

```typescript
function processScores(scores: number[]): void {
    for score in scores {
        if (score < 0) {
            print.warn("Invalid score: " + score);
            continue;
        }
        
        if (score > 100) {
            print.warn("Score out of range: " + score);
            continue;
        }
        
        print("Valid score: " + score);
    }
}
```

### Early Exit Patterns

```typescript
function findFirstEven(arr: number[]): number {
    for num in arr {
        if (num % 2 == 0) {
            return num;
        }
    }
    return -1;  // Not found
}
```

## Complete Example

```typescript
const MIN_SCORE: number = 60;
const MAX_ATTEMPTS: number = 3;

function gradeStudent(score: number): string {
    if (score >= 90) {
        return "A";
    } else if (score >= 80) {
        return "B";
    } else if (score >= 70) {
        return "C";
    } else if (score >= 60) {
        return "D";
    } else {
        return "F";
    }
}

function processScores(scores: number[]): void {
    let passCount: number = 0;
    
    for score in scores {
        if (score < MIN_SCORE) {
            print.warn("Failed score: " + score);
            continue;
        }
        
        let grade: string = gradeStudent(score);
        print("Score: " + score + " Grade: " + grade);
        passCount = passCount + 1;
    }
    
    print.info("Passed: " + passCount + " out of " + scores.length);
}

function main(): void {
    let scores: number[] = [85, 45, 92, 78, 55, 88];
    processScores(scores);
}
```

## Common Patterns

### Check Before Processing

```typescript
for item in items {
    if (!isValid(item)) {
        continue;
    }
    process(item);
}
```

### Early Return

```typescript
function search(arr: number[], target: number): boolean {
    for item in arr {
        if (item == target) {
            return true;
        }
    }
    return false;
}
```

### Counter Pattern

```typescript
let count: number = 0;

for item in items {
    if (matches(item)) {
        count = count + 1;
    }
}
```

## Best Practices

1. **Use descriptive variable names** in loops
   ```typescript
   for num in numbers { }      // ✓ Clear
   for x in arr { }            // ✗ Unclear
   ```

2. **Keep loop bodies simple** — Extract complex logic into functions
   ```typescript
   for item in items {
       process(item);          // ✓ Clear
   }
   ```

3. **Use break for found conditions** — Early exit is efficient
   ```typescript
   if (found) {
       break;
   }
   ```

4. **Use continue for skip conditions** — Avoid deep nesting
   ```typescript
   if (!valid) {
       continue;
   }
   ```

## Next Steps

You've learned control flow! Now explore:
- **[Arrays](11-arrays.md)** — Working with collections
- **[String Methods](12-string-methods.md)** — String manipulation (coming soon)
- **[Advanced Topics](13-advanced.md)** — Structs, enums, error handling (coming soon)

