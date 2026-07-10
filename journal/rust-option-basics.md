Rust Option Primer - notes
==========================

Core idea
---------
Option<T> is Rust's "nullable value, but safe".
In C terms: it's an Optional, a std::optional, or a tagged union where you explicitly
handle "has value" vs "empty" instead of silently dereferencing NULL.

Why Option exists
-----------------
- No null pointers by default in Rust
- Forces caller to handle the empty/no-value case
- Compiler tracks whether you've handled it

Shapes
------
None          → there is no value (like NULL / empty slot)
Some(value)   → there is a value of type T

Compare:
C embedded:
  - GPIO read: returns int, or -1 on error → easy to forget to check -1
  - Better: return struct or enum with valid vs invalid

Rust:
  Option<i32> either has a real i32 or None

Option is an enum
-----------------
pub enum Option<T> {
    None,
    Some(T),
}

if let
------
Syntax: if let Pattern(value) = expression { ... }
Meaning: "if expression matches Pattern, bind the inner value and run the block"

Think of it as a safe, single-case match.

Example:
  let x = Some("hello");

  if let Some(word) = x {
      // word is "hello" here
  }
  else {
      // Option was None
  }

Equivalent to:
  match x {
      Some(word) => { ... }
      None => { ... }
  }

while let
---------
Like if let, but repeats while the pattern keeps matching.

Example from your code:
  while let Some(Some(integer)) = optional_integers.pop() {
      // runs only when BOTH Option layers are Some
  }

Each iteration:
  optional_integers.pop() returns Option<Option<i8>>
  1st Some unwraps the Vec pop result
  2nd Some unwraps the inner integer value

unwrap
------
Dangerous but concise:

  let x: Option<i32> = Some(5);
  let y = x.unwrap();  // y == 5, panics if x is None

Think of it as: "I promise this is Some and I want the raw value now."
In C terms: dereferencing without a NULL check.

If you unwrap() a None, the program panics (like a failed assert).
That's why rustlings wants you to use pattern matching instead of
relying on unwrap.

Layered options
---------------
When you have Vec<Option<i8>>, calling pop() returns Option<Option<i8>>.

Visual:
  optional_integers.pop()
  └─ None                       → vec was empty, stop
  └─ Some(inner)
       └─ None                  → skip this entry
       └─ Some(integer)         → process it

Pattern matching both layers:
  while let Some(Some(integer)) = optional_integers.pop()

This rejects both None cases in one step.

Key rule
--------
- Use pattern matching (if let / match / while let) when None is realistic
- unwrap() only when a None would mean a bug you want to crash on
