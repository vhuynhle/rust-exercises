# Currency Conversion

> Convert from Euros to US dollars.
> Ensure that fractions of a cent are rounded up to the next penny.

The calculation is simple. To ensure that fractions are rounded up, we can use
the function `f64::ceil`.

```rust,noplayground
{{#include ../../code/chapter_2/src/bin/currency_conversion.rs:calc}}
```
