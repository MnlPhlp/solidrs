#[cfg(test)]
use crate::*;
#[cfg(not(test))]
#[allow(clippy::wildcard_imports)]
use solidrs::*;

#[must_use]
pub fn render() -> String {
    let a = cube(10, 20, 5).center();
    let b = cylinder(10, 5).translate(0, 0, 2.5);
    let c = a + b;
    c.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
