# Temperature Converter

> Convert temperatures between Celsius and Fahrenheit.
>
> $$C = (F − 32) \times 5 / 9$$
> $$F = (C \times 9 / 5) + 32$$

The tools so far are sufficient to solve this exercise.
As something extra, we implement the `Display` trait for `TemperatureScale` enum to output it as strings:

```rust,noplayground
{{#include ../../code/chapter_3/src/bin/temperature_converter.rs:temperature_scale_enum}}

{{#include ../../code/chapter_3/src/bin/temperature_converter.rs:display_trait}}
```
