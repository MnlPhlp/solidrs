#[cfg(test)]
use crate::*;
#[cfg(not(test))]
#[allow(clippy::wildcard_imports)]
use solidrs::*;
#[must_use]
pub fn render() -> String {
    var!(WIDTH, 10, "cube width");
    var!(HEIGHT, 20, "cube height");
    let a = cube(WIDTH, WIDTH / 2, HEIGHT).center();
    // you can save calculated points
    // they will still be displayed as the calculation when rendering.
    // this way changes to the variables in openscad apply everywhere
    let cube_top = HEIGHT / 2;
    // if you use calc! the calculation will be stored to a variable in openscad
    calc!(CYLINDER_HEIGHT, WIDTH / 2);
    let b = cylinder(CYLINDER_HEIGHT, 5).translate(0, 0, cube_top);
    let c = a + b;
    c.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
