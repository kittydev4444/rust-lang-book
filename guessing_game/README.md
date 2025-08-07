# 🎯 Guessing Game – Rust Book Chapter 1 Summary

This project is based on the first chapter of the Rust book, where you build a simple CLI game that lets the user guess a random number.

---

## 🧠 What You Learned

### 📦 Crates and Dependencies

- **`rand`** – for random number generation.
- **`colored`** – to style terminal output (e.g., red/green messages).

### 💬 Basic I/O

- Reading input from the user using `io::stdin()`.
- Using `String::new()` and mutable variables.

### 🔢 Parsing Input

- Converting strings to numbers using `.trim().parse()`.
- Handling parse errors gracefully using `match`.

### 🔁 Control Flow

- Infinite loops using `loop {}`.
- Breaking out of loops with `break`.
- Branching using `match` and pattern matching with `Ordering`.

### ⚖️ Comparison

- `Ordering::Less`, `Greater`, `Equal` from `std::cmp::Ordering`.
- Comparing values with `.cmp(&other)`.

### 🎨 Output Styling

- Colorful output using `.red()` and `.green()` from `colored`.

---

## 📌 Key Rust Concepts Practiced

| Concept               | Example                          |
| --------------------- | -------------------------------- |
| Variable Binding      | `let secret_number = ...`        |
| Mutability            | `let mut guess = String::new();` |
| Error Handling        | `match guess.trim().parse()`     |
| Looping               | `loop {}`                        |
| Pattern Matching      | `match guess.cmp(...)`           |
| Modules/Imports       | `use rand::Rng;`                 |
| External Crates       | `rand`, `colored`                |
| Ownership & Borrowing | `&secret_number`                 |

---

## ✅ Summary

> You built an interactive CLI game that reinforces the fundamentals of Rust: ownership, pattern matching, error handling, control flow, and external crate usage.

---

Happy coding! 🦀✨
