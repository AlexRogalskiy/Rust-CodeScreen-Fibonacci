extern crate fibonacci;
use fibonacci::calculate;

#[test]
fn test_1() {
    assert_eq!(calculate(14), 377);
}

#[test]
fn test_2() {
    assert_eq!(calculate(15), 610);
}

#[test]
fn test_3() {
    assert_eq!(calculate(18), 2584);
}

#[test]
fn test_4() {
    assert_eq!(calculate(19), 4181);
}
