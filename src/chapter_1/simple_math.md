# Simple Math

## Challenge

> Read two numbers from stdin and compute their sum, difference, product, and quotient.

## Solution

In previous challenges, we have learned how to read strings from stdin. Here we make a function for
that:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/simple_math.rs:read_line}}
```

The next step is to convert input strings into integers. Luckily that can be done with the built-in
function `str::parse()`.

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/simple_math.rs:parse_num_1}}
```
The second number can be parsed similarly.

The final step -- computing the two numbers' sum, difference, product, and quotients -- is
straightforward with built-in operators:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/simple_math.rs:simple_math}}
```
