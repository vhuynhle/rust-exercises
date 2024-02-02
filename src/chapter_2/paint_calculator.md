# Paint Calculator

> Given the size of a room and the amount of paint needed to paint a square feet of ceiling,
> calculate the total amount of paint needed.
> You can only buy a whole number of gallons of paint.

In this exercise, we need to perform rounding up a number instead of rounding down.
The calculation itself is easy:

```rust
{{#include ../../code/chapter_2/src/bin/paint_calculator.rs:calc}}
```

The remaining part is to print the result:

```rust
{{#include ../../code/chapter_2/src/bin/paint_calculator.rs:print}}
```

And here are the functions to pluralize a word:

```rust
{{#include ../../code/chapter_2/src/bin/paint_calculator.rs:pluralize}}
```

```rust
{{#include ../../code/chapter_2/src/bin/paint_calculator.rs:find_noun_form}}
```

Notice how we used trait to write a generic `find_noun_fom` function that accepts both `u64` and `f64`
as the first parameter.
In practice, we could use the `num` crate instead.
