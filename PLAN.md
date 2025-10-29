# jRust Development Plan

## Implemented Features ✅

### Core Language Features
- [x] Variables (`let`)
- [x] Constants (`const`)
- [x] Type inference
- [x] Comments (single-line `//` and multi-line `/* */`)

### Type System
- [x] Primitive types (`number`, `string`, `boolean`, `void`, `any`)
- [x] Arrays (dynamic `T[]` and static `T[T, N]`)
- [x] Structs (custom data structures)
- [x] Enums (with optional associated data)
- [x] Explicit type annotations

### Functions
- [x] Function declarations
- [x] Parameters with types
- [x] Return types
- [x] Function calls
- [x] Return statements

### Control Flow
- [x] If/Else statements
- [x] For loops (with `in` for iteration)
- [x] While loops
- [x] Break statements
- [x] Continue statements

### Error Handling
- [x] Try/Catch blocks
- [x] Throw statements

### Expressions & Operators
- [x] Binary operations (`+`, `-`, `*`, `/`, `%`)
- [x] Comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- [x] Logical operators (`&&`, `||`, `!`)
- [x] String concatenation
- [x] Array indexing
- [x] Member access (dot notation)
- [x] Method calls

### Module System
- [x] Import statements
- [x] Export statements
- [x] Module resolution

### I/O
- [x] Print statements

---

## Missing Features 🚧

### Object-Oriented Programming
- [ ] Classes
- [ ] Constructors
- [ ] Instance methods
- [ ] Static methods
- [ ] Inheritance
- [ ] Interfaces/Traits
- [ ] Private/Public visibility modifiers
- [ ] Getters/Setters
- [ ] `this` keyword (token exists but not implemented)
- [ ] `new` keyword (token exists but not implemented)

### Advanced Type System
- [ ] Generics
- [ ] Type aliases (`type` keyword exists but not implemented)
- [ ] Union types
- [ ] Intersection types
- [ ] Tuple types (as distinct from arrays)
- [ ] Option/Maybe types (explicit)
- [ ] Result types (explicit)
- [ ] Type guards
- [ ] Type casting

### Pattern Matching
- [ ] Match expressions
- [ ] Destructuring assignments
- [ ] Pattern matching on enums
- [ ] Pattern matching on structs

### Functions (Advanced)
- [ ] Closures/Lambda expressions
- [ ] Arrow functions
- [ ] Higher-order functions
- [ ] Default parameters
- [ ] Named parameters
- [ ] Rest parameters
- [ ] Variadic functions

### Collections
- [ ] Dictionary/Map types
- [ ] Set types
- [ ] Tuple types

### Async Programming
- [ ] Async/await
- [ ] Promises/Futures
- [ ] Async functions

### Language Features
- [ ] Spread operator (`...`)
- [ ] Optional chaining (`?.`)
- [ ] Nullish coalescing (`??`)
- [ ] Template literals/String interpolation
- [ ] Operator overloading
- [ ] Decorators/Attributes

### Memory & References
- [ ] References (`&`, `&mut`) - tokens exist but not fully implemented
- [ ] Pointers
- [ ] Borrowing rules
- [ ] Lifetimes

### Module System (Advanced)
- [ ] Module paths (`::`)
- [ ] Namespaces
- [ ] Re-exports
- [ ] Glob imports

### Metaprogramming
- [ ] Macros
- [ ] Compiler directives
- [ ] Conditional compilation

### Documentation & Testing
- [ ] Doc comments (`///`)
- [ ] In-code unit tests
- [ ] Benchmarks

### Iterators
- [ ] Iterator protocol
- [ ] Generators
- [ ] Yield expressions

### Misc
- [ ] Switch/Case statements
- [ ] Do-while loops
- [ ] Range expressions
- [ ] Labeled statements
- [ ] Goto (if needed)

---

## Priority Roadmap

### Phase 1: Foundation (✅ COMPLETE)
- ✅ Lexer
- ✅ Parser
- ✅ AST
- ✅ Basic code generation
- ✅ CLI tool
- ✅ Project structure

### Phase 2: Core Features (✅ COMPLETE)
- ✅ Variables and constants
- ✅ Functions
- ✅ Control flow (if/else, loops)
- ✅ Type system (primitives, arrays)
- ✅ Print statements

### Phase 3: Advanced Features (✅ COMPLETE)
- ✅ Structs
- ✅ Enums
- ✅ Type inference
- ✅ Static arrays
- ✅ Try/Catch/Throw
- ✅ Import/Export

### Phase 4: Object-Oriented (🚧 NEXT)
- [ ] Classes
- [ ] Constructors
- [ ] Instance methods
- [ ] Static methods
- [ ] Visibility modifiers
- [ ] `this` and `new` keywords

### Phase 5: Type System Enhancement
- [ ] Generics
- [ ] Type aliases
- [ ] Union types
- [ ] Tuple types
- [ ] Option/Result types

### Phase 6: Functional Programming
- [ ] Closures
- [ ] Arrow functions
- [ ] Higher-order functions
- [ ] Pattern matching

### Phase 7: Async Programming
- [ ] Async/await
- [ ] Promises
- [ ] Async functions

### Phase 8: Advanced Collections
- [ ] Dictionary/Map types
- [ ] Set types
- [ ] Advanced iterators

### Phase 9: Developer Experience
- [ ] Better error messages
- [ ] Language Server Protocol (LSP)
- [ ] VS Code extension
- [ ] Debugger support

### Phase 10: Performance & Optimization
- [ ] Compiler optimizations
- [ ] Benchmarking tools
- [ ] Profiling support

---

## Statistics

**Implemented:** 25 major features  
**Missing:** 60+ features  
**Progress:** ~30% complete (core foundation solid)

---

**Last Updated:** October 30, 2025  
**Current Phase:** Phase 3 Complete → Phase 4 Next
