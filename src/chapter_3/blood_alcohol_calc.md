# Blood Alcohol Calculator

> Calculate the blood alcohol content (BAC) based on 5 inputs:
>
> - body weight,
> - gender,
> - number of drinks,
> - amount of alcohol by volume of the drink consumed, and
> - amount of time since the last drink.
>
> Finally, decide if it's legal to drive by compare the BAC with a fixed threshold (0.08).

The first step in solving this exercise is to read the input. We already have code to read
numbers, so the only new thing is to read the gender. We can read and store the gender as a string,
but it's better to have a dedicated data type and early validation.
Here we use an enum for gender and implement `FromStr` trait to parse it:

```rust,noplayground
{{#include ../../code/chapter_3/src/bin/blood_alcohol_calc.rs:parse_gender}}
```
The code above features the `FromStr` trait and matching on multiple alternatives using a vertical bar `|`.

After reading the input, it is straightforward to calculate the BAC.

```rust,noplayground
{{#include ../../code/chapter_3/src/bin/blood_alcohol_calc.rs:bac_formula}}
```
