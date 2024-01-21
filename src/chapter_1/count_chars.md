# Counting the Number of Characters

## Challenge

> Create a program that reads a string, then echoes back the string together with its length.

## Solution

We already know how to read a string from user input from the previous challenge.
The remaining question is how to calculate its length.
Intuitively, the length of a string is the number of characters that it has.
But what does it mean by a *character*?


### The lengths of ASCII strings
For ASCII strings, each character is represented by a 1-byte code point.
The length of a string is the same as the number of bytes used to store the string.

### The lengths of UTF-8 strings
For Unicode strings, the answer is more complex. Consider the emoji '🤦🏼‍♂️'. It is one graphical unit
represented by 17 bytes:[^1]

| *Scalar name*                             | *Printed scalar* | *Code units* |
|-------------------------------------------|------------------|--------------|
| U+1F926 Face Palm                         | 🤦               | 4            |
| U+1F3FC Emoji Modifier Fitzpatrick Type-3 | 🏼               | 4            |
| U+200D Zero Width Joiner                  | not printable    | 3            |
| U+2642 Male Sign                          | ♂                | 3            |
| U+FE0F Variation Selector-16              | ◌️                | 3            |
|                                           |                  | Total: 17    |

Here we are presented with several terms:
- UTF-8 code unit: 8 bits/1 byte
- Unicode code point: A position in the Unicode character set. In the UTF-8 encoding, each code point
is represented by one to four code units.
- Unicode scalar value: A *Unicode code point* except high-surrogate and low-surrogate code points.
In other words, the set of scalar values is a subset of code points.
- Extended grapheme cluster: The whole emoji is an extended grapheme cluster.

Therefore, when it comes to a UTF-8 string, we have multiple *lengths* measurements.
For the string "🤦🏼‍♂️":
- The number of bytes used to encode it: 17,
- The number of Unicode scalar values/code points: 5,
- The number of extended grapheme clusters: 1.

### The Rust code

To count the number of code units and scalar values, we can use the built-in functions `str::len()`
and `str::chars()`. For the number of extended grapheme clusters, we shall use the crate [unic_segment](https://docs.rs/unic-segment/latest/unic_segment/) instead of implementing our own
segmentation algorithm.

```rust,noplayground
{{#include ../../code/chapter_1/src/bin/strlen.rs:compute_length}}
```

Example output:
```
Enter a string:
🤦🏼‍♂️🤦🏼
Your string '🤦🏼‍♂️🤦🏼' has:
        25 byte(s)
        7 scalar values
        2 extended grapheme cluster(s).
```

[^1] [It's Not Wrong that "🤦🏼‍♂️".length == 7](https://hsivonen.fi/string-length/), Henri Sivonen.
