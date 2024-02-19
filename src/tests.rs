use crate::*;

#[test]
fn simple_example() {
    let a = cube(10, 20, 5).center();
    let b = cylinder(10, 5).translate(0, 0, 2.5);
    let c = a + b;
    let expected = include_str!("../test_output/simple.scad");
    assert_eq!(expected, c.render_scad())
}

#[test]
fn variables_example() {
    var!(width, 10, "cube width");
    var!(height, 20, "cube height");
    var!(depth, 5, "cube depth");
    let a = cube(width, depth, height).center();
    // currently calculations with vars fall back to numbers, but I plan on improving this
    // this means that this translate will not match the cube surface if the height is changed in openSCAD
    let b = cylinder(10, 5).translate(0, 0, height / 2);
    let c = a + b;
    let expected = include_str!("../test_output/variables.scad");
    assert_eq!(expected, c.render_scad())
}
