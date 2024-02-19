# SolidRS

Rust library to generate openScad models. Inspired by SolidPython.

Currently only very few commands are implemented but I plan on adding them as I need them.

## Example

```rust
use solidrs::*;

fn main() {
    let a = cube(10, 20, 5).center();
    let b = cylinder(10, 5).translate(0, 0, 2.5);
    let c = a + b;
    c.save_as_scad("test");
}
```
