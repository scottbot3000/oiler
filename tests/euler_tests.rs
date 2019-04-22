use oiler::euler::*;

#[test]
fn euler_1_test() {
    assert_eq!(euler_1(10), 23);
    assert_eq!(euler_1(1000), 233168);
}

#[test]
fn euler_1_iter_test() {
    assert_eq!(euler_1_iter(10), 23);
    assert_eq!(euler_1_iter(1000), 233168);
}

#[test]
fn euler_1_par_iter_test() {
    assert_eq!(euler_1_par_iter(10), 23);
    assert_eq!(euler_1_par_iter(1000), 233168);
}

#[test]
fn euler_1_loop_test() {
    assert_eq!(euler_1_loop(10), 23);
    assert_eq!(euler_1_loop(1000), 233168);
}