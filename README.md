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

## Variables

SolidRS is able to generate variables in the resulting openSCAD script.
These variables can then be configured in the openSCAD Editor.

this rust file:

```rust
use solidrs::*;

fn main() {
    var!(width, 10, "cube width");
    var!(height, 20, "cube height");
    let a = cube(width, width / 2, height).center();
    // you can save calculated points
    // they will still be displayed as the calculation when rendering
    let cube_top = height / 2;
    // if you use var to save it, the calculation will also be written to a variable in openScad
    var!(cylinder_height, width / 2);
    let b = cylinder(cylinder_height, 5).translate(0, 0, cube_top);
    let c = a + b;
    c.save_as_scad("vars");
}
```

produces this scad file:

```scad
// cube width
width = 10;
// cube height
height = 20;
cylinder_height = width / 2;
union(){
    cube([width,width / 2,height],center = true);
    translate([0,0,height / 2]){
        cylinder(cylinder_height, r = 5);
    }
}
```
