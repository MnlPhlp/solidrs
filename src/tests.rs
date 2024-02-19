#[test]
fn simple_example() {
    let expected = include_str!("../test_output/simple.scad");
    let actual = crate::examples::simple::render();
    assert_eq!(expected, actual);
}

#[test]
fn variables_example() {
    let expected = include_str!("../test_output/variables.scad");
    let actual = crate::examples::variables::render();
    assert_eq!(expected, actual);
}

#[test]
fn car_example() {
    let expected = include_str!("../test_output/car.scad");
    let actual = crate::examples::car::render();
    assert_eq!(expected, actual);
}
