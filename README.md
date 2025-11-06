# 🦀 Rust Learning Path — From Go/Python/TS to Rust

Welcome! This repository contains hands-on examples and notes created while learning **Rust** as a developer with prior experience in **Golang, Python, and TypeScript**.

The goal is to build a practical understanding of Rust through small, focused examples demonstrating the language's principles, unique strengths, and mindset.

---

## ✅ Why Learn Rust?

Rust offers:

* **Memory safety without garbage collection**
* **Zero-cost abstractions** (performance comparable to C/C++)
* **Fearless concurrency**
* **Powerful type system + expressive enums & pattern matching**
* **Great tooling (Cargo, rustfmt, Clippy)**

This makes Rust particularly strong in:

* Systems programming
* Performance-critical services
* Embedded & OS development
* WebAssembly
* High-performance backend development

---

## 📘 Learning Approach

This repository documents learning through:
✅ Minimal runnable examples
✅ Layered explanations
✅ Comparison with Go/Python/TypeScript
✅ Notes on compiler errors and how to fix them

Each topic is broken down into small examples.

---

## 🔥 Core Examples

### ✅ 1) Ownership & Borrowing

Learn how Rust manages memory *without a GC*.

* Move semantics
* Immutable & mutable borrows
* Borrow checker guarantees

### ✅ 2) Enums + Pattern Matching

Expressive type modeling via algebraic data types.

### ✅ 3) Traits + Generics

Static polymorphism with zero runtime cost.

### ✅ 4) Lifetimes

How Rust tracks reference validity at compile time.

### ✅ 5) Fearless Concurrency

Compile-time race prevention with threads and messaging channels.

> See the `examples/` folder for runnable snippets.

---

## 🚀 Getting Started

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Create new project
cargo new rust-learning
cd rust-learning
cargo run
```

---

## 📂 Project Structure

```
.
├── examples/
│   ├── ownership.rs
│   ├── enums_match.rs
│   ├── traits_generics.rs
│   ├── lifetimes.rs
│   └── concurrency.rs
└── README.md
```

---

## 🧠 Tips

* Compiler errors are your friend → Rust teaches you
* Prefer returning owned values to avoid lifetime pain
* Focus first on ownership/borrowing; lifetimes become natural
* Clippy can guide idiomatic Rust style:

```bash
cargo clippy
```

---

## 🔄 Comparison Notes

| Concept      | Rust              | Go         | Python      | TypeScript |
| ------------ | ----------------- | ---------- | ----------- | ---------- |
| Memory       | Ownership         | GC         | GC          | GC         |
| Generics     | Monomorphized     | Native     | runtime     | erased     |
| Polymorphism | Traits            | Interfaces | Duck typing | Structural |
| Concurrency  | Compile-time safe | Goroutines | GIL         | Event-loop |

---

## ✅ Next Steps

* Write unit tests
* Explore Result & error handling
* Look at async (Tokio)
* Build a small CLI
* Try web frameworks (Axum)

---

## 📝 Notes

This repo is continuously updated as learning progresses.

Feel free to fork or open Issues/PRs if you are learning along.

---

## 📜 License

MIT

---

Happy hacking! 🦀

