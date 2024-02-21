#[cfg(test)]
use crate::*;
#[cfg(not(test))]
use solidrs::*;
pub fn render() -> String {
    var!(width, 10, "cube width");
    var!(height, 20, "cube height");
    let a = cube(width, width / 2, height).center();
    // you can save calculated points
    // they will still be displayed as the calculation when rendering
    let cube_top = height / 2;
    let b = cylinder(10, 5).translate(0, 0, cube_top);
    let c = a + b;
    c.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
