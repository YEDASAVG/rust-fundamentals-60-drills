# Rust Fundamentals - 60 Drills Learning Journey

A comprehensive hands-on learning project covering Rust fundamentals through 60 progressive drills across 5 core topics.

## 📚 Topics Covered

### Phase 1: Loops (20 drills)
- `for`, `while`, `loop` constructs
- Iterator methods: `.iter()`, `.enumerate()`, `.rev()`, `.step_by()`
- Functional patterns: `.filter()`, `.map()`, `.collect()`, `.sum()`
- Advanced iterators: `.find()`, `.any()`, `.all()`, `.skip()`, `.take()`, `.zip()`, `.flatten()`
- Closures and iterator chaining

### Phase 2: Control Flow (10 drills)
- `if/else` statements and expressions
- Pattern matching with `match`
- Working with enums, `Option<T>`, and `Result<T, E>`
- Match guards and nested patterns
- `if let` syntax

### Phase 3: Functions (10 drills)
- Function parameters and return types
- Unit type and early returns
- Recursion
- Closures and environment capture
- Higher-order functions
- Method syntax vs associated functions
- Generic functions with trait bounds

### Phase 4: Variables & Ownership (10 drills)
- Mutability and shadowing
- Move semantics and ownership transfer
- Deep copying with `.clone()`
- Borrowing (immutable and mutable references)
- Borrowing rules and lifetime validation
- Dangling reference prevention
- String slices (`&str`)
- Copy vs Move traits

### Phase 5: Collections (10 drills)
- `Vec<T>`: creation, indexing, push/pop
- Vector iteration (immutable and mutable)
- `HashMap<K, V>`: insert, get, entry API
- HashMap iteration and updates
- `.collect()` transformations
- Filtering and collecting patterns
- Practical application: word frequency counter

## 🗂️ Project Structure

```
src/
├── bin/
│   ├── loops.rs         # Phase 1: Loop drills (1.1-1.20)
│   ├── control_flow.rs  # Phase 2: Control flow drills (2.1-2.10)
│   ├── functions.rs     # Phase 3: Function drills (3.1-3.10)
│   ├── ownership.rs     # Phase 4: Ownership drills (4.1-4.10)
│   └── collections.rs   # Phase 5: Collection drills (5.1-5.10)
└── main.rs             # Original compression project
```

## 🚀 Running the Drills

Each phase can be run independently:

```bash
# Run loops drills
cargo run --bin loops

# Run control flow drills
cargo run --bin control_flow

# Run function drills
cargo run --bin functions

# Run ownership drills
cargo run --bin ownership

# Run collection drills
cargo run --bin collections
```

## 📝 Learning Approach

Each drill follows a problem-driven methodology:
1. **Problem statement** - Clear objective and requirements
2. **Independent implementation** - Code solutions without direct answers
3. **Testing and validation** - Verify output matches expected results
4. **Concept reinforcement** - Understand why solutions work

All completed drills are commented out with their problem statements preserved for future reference.

## 🛠️ Technologies

- **Rust Edition**: 2024
- **Dependencies**: 
  - `flate2 = "1.0"` (for compression project)

## 📖 Key Learnings

- **Memory Safety**: Ownership system prevents memory bugs at compile time
- **Zero-Cost Abstractions**: Iterator patterns with no runtime overhead
- **Explicit Error Handling**: `Option` and `Result` types for safe error propagation
- **Functional Programming**: Closures, higher-order functions, and iterator chaining
- **Type Safety**: Strong static typing with generics and trait bounds

## 🎯 Completion Status

✅ **60/60 drills completed (100%)**

- ✅ Phase 1: Loops (20/20)
- ✅ Phase 2: Control Flow (10/10)
- ✅ Phase 3: Functions (10/10)
- ✅ Phase 4: Ownership (10/10)
- ✅ Phase 5: Collections (10/10)

## 📄 License

This is a personal learning project.
