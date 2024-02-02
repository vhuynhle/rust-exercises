# Area of a Rectangular Room

> Read the length and width in feet of a rectangle room.
> Print the area of the room in both square feet and square meters.

This is a simple exercise to practice calculation.

The area in square feet is simply the product of the sizes in feet `length_ft`, `width_ft`.
To calculate the area in square meters, we can first convert the width and length to meter then multiply them.

```rust,noplayground
{{#include ../../code/chapter_2/src/bin/rect_area.rs:area_calc}}
```

where `FT_TO_M` is a constant conversion factor:
```rust,noplayground
{{#include ../../code/chapter_2/src/bin/rect_area.rs:constants}}
```
