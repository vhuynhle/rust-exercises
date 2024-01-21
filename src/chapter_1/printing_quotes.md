# Printing Quotes

## Challenge

> Prompt for a quote and an author's name.
> Display the author's name and then the quote inside quotation marks.

## Solution

The main purpose of this exercise is to create strings containing [character escapes](https://doc.rust-lang.org/reference/tokens.html#character-escapes). Knowing character
escapes, the solution is simple:

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/char_escapes.rs}}
```

Sample output:

```
What is the quote?
51% of quotes on the Internet are fake.
Who said it?
Me
Me says, "51% of quotes on the Internet are fake."
```

Of course, there are other escape sequences, for instance:

```rust
println!("\"\nI\u{2764}\u{FE0F}\u{1F1FB}\u{1F1F3}\r\"")
```
This code snippet prints

```
"
I❤️🇻🇳
"
```
