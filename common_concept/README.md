# 🦀 Rust Common Concepts

This folder contains examples and explanations of common Rust concepts such as variables, data types, and functions. Each concept is demonstrated through simple code samples.

---

## 📦 1. Variables

### ✅ What You Learned

- Variables are declared using `let` and are **immutable by default**.
- Use `let mut` to make a variable mutable.
- **Shadowing** allows you to redeclare a variable with the same name, even with a different type.
- Constants use `const`, require type annotations, and are always immutable.

### 🧪 Example

```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);

    let x = "now it's a string"; // shadowing
    println!("x is now: {}", x);

    let mut y = 10;
    y = 20;
    println!("mutable y is: {}", y);

    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
}
```

---

## 🧮 2. Data Types

### ✅ What You Learned

Rust is a **statically typed language**, meaning every variable must have a known type at compile time.

### 🔹 Scalar Types

These represent a single value:

- `i32` – integer
- `f64` – floating point number
- `bool` – true/false
- `char` – a single Unicode character

```rust
fn main() {
    let age: i32 = 27;
    let pi: f64 = 3.14;
    let is_logged_in: bool = true;
    let initial: char = 'R';

    println!("Age: {}", age);
    println!("Pi: {}", pi);
    println!("Logged in? {}", is_logged_in);
    println!("Initial: {}", initial);
}
```

### 🔸 Compound Types

These group multiple values into one type.

#### 🧱 Tuples

- Can hold different types.
- Fixed length.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("Tuple values: {}, {}, {}", a, b, c);
}
```

#### 📦 Arrays

- Fixed length.
- All elements must be the same type.

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let first = arr[0];
    let second = arr[1];

    println!("First: {}, Second: {}", first, second);
}
```

---

## 🧰 3. Functions

### ✅ What You Learned

- Functions are defined using `fn`.
- Parameters must include types.
- Return values use `->` and omit the semicolon on the final expression to return it.
- `main` is the entry point of every Rust program.

### 🧪 Example

```rust
fn main() {
    greet("Rust");

    let result = add(5, 3);
    println!("Sum is: {}", result);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon means this value is returned
}
```

---

## 📘 Summary

| Concept              | Keyword                      | Notes                                    |
| -------------------- | ---------------------------- | ---------------------------------------- |
| Variable declaration | `let`, `let mut`             | Immutable by default                     |
| Shadowing            | `let` again                  | Create a new variable with the same name |
| Constant             | `const`                      | Requires type annotation                 |
| Scalar types         | `i32`, `f64`, `bool`, `char` | Basic types                              |
| Compound types       | `tuple`, `array`             | Group multiple values                    |
| Function definition  | `fn`                         | Must include type annotations            |

---

> ✨ Use this folder to test and experiment with these Rust fundamentals!
