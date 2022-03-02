use all_asserts::{
    assert_false, assert_ge, assert_gt, assert_le, assert_lt, assert_near, assert_nrange,
    assert_range, assert_true,
};

#[test]
#[should_panic]
fn panic_when_not_gt() {
    assert_gt!(4, 4);
}
#[test]
#[should_panic]
fn panic_when_not_ge() {
    assert_ge!(3, 4);
}
#[test]
#[should_panic]
fn panic_when_not_lt() {
    assert_lt!(4, 4);
}
#[test]
#[should_panic]
fn panic_when_not_le() {
    assert_le!(4, 3);
}
#[test]
fn test_assert_range_pass() {
    assert_range!(0..=10, 10);
    assert_range!(1.0..2.0, 1.5);
    assert_range!(1.0..2.0, 1.3);
}
#[test]
#[should_panic]
fn test_assert_range_fail() {
    assert_range!(0..10, 11);
    assert_range!(1.0..=2.0, 2.1);
}
#[test]
fn test_assert_nrange_pass() {
    assert_nrange!(10..20, 20);
    assert_nrange!(1.0..2.0, 2.0);
}
#[test]
#[should_panic]
fn test_assert_nrange_fail() {
    assert_nrange!(10..=20, 20);
    assert_nrange!(1.0..=2.0, 2.0);
}
#[test]
#[should_panic]
fn test_assert_range_with_fail_msg() {
    // Assert with a message
    assert_range!(
        1.0..=2.0,
        2.1,
        "Failed to assert that 2.1 is in the interval [1.0,2.0]"
    );
}
#[test]
#[should_panic]
fn test_assert_nrange_with_fail_msg() {
    // Assert with a message
    assert_nrange!(1.0..2.0, 1.5, "Oops! 1.5 is in the interval [1.0,2.0)")
}
#[test]
fn test_assert_near_pass() {
    assert_near!(0.0, 0.0, 0.1);
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON), f32::EPSILON);
    assert_near!(1.0f32, (1.0f32 - f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(
    expected = "assertion failed: `1.0000002 is not within epsilon 0.00000011920929 of 1"
)]
fn test_assert_near_fail_high() {
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON + f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(
    expected = "assertion failed: `0.99999976 is not within epsilon 0.00000011920929 of 1"
)]
fn test_assert_near_fail_low() {
    assert_near!(1.0f32, (1.0f32 - f32::EPSILON - f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic]
fn panic_when_not_true() {
    assert_true!(false);
}
#[test]
fn do_not_panic_when_true() {
    assert_true!(true);
}
#[test]
#[should_panic(expected = "a = 3, b = 28")]
fn panic_when_not_true_including_message() {
    let a = 3;
    let b = 28;
    assert_true!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn do_not_panic_when_true_and_do_not_emit_message() {
    let a = 3;
    let b = 27;
    assert_true!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
#[should_panic]
fn panic_when_not_false() {
    assert_false!(true);
}
#[test]
fn do_not_panic_when_false() {
    assert_false!(false);
}
#[test]
#[should_panic(expected = "a = 3, b = 27")]
fn panic_when_not_false_including_message() {
    let a = 3;
    let b = 27;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn do_not_panic_when_false_and_do_not_emit_message() {
    let a = 3;
    let b = 28;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}
