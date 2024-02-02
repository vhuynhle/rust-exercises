# Retirement Calculator

## Challenge

> Given the number of years that a user need to work, calculate their retirement year.

## Solution

The main point of this challenge is to learn to work with date/time.
In particular, we need to get the current year.

Unfortunately, the standard library does not provide such a function.
We can use the `chrono` crate instead:

```bash
cargo add chrono
```

And now getting the current (UTC) year is simple:

```rust
use chrono::{prelude::Utc, Datelike};
let current_utc_year = Utc::now().year();
```

The crate also supports local time:

```
use chrono::{prelude::Datelike, Local};
let current_local_year = Local::now().year();
```
