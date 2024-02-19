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
    var!(depth, 5, "cube depth");
    let a = cube(width, depth, height).center();
    // currently calculations with vars fall back to numbers, but I plan on improving this
    // this means that this translate will not match the cube surface if the height is changed in openSCAD
    let b = cylinder(10, 5).translate(0, 0, height / 2);
    let c = a + b;
    print!("{}", c.render_scad());
}
```

produces this scad file:

```scad
// cube depth
depth = 5;
// cube height
height = 20;
// cube width
width = 10;
union(){
    cube([width,depth,height],center = true);
    translate([0,0,10]){
        cylinder(10, r = 5);
    }
}
```
