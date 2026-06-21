use all_asserts::{
    assert_false, assert_ge, assert_gt, assert_le, assert_lt, assert_near, assert_nrange,
    assert_range, assert_true,
};

/*
    numbers
*/

// ge
#[test]
fn assert_ge_ok() {
    assert_ge!(5, 4);
    assert_ge!(4, 4);
}
#[test]
fn assert_ge_ok_msg() {
    assert_ge!(5, 4, "shouldn't print");
    assert_ge!(4, 4, "shouldn't print");
}
#[test]
#[should_panic]
fn assert_ge_fail() {
    assert_ge!(3, 4);
}
#[test]
#[should_panic = "that failed"]
fn assert_ge_fail_msg() {
    assert_ge!(3, 4, "that failed");
}

// gt
#[test]
fn assert_gt_ok() {
    assert_gt!(5, 4);
}
#[test]
fn assert_gt_ok_msg() {
    assert_gt!(5, 4, "shouldn't print");
}
#[test]
#[should_panic]
fn assert_gt_fail_1() {
    assert_gt!(4, 4);
}
#[test]
#[should_panic]
fn assert_gt_fail_2() {
    assert_gt!(3, 4);
}
#[test]
#[should_panic = "that failed"]
fn assert_gt_fail_msg() {
    assert_gt!(3, 4, "that failed");
}

// le
#[test]
fn assert_le_ok() {
    assert_le!(3, 4);
    assert_le!(3, 3);
}
#[test]
fn assert_le_ok_msg() {
    assert_le!(3, 4, "shouldn't print");
    assert_le!(3, 3, "shouldn't print");
}
#[test]
#[should_panic]
fn assert_le_fail() {
    assert_le!(5, 3);
}
#[test]
#[should_panic = "that failed"]
fn assert_le_fail_msg() {
    assert_le!(5, 3, "that failed");
}

// lt
#[test]
fn assert_lt_ok() {
    assert_lt!(4, 5);
}
#[test]
fn assert_lt_ok_msg() {
    assert_lt!(4, 5, "shouldn't print");
}
#[test]
#[should_panic]
fn assert_lt_fail_1() {
    assert_lt!(4, 4);
}
#[test]
#[should_panic]
fn assert_lt_fail_2() {
    assert_lt!(5, 4);
}
#[test]
#[should_panic = "5 < 4 is false"]
fn assert_lt_fail_msg() {
    assert_lt!(5, 4, "5 < 4 is false");
}

/*
    boolean
*/

#[test]
fn assert_false_ok() {
    assert_false!(false);
    assert_false!(10 > 20);
}
#[test]
#[should_panic]
fn assert_false_fail() {
    assert_false!(true);
}
#[test]
#[should_panic(expected = "a = 3, b = 27")]
fn assert_false_fail_msg() {
    let a = 3;
    let b = 27;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn assert_false_ok_msg() {
    let a = 3;
    let b = 28;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}

#[test]
fn assert_true_ok() {
    assert_true!(true);
    assert_true!(20 > 10);
}
#[test]
#[should_panic]
fn assert_true_fail() {
    assert_true!(false);
}
#[test]
#[should_panic(expected = "a = 3, b = 28")]
fn assert_true_fail_msg() {
    let a = 3;
    let b = 28;
    assert_true!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn assert_true_ok_msg() {
    let a = 3;
    let b = 27;
    assert_true!(a + b == 30, "a = {}, b = {}", a, b);
}

/*
    range
*/

// range
#[test]
fn assert_range_pass() {
    assert_range!(0..=10, 10);
    assert_range!(1.0..2.0, 1.5);
    assert_range!(1.0..2.0, 1.3);
}
#[test]
#[should_panic]
fn assert_range_fail() {
    assert_range!(0..10, 11);
    assert_range!(1.0..=2.0, 2.1);
}
#[test]
#[should_panic]
fn assert_range_fail_msg() {
    // Assert with a message
    assert_range!(
        1.0..=2.0,
        2.1,
        "Failed to assert that 2.1 is in the interval [1.0,2.0]"
    );
}

// nrange
#[test]
fn assert_nrange_pass() {
    assert_nrange!(10..20, 20);
    assert_nrange!(1.0..2.0, 2.0);
}
#[test]
#[should_panic]
fn assert_nrange_fail() {
    assert_nrange!(10..=20, 20);
    assert_nrange!(1.0..=2.0, 2.0);
}
#[test]
#[should_panic]
fn assert_nrange_fail_msg() {
    // Assert with a message
    assert_nrange!(1.0..2.0, 1.5, "Oops! 1.5 is in the interval [1.0,2.0)")
}

/*
    boundary
*/

#[test]
fn assert_near_pass() {
    assert_near!(0.0, 0.0, 0.1);
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON), f32::EPSILON);
    assert_near!(1.0f32, (1.0f32 - f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(
    expected = "assertion failed: `1.0000002 is not within epsilon 0.00000011920929 of 1"
)]
fn assert_near_fail_high() {
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON + f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(
    expected = "assertion failed: `0.99999976 is not within epsilon 0.00000011920929 of 1"
)]
fn assert_near_fail_low() {
    assert_near!(1.0f32, (1.0f32 - f32::EPSILON - f32::EPSILON), f32::EPSILON);
}
