# Self-Checkout

> Create a simple self-checkout system.
> Prompt for the prices and quantities of several items.
> Calculate the subtotal, tax, and total using a tax rate of 5.5%.

This is a simple exercise. The main challenge is to decide when to stop reading user input.
Here we use `match` clause to check for invalid input and stop:

```rust
{{#include ../../code/chapter_2/src/bin/self_checkout.rs:read_loop}}
```
