# jRust Documentation Index

Welcome to the jRust documentation! This guide will help you navigate all available resources and learn jRust from basics to advanced topics.

## Quick Navigation

### Getting Started (15 minutes)
1. [Introduction to jRust](01-introduction.md) — Overview and design philosophy
2. [Getting Started](02-get-started.md) — Installation and first program

### Learning the Basics (1-2 hours)
3. [Variables and Constants](03-variables.md) — Declare and manage data
4. [Primitive Types](04-primitive-types.md) — number, string, boolean, void
5. [Functions](05-functions.md) — Define and call functions
6. [Comments](06-comments.md) — Document your code

### Control Flow & Data (1 hour)
7. [Control Flow](10-control-flow.md) — if/else, loops, break, continue
8. [Arrays](11-arrays.md) — Create, access, and iterate arrays

### Advanced Features (2-3 hours)
9. [Advanced Types](12-advanced-types.md) — Structs, enums, type inference
10. [Array Methods](15-array-methods.md) — push, pop, map, filter, slice
11. [String Methods](16-string-methods.md) — toUpperCase, substring, split
12. [Error Handling](17-error-handling.md) — try/catch for robust code

### Deep Concepts (1 hour)
13. [Ownership and Borrowing](07-ownership-and-borrowing.md) — Memory safety
14. [Strings](08-strings.md) — String operations and concatenation
15. [Output and Logging](09-output.md) — Print statements and logging

### Future Features (Preview)
16. [Advanced Topics](13-advanced.md) — Planned features and roadmap

## Learning Paths

### Path 1: Complete Beginner
If you're new to programming, follow this order:

1. Read: [Introduction to jRust](01-introduction.md)
2. Follow: [Getting Started](02-get-started.md) — Build your first program
3. Learn: [Variables and Constants](03-variables.md) — Store data
4. Learn: [Primitive Types](04-primitive-types.md) — Work with different data types
5. Practice: [Functions](05-functions.md) — Organize code into functions
6. Practice: [Comments](06-comments.md) — Document code
7. Learn: [Control Flow](10-control-flow.md) — Make decisions in code
8. Practice: [Arrays](11-arrays.md) — Work with collections
9. Learn: [Output and Logging](09-output.md) — Display results
10. Study: [Ownership and Borrowing](07-ownership-and-borrowing.md) — Understand memory safety
11. Learn: [Strings](08-strings.md) — Advanced string operations
12. Explore: [Advanced Topics](13-advanced.md) — See what's coming

**Estimated Time:** 4-6 hours

### Path 2: Intermediate Programmer (Familiar with Other Languages)
If you know another language, you can move faster:

1. Skim: [Introduction to jRust](01-introduction.md)
2. Quick Setup: [Getting Started](02-get-started.md) — Get the tools running
3. Review: [Variables and Constants](03-variables.md) — Check jRust syntax
4. Review: [Primitive Types](04-primitive-types.md) — Understand type system
5. Study: [Control Flow](10-control-flow.md) — Learn loop control (break/continue)
6. Study: [Functions](05-functions.md) — See parameter and return syntax
7. Study: [Arrays](11-arrays.md) — Array syntax and iteration
8. Important: [Ownership and Borrowing](07-ownership-and-borrowing.md) — Critical difference from other languages
9. Reference: [Strings](08-strings.md) — String handling details
10. Reference: [Output and Logging](09-output.md) — Logging capabilities
11. Explore: [Advanced Topics](13-advanced.md) — Upcoming features

**Estimated Time:** 2-3 hours

### Path 3: Experienced Rust Developer
If you know Rust:

1. [Introduction to jRust](01-introduction.md) — How jRust differs from Rust
2. [Getting Started](02-get-started.md) — Setup and transpilation
3. [Variables and Constants](03-variables.md) — jRust syntax
4. [Control Flow](10-control-flow.md) — Language features
5. [Functions](05-functions.md) — Function system
6. [Arrays](11-arrays.md) — Collection syntax
7. [Advanced Topics](13-advanced.md) — Planned features
8. Reference: Other docs as needed

**Estimated Time:** 30-45 minutes

## Documentation Structure

### Core Language Docs
| Document | Focus | Time | Level |
|----------|-------|------|-------|
| [Introduction](01-introduction.md) | Why jRust? Design philosophy | 5 min | Beginner |
| [Getting Started](02-get-started.md) | Installation and first steps | 15 min | Beginner |
| [Variables](03-variables.md) | Variable and constant declarations | 20 min | Beginner |
| [Primitive Types](04-primitive-types.md) | number, string, boolean, void types | 15 min | Beginner |
| [Functions](05-functions.md) | Function declarations and calls | 25 min | Beginner |
| [Comments](06-comments.md) | Code documentation | 10 min | Beginner |

### Language Features
| Document | Focus | Time | Level |
|----------|-------|------|-------|
| [Control Flow](10-control-flow.md) | if/else, for, while, break, continue | 30 min | Beginner |
| [Arrays](11-arrays.md) | Array creation, access, iteration | 25 min | Beginner |
| [Strings](08-strings.md) | String operations and concatenation | 20 min | Beginner |
| [Ownership & Borrowing](07-ownership-and-borrowing.md) | Memory safety, references | 30 min | Intermediate |

### Tools & Environment
| Document | Focus | Time | Level |
|----------|-------|------|-------|
| [Output & Logging](09-output.md) | Logging and debugging | 20 min | Beginner |

### Upcoming Features
| Document | Focus | Time | Level |
|----------|-------|------|-------|
| [Advanced Topics](13-advanced.md) | Roadmap and planned features | 20 min | Intermediate |

## Feature Coverage

### Current Features (Phase 3.2) ✅

✅ **Fully Implemented:**
- Variables and constants with **type inference**
- All primitive types (number, string, boolean, void, any)
- **Struct types** for custom data structures
- **Enumerations** with optional data
- User-defined functions with parameters and returns
- If/else conditionals
- for loops with iteration
- while loops with conditions
- break statements for loop exit
- continue statements for loop iteration
- **Dynamic arrays** (Vec<T>) and **static arrays** ([T; N]) with fixed size
- Arrays with indexing and iteration
- **Array methods:** push, pop, shift, unshift, slice, map, filter, reverse, sort, contains
- **String methods:** toUpperCase, toLowerCase, substring, charAt, indexOf, trim, split, join
- **Error handling:** try/catch blocks and throw statements
- Ownership and borrowing (&, &mut)
- String concatenation
- Comments (single and multi-line)
- Logging functions (print)

📋 **Coming Soon (Phase 3.3-4):**
- Closures and lambda expressions
- Pattern matching
- Async/await support
- Module system improvements
- Generic types
- Advanced error types

## Quick Reference

### Syntax Cheatsheet

**Variables:**
```javascript
let x: number = 10;
const PI: number = 3.14;
let name: string = "Alice";
```

**Functions:**
```javascript
function add(a: number, b: number): number {
    return a + b;
}
```

**Control Flow:**
```javascript
if (x > 0) {
    print("positive");
} else {
    print("not positive");
}

for (let i: number in range(0, 10)) {
    if (i == 5) break;
    if (i % 2 == 0) continue;
    print(i);
}
```

**Arrays:**
```javascript
// Dynamic arrays (growable)
let arr: number[] = [1, 2, 3];

// Static arrays (fixed size)
let fixed: number[number, 5] = [1, 2, 3, 4, 5];
let rgb: number[number, 3] = [255, 128, 0];

// Array access
let first: number = arr[0];

// Array iteration
for (let item: number in arr) {
    print(item);
}
```

**Logging:**
```javascript
print("Message");
print.error("Error");
print.info("Info");
print.warn("Warning");
print.debug("Debug");
```

## How to Use These Docs

### For Learning
1. Pick a learning path above
2. Read docs in order
3. Try examples as you go
4. Practice with your own code

### For Reference
1. Use the Quick Navigation section to jump to topics
2. Use the table of contents in each document
3. Search for specific keywords

### For Building Projects
1. Start with Getting Started
2. Reference specific docs as needed
3. Check Advanced Topics for upcoming features

## Tips for Success

### Start Small
- Begin with simple programs
- Add features incrementally
- Build confidence gradually

### Practice
- Type out examples (don't copy-paste)
- Modify examples to experiment
- Create your own small projects

### Understand Ownership
- This is the most important concept
- It differs from most languages
- Invest time understanding it

### Read Error Messages
- Compiler errors are helpful
- They tell you what's wrong
- Use them to learn

### Refer Back
- Don't memorize everything
- Come back to docs frequently
- This is normal and expected

## Common Questions

**Q: What's the best way to learn jRust?**
A: Pick a learning path above, follow it in order, and practice as you go.

**Q: Can I skip Ownership and Borrowing?**
A: No, it's crucial to understanding jRust. Give yourself time with that section.

**Q: Where are the examples?**
A: Each document has code examples. The docs work better as a reference alongside actual code.

**Q: What language is jRust compiled to?**
A: jRust transpiles to Rust code, which then compiles to machine code.

**Q: When will advanced features be available?**
A: Check [Advanced Topics](13-advanced.md) for the roadmap and expected timeline.

## Getting Help

### Resources
- GitHub Issues: https://github.com/ifeora-emeka/jRust/issues
- Project README: Check the main README.md
- Source Code: Browse the crates/ directory

### Contributing
- See CONTRIBUTING.md for guidelines
- Documentation improvements welcome
- Share feedback and suggestions

## Documentation Roadmap

### Current Version (Phase 3.1)
- ✅ 13 documentation files
- ✅ 6,500+ lines of content
- ✅ 100+ code examples
- ✅ Complete for current features

### Next Updates
- Advanced Topics expansion as features are added
- Troubleshooting guide
- Example projects directory
- Interactive tutorials (future)

## Staying Updated

Check these files for updates:
- **README.md** — Project overview and latest news
- **CHANGELOG.md** — Version history and changes
- **MVP-PLAN.md** — Development roadmap
- **language-spec-and-architecture.md** — Technical details

Subscribe to updates:
- Watch the GitHub repository
- Check releases for feature announcements
- Review documentation updates

---

## Start Your jRust Journey

Ready to learn? Pick your learning path above and get started!

**Beginner?** Start with [Introduction](01-introduction.md) and [Getting Started](02-get-started.md).

**Experienced?** Jump to [Variables](03-variables.md) or your area of interest.

**Want to see what's coming?** Check [Advanced Topics](13-advanced.md).

---

**Last Updated:** Phase 3.1
**Total Documentation:** 13 files, 6,500+ lines
**Coverage:** Complete for current features

