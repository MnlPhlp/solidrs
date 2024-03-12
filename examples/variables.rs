#[cfg(test)]
use crate::*;
#[cfg(not(test))]
#[allow(clippy::wildcard_imports)]
use solidrs::*;
pub fn render() -> String {
    var!(width, 10, "cube width");
    var!(height, 20, "cube height");
    let a = cube(width, width / 2, height).center();
    // you can save calculated points
    // they will still be displayed as the calculation when rendering.
    // this way changes to the variables in openscad apply everywhere
    let cube_top = height / 2;
    let cylinder_height = width / 2;
    let b = cylinder(cylinder_height, 5).translate(0, 0, cube_top);
    let c = a + b;
    c.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
