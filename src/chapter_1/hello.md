# Hello, World!

## Challenge

> Ask the user for their name and greet them.

## Solution
This is a variation of the classic "Hello, World" program. Instead of just printing out a fixed string,
we to read a string use it in our greeting:

```
What is your name?
Huynh
Hello, Huynh!
```

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/hello.rs}}
```
An interesting Rust feature used here is [variable shadowing](https://en.wikipedia.org/wiki/Variable_shadowing),
where the second variable `name` shadows the first variable of the same name. Also, note that
the compiler guarantees that the second variable is not a
[dangling reference](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references).

## Variation 1
In this variation, no variable is allowed. Therefore, we cannot no longer use `Stdin::read_line()`,
which writes to an existing buffer.
Instead, we can use `Stdin::lines()`, which returns an iterator over input lines.

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/hello_v1.rs:5:12}}
```

Doing multiple things in one statement like this maybe a bad practice, but we did solve the challenge.

## Variation 2
Now, we want different greetings for different people. The first thing we need to add is a list of
greetings to choose from. Here is a short list:[^1]

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/hello_v2.rs:phrasebook}}
```

As there are no further requirements, we can choose among several equally acceptable approaches, for example:
- Choose a random greeting every time.
- Choose the greeting based on the initials.
- Choose the greeting based on a hash of the name.

Let's go with the first approach. To make this easier, we shall depend on the [rand](https://crates.io/crates/rand)
crate to generate random numbers:

```toml
# File: Cargo.toml
[dependencies]
rand = "0.8.5"
```

```rust,noplayground
// File: hello_v2.rs
use rand::{thread_rng, Rng};
```

And here is the code to print randomized greetings:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/hello_v2.rs:randomized_greetings}}
```

[^1]: As a non-native English speaker, I'm always amazed by the variety of ways to say "Hello" in English.
For example, this blog documents ["107 interesting and different ways to say hi in English"](https://www.berlitz.com/blog/hello-in-english).
