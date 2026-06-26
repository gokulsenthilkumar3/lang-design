# Syntax Sketches

This file contains illustrative syntax examples. These are design sketches, not final syntax. Everything here is subject to change through the RFC process.

---

## Basic Types and Variables

```
// Immutable binding — type inferred
let name = "Gokul"
let count = 42
let ratio = 3.14

// Mutable binding — mutation is explicit
let mut total = 0
total = total + 1

// Explicit type annotation (at boundaries)
let id: UUID = UUID.new()
```

---

## Functions

```
// Simple function
fn square(x: Int) -> Int {
    x * x
}

// Multiple return via tuple
fn min_max(xs: List[Int]) -> (Int, Int) {
    (xs.min(), xs.max())
}

// Function with IO effect
fn read_config(path: Path) -> Result[Config, IOError]
    requires IO
{
    let content = File.read(path)?   // ? propagates error
    Config.parse(content)
}

// Pure function (no effects required — verifiable)
fn factorial(n: Nat) -> Nat {
    match n {
        0 => 1
        n => n * factorial(n - 1)
    }
}
```

---

## Structs and Enums

```
// Struct
struct User {
    id: UUID
    name: Str
    email: Email
    created_at: Timestamp
}

// Struct with default fields
struct Config {
    host: Str = "localhost"
    port: Int = 8080
    debug: Bool = false
}

// Enum (algebraic data type)
enum Result[T, E] {
    Ok(T)
    Err(E)
}

enum Tree[T] {
    Leaf
    Node { value: T, left: Tree[T], right: Tree[T] }
}
```

---

## Pattern Matching

```
// Match on enum
fn describe(shape: Shape) -> Str {
    match shape {
        Circle { radius } => "circle with radius {radius}"
        Rectangle { width, height } => "rectangle {width}x{height}"
        Triangle { base, height } => "triangle base {base} height {height}"
    }
}

// Match with guard
fn classify(n: Int) -> Str {
    match n {
        0 => "zero"
        n if n < 0 => "negative"
        n if n % 2 == 0 => "positive even"
        _ => "positive odd"
    }
}
```

---

## Ownership and Borrowing (Sketch)

```
// Ownership transfer (move semantics, like Rust)
let data = Buffer.new(1024)
process(data)           // data is moved; cannot use data after this

// Borrow (read-only reference, inferred by compiler)
let data = Buffer.new(1024)
let size = measure(&data)  // &data = borrow; data still owned here
process(data)              // move after borrow ends — valid

// Mutable borrow
let mut data = Buffer.new(1024)
fill(&mut data)            // mutable borrow
process(data)              // move after mutable borrow ends
```

---

## Gradual Formalism (Refinement Types, Opt-In)

```
// Informal — no proof obligations, compiles and runs
fn divide(a: Int, b: Int) -> Int {
    a / b   // runtime panic if b == 0
}

// Formal — proof obligation declared; compiler checks it
#[verify]
fn divide_safe(a: Int, b: Int) -> Int
    requires b != 0
    ensures result * b == a
{
    a / b   // no runtime panic possible; proven at compile time
}

// Refined type alias
type NonZeroInt = Int where self != 0

fn divide_typed(a: Int, b: NonZeroInt) -> Int {
    a / b   // b is statically known to be non-zero
}
```

---

## Algebraic Effects

```
// Define an effect
effect Logger {
    fn log(msg: Str) -> ()
}

// Function that uses the Logger effect
fn process(data: Data) -> Result[Output, ProcessError]
    requires Logger
{
    Logger.log("starting process")
    // ...
    Logger.log("done")
    Ok(output)
}

// Handle the effect at the call site
with Logger.handle(|msg| println(msg)) {
    process(data)
}

// Test with a mock handler
with Logger.handle(|msg| test_log.append(msg)) {
    process(data)
}
```

---

## AI / Tensor Types (Sketch)

```
// Tensor with shape encoded in the type
let weights: Tensor[Float32, (768, 768)] = Tensor.zeros()
let input:   Tensor[Float32, (1, 768)]   = embed(tokens)

// Shape mismatch caught at compile time, not runtime
let output = matmul(input, weights)   // type: Tensor[Float32, (1, 768)]

// Batch dimension
let batch: Tensor[Float32, (32, 768)] = embed_batch(tokens)
```
