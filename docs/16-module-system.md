# Module System and Imports

jRust provides a powerful module system that supports both **Rust standard library/crate imports** and **local module imports/exports**, making it easy to organize code and leverage Rust's ecosystem.

## Import Statement Types

### 1. Rust Standard Library Imports

Import from Rust's std library using Rust-style paths:

```javascript
// Single import
import File from "std::fs";

// Multiple imports
import {Read, Write, Seek} from "std::io";

// Import with alias
import HashMap from "std::collections" as Map;

// Import everything (self)
import {self, Read} from "std::io";
```

**Compiles to:**

```rust
use std::fs::File;
use std::io::{Read, Write, Seek};
use std::collections::HashMap as Map;
use std::io::{self, Read};
```

### 2. External Crate Imports

Import from external Rust crates (from Cargo.toml):

```javascript
// Serde imports
import {Serialize, Deserialize} from "serde";

// Tokio imports
import tokio from "tokio";

// reqwest for HTTP
import Client from "reqwest";
```

**Compiles to:**

```rust
use serde::{Serialize, Deserialize};
use tokio;
use reqwest::Client;
```

### 3. Local Module Imports

Import from local jRust modules using relative paths:

```javascript
// Import from local file
import {add, multiply} from "./utils/math";

// Import from index file
import helpers from "./utils";

// Import with alias
import {User} from "./models/user" as UserModel;
```

**Compiles to:**

```rust
use utils::math::{add, multiply};
use utils::helpers;
use models::user::User as UserModel;
```

## Export Statements

Use `export` to make functions, structs, enums, and constants available to other modules:

### Exporting Functions

```javascript
// Export a function
export function add(a: number, b: number): number {
    return a + b;
}

export function multiply(a: number, b: number): number {
    return a * b;
}
```

**Compiles to:**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

### Exporting Structs

```javascript
// Export a struct
export struct Point {
    x: number,
    y: number
}

export struct User {
    name: string,
    age: number,
    email: string
}
```

**Compiles to:**

```rust
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}
```

### Exporting Enums

```javascript
// Export an enum
export enum Status {
    Pending,
    Active,
    Completed
}

export enum Result {
    Ok,
    Err
}
```

**Compiles to:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Result {
    Ok,
    Err,
}
```

### Exporting Constants

```javascript
// Export constants
export const PI: number = 3.14159;
export const MAX_SIZE: number = 1000;
```

**Compiles to:**

```rust
pub const PI: &str = 3.14159;
pub const MAX_SIZE: i32 = 1000;
```

## Complete Examples

### Example 1: Math Utilities Module

**File: utils/math.jr**

```javascript
export const PI: number = 3.14159;

export function add(a: number, b: number): number {
    return a + b;
}

export function multiply(a: number, b: number): number {
    return a * b;
}

export function circleArea(radius: number): number {
    return PI * radius * radius;
}
```

**File: index.jr**

```javascript
import {add, multiply, circleArea, PI} from "./utils/math";

function main(): void {
    let sum = add(10, 20);
    print("Sum: " + sum);
    
    let product = multiply(5, 6);
    print("Product: " + product);
    
    let area = circleArea(5);
    print("Circle area: " + area);
}
```

**Generated: utils/math.rs**

```rust
pub const PI: f64 = 3.14159;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn circle_area(radius: i32) -> i32 {
    PI * radius * radius
}
```

**Generated: main.rs**

```rust
use utils::math::{add, multiply, circle_area, PI};

fn main() {
    let mut sum = add(10, 20);
    println!("Sum: {}", sum);
    
    let mut product = multiply(5, 6);
    println!("Product: {}", product);
    
    let mut area = circle_area(5);
    println!("Circle area: {}", area);
}
```

### Example 2: Using Rust Std Library

**File: index.jr**

```javascript
import {File, Read} from "std::fs";
import {HashMap} from "std::collections";

function readConfig(path: string): void {
    let file = File::open(path);
    print("File opened");
}

function createCache(): void {
    let cache = HashMap::new();
    print("Cache created");
}
```

**Generated: main.rs**

```rust
use std::fs::{File, Read};
use std::collections::HashMap;

fn read_config(path: String) {
    let mut file = File::open(path);
    println!("File opened");
}

fn create_cache() {
    let mut cache = HashMap::new();
    println!("Cache created");
}

fn main() {
}
```

### Example 3: External Crates (Serde)

**File: models/user.jr**

```javascript
import {Serialize, Deserialize} from "serde";

export struct User {
    name: string,
    email: string,
    age: number
}
```

**File: index.jr**

```javascript
import {User} from "./models/user";

function main(): void {
    let user = User {
        name: "Alice",
        email: "alice@example.com",
        age: 30
    };
    
    print("User created");
}
```

**Generated: models/user.rs**

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}
```

**Generated: main.rs**

```rust
mod models;
use models::user::User;

fn main() {
    let mut user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };
    
    println!("User created");
}
```

## Import Syntax Reference

| Syntax | Description | Example |
|--------|-------------|---------|
| `import Name from "path"` | Single import | `import File from "std::fs"` |
| `import {A, B} from "path"` | Multiple imports | `import {Read, Write} from "std::io"` |
| `import Name from "path" as Alias` | Import with alias | `import HashMap from "std::collections" as Map` |
| `import {A as B} from "path"` | Import item with alias | `import {File as F} from "std::fs"` |
| `import Name from "./path"` | Local module import | `import {add} from "./utils"` |

## Module Resolution

### External Crates/Std Library
- Paths with `::` are treated as Rust crate/std imports
- Examples: `"std::fs"`, `"std::collections"`, `"serde"`
- Maps directly to Rust `use` statements

### Local Modules
- Paths starting with `./` or `../` are local imports
- File structure: `./utils/math.jr` → `utils/math.rs`
- Directory with `index.jr` becomes module: `./utils` → `utils/mod.rs`

## Project Structure

```
my-app/
├── src/
│   ├── index.jr          → main.rs
│   ├── utils/
│   │   ├── index.jr      → utils/mod.rs
│   │   ├── math.jr       → utils/math.rs
│   │   └── strings.jr    → utils/strings.rs
│   ├── models/
│   │   ├── index.jr      → models/mod.rs
│   │   ├── user.jr       → models/user.rs
│   │   └── product.jr    → models/product.rs
│   └── api/
│       ├── index.jr      → api/mod.rs
│       └── client.jr     → api/client.rs
├── jrust.toml
└── Cargo.toml (generated)
```

## Best Practices

1. **Group related functionality into modules**
   ```javascript
   // utils/math.jr - Math operations
   // utils/strings.jr - String operations
   // utils/index.jr - Re-export utilities
   ```

2. **Use explicit imports**
   ```javascript
   import {add, multiply} from "./utils/math";  // ✓ Clear
   import * as math from "./utils/math";         // ✗ Less clear
   ```

3. **Export only what's needed**
   ```javascript
   // Only export public API
   export function publicAPI(): void { }
   
   // Keep internal functions private
   function internalHelper(): void { }
   ```

4. **Use aliases for clarity**
   ```javascript
   import HashMap from "std::collections" as Map;
   let cache = Map::new();
   ```

5. **Organize by feature, not type**
   ```
   ✓ Good:
   src/
   ├── user/
   │   ├── model.jr
   │   ├── service.jr
   │   └── api.jr
   
   ✗ Less ideal:
   src/
   ├── models/
   ├── services/
   └── apis/
   ```

## Error Examples

### Error 1: Missing Import Path

```javascript
import {File} from;  // ✗ Error
```

**Error:**
```
Error: Expected string literal for import path
  at line 1, column 19
```

**Fix:**
```javascript
import {File} from "std::fs";  // ✓ Correct
```

### Error 2: Invalid Import Syntax

```javascript
import File "std::fs";  // ✗ Error: missing 'from'
```

**Error:**
```
Error: Expected 'from' after import identifier
  at line 1, column 13
```

**Fix:**
```javascript
import File from "std::fs";  // ✓ Correct
```

### Error 3: Export Non-Exportable

```javascript
export if x > 0 { }  // ✗ Error
```

**Error:**
```
Error: Expected function, struct, enum, const, or let after export
  at line 1, column 8
```

**Fix:**
```javascript
export function checkPositive(x: number): boolean {
    return x > 0;
}
```

### Error 4: Circular Dependency

```javascript
// a.jr
import {funcB} from "./b";

// b.jr
import {funcA} from "./a";  // ✗ Circular dependency
```

**Fix:** Extract shared code to a third module:
```javascript
// shared.jr
export function sharedFunc(): void { }

// a.jr
import {sharedFunc} from "./shared";

// b.jr
import {sharedFunc} from "./shared";
```

## Configuration (jrust.toml)

Configure external crate dependencies:

```toml
[package]
name = "my-app"
version = "0.0.1"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"
```

## See Also

- **[Variables](03-variables.md)** - Variable declarations
- **[Functions](05-functions.md)** - Function definitions
- **[Structs](12-advanced-types.md)** - Struct definitions
- **[Advanced](13-advanced.md)** - Advanced features
