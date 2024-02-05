# Password Validation

> Write a simple program checking user inputs against a secret string.

The main logic checking user input is simple:

```rust,noplayground
{{#include ../../code/chapter_3/src/bin/password_validator.rs:check}}
```

It is a common practice to not show the user's password when reading it.
To do so, we can use the crate [`rpassword`](https://crates.io/crates/rpassword):

```rust,noplayground
{{#include ../../code/chapter_3/src/bin/password_validator.rs:read_password}}
```
