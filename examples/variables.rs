#[cfg(test)]
use crate::*;
#[cfg(not(test))]
use solidrs::*;
pub fn render() -> String {
    var!(width, 10, "cube width");
    var!(height, 20, "cube height");
    var!(depth, 5, "cube depth");
    let a = cube(width, depth, height).center();
    // currently calculations with vars fall back to numbers, but I plan on improving this
    // this means that this translate will not match the cube surface if the height is changed in openSCAD
    let b = cylinder(10, 5).translate(0, 0, height / 2);
    let c = a + b;
    c.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
