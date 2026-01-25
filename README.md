# Expression Solver

A **minimal expression language**, compiler, and stack-based virtual machine written in **Rust**.

This project parses high-level mathematical and logical expressions, compiles them into custom bytecode, and executes them on a purpose-built VM. The design is inspired by **LISP-style expressions**, **stack machines**, and classic compiler architecture.

> Repository: [https://github.com/omniflare/expression-solver](https://github.com/omniflare/expression-solver)

> Author: **omniflare**

---

## Features :

* Custom expression language
* Lexer → Parser → AST → Compiler → VM -> Result
* Stack-based virtual machine
* Register-based variable storage
* Lexical scoping via `define`
* Arithmetic and comparison operators
* Conditional execution (`if`)
* Deterministic bytecode execution
* Designed to be **Turing complete**

---

## Flow of computation

```
Source File
   ↓
Lexer (tokens)
   ↓
Parser (AST)
   ↓
Compiler (bytecode)
   ↓
Virtual Machine (execution)
```

Each stage is isolated and explicit, making the system easy to extend and reason about.

---

## 🔍 Language Overview

### Expression-based language

Everything is an expression and produces a value, including:

* arithmetic
* variable definitions
* conditionals

There are no statements — only expressions.

---

## Current Supported Tokens

### Literals

* Integer numbers (e.g. `5`, `42`, `-10`)

### Identifiers

* Variable names (`x`, `y`, `result`, `_tmp`)

### Keywords

* `define`
* `if`

### Arithmetic Operators

* `+`  (addition)
* `-`  (subtraction, unary negation)
* `*`  (multiplication)
* `/`  (division)
* `%`  (modulus)
* `**` (exponent)
* `//` (floor div)

### Comparison Operators

* `==` (equal)
* `!=` (not equal)
* `<`  (less than)
* `>`  (greater than)
* `<=` (less than or equal)
* `>=` (greater than or equal)

### Delimiters

* `(` `)` for grouping and structure

---

##  Parser

* **Type:** Recursive-descent parser
* **Precedence-aware**
* Handles:

  * unary expressions
  * binary expressions
  * nested expressions
  * lexical scoping
  * conditional branching

Grammar is expression-first and LISP-inspired.

---

##  AST (Abstract Syntax Tree)

Key expression types:

* `Number`
* `Variable`
* `Define` (lexically scoped variable binding)
* `Unary`
* `Binary`
* `If`

The AST represents *meaning*, not execution.

---

## 🧮 Compiler

The compiler traverses the AST and emits **linear bytecode** for the VM.

### Responsibilities:

* Stack discipline
* Register allocation for variables
* Scope tracking
* Jump target patching
* Control-flow lowering (`if` → jumps)

The compiler does **not evaluate expressions** — it only arranges instructions.

---

## 🖥️ Virtual Machine

### Architecture

* **Stack-based execution**
* **Register file** for variables
* Explicit instruction pointer (`IP`)
* Stack pointer (`SP`)
* Deterministic execution model

### Core Components

* Operand stack
* Register array
* Instruction pointer
* Bytecode program memory

---

## Supported VM Instructions

### Stack Operations

* `PSH` – push value
* `POP` – pop value

### Arithmetic

* `ADD`
* `SUB`
* `MUL`
* `DIV`

### Comparisons (push `1` for true, `0` for false)

* `EQ`, `NE`
* `LT`, `GT`
* `LE`, `GE`

### Registers

* `SET` – store stack value into register
* `GET` – load register value onto stack

### Control Flow

* `JZ`  – jump if zero (false)
* `JMP` – unconditional jump

### Program Control

* `HLT` – halt execution

---

## Execution Model

* All expressions leave their result on the stack
* Binary operators pop two values and push one
* Comparisons produce boolean integers (`0` or `1`)
* Control flow manipulates the instruction pointer
* Variables live in registers, not on the stack

---

## 🧪 Example Programs

### Variable Binding

```text
define (x 5
  x + 3
)
```

### Nested Scopes

```text
define (x 5
  define (y 3
    x - 2*y
  )
)
```

### Conditional Expression

```text
if (x > 10)
  x
  x * 2
```

### Conditional with Variables

```text
define (x 4
  if (x < 5)
    100
    200
)
```

---

## Extensibility

The system is designed to grow. Possible extensions include:

* User-defined functions (upcoming next )
* Recursion (along with functions)
* Memory model improvements
* Garbage collection (most likely this will be next)
* Bytecode optimizer
* Debugger / tracer

---

##  Author

**omniflare**

GitHub: [https://github.com/omniflare](https://github.com/omniflare)
