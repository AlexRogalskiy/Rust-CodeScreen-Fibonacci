extern crate fibonacci;
use fibonacci::calculate;

#[test]
fn test_1() {
    assert_eq!(calculate(0), 0);
}

#[test]
fn test_2() {
    assert_eq!(calculate(1), 1);
}

#[test]
fn test_3() {
    assert_eq!(calculate(20), 6765);
}

#[test]
fn test_4() {
    assert_eq!(calculate(25), 75025);
}
