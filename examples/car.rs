#[cfg(test)]
use crate::*;
#[cfg(not(test))]
#[allow(clippy::wildcard_imports)]
use solidrs::*;

// this is derived from the code at the end of
// chapter one of the OpenSCAD wiki tutoril
// https://en.wikibooks.org/wiki/OpenSCAD_Tutorial/Chapter_1

#[must_use]
pub fn render() -> String {
    let mut car = cube(60, 20, 10).center();
    car += cube(30, 20, 10).center().translate(5, 0, 10. - 0.001);
    let tire = cylinder(3, 8).center().rotate(90, 0, 0);
    car += tire.clone().translate(-20, -15, 0);
    car += tire.clone().translate(-20, 15, 0);
    car += tire.clone().translate(20, -15, 0);
    car += tire.translate(20, 15, 0);
    let axle = cylinder(30, 2).center().rotate(90, 0, 0);
    car += axle.clone().translate(-20, 0, 0);
    car += axle.translate(20, 0, 0);
    car = car.set_fa(1).set_fs(0.4);
    car.render_scad()
}

#[cfg(not(test))]
fn main() {
    print!("{}", render());
}
