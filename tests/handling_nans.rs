use all_asserts::{
    assert_ge, assert_gt, assert_le, assert_lt, assert_near, assert_nrange, assert_range,
};

#[test]
#[should_panic]
fn panic_when_nan_gt_1() {
    assert_gt!(f32::NAN, 4.);
}
#[test]
#[should_panic]
fn panic_when_nan_gt_2() {
    assert_gt!(4., f32::NAN);
}

#[test]
#[should_panic]
fn panic_when_nan_ge_1() {
    assert_ge!(f32::NAN, 4.);
}
#[test]
#[should_panic]
fn panic_when_nan_ge_2() {
    assert_ge!(4., f32::NAN);
}

#[test]
#[should_panic]
fn panic_when_nan_lt_1() {
    assert_lt!(f32::NAN, 4.);
}
#[test]
#[should_panic]
fn panic_when_nan_lt_2() {
    assert_lt!(4., f32::NAN);
}

#[test]
#[should_panic]
fn panic_when_nan_le_1() {
    assert_le!(f32::NAN, 4.);
}
#[test]
#[should_panic]
fn panic_when_nan_le_2() {
    assert_le!(4., f32::NAN);
}

#[test]
#[should_panic]
fn panic_when_nan_range_1() {
    let a = f32::NAN;
    let b = 3.0;
    assert_range!(a..b, 3.0);
}

#[test]
#[should_panic]
fn panic_when_nan_range_2() {
    let a = 3.0;
    let b = f32::NAN;
    assert_range!(a..b, 3.0);
}

#[test]
#[should_panic]
fn panic_when_nan_range_3() {
    assert_range!(1.0..=3.0, f32::NAN);
}

#[test]
fn panic_when_nan_nrange_1() {
    let a = f32::NAN;
    let b = 3.0;
    let c = 3.0;
    // being inside of range means a<=c && c<b. The a<=c condition is false,
    // therefore c is not in a..b and the test for nrange should pass.
    assert_nrange!(a..b, c);
}

#[test]
fn panic_when_nan_nrange_2() {
    let a = 3.0;
    let b = f32::NAN;
    // being inside of range means a<=c && c<b. The c<b condition is false,
    // therefore c is not in a..b and the test for nrange should pass.
    assert_nrange!(a..b, 3.0);
}

#[test]
fn panic_when_nan_nrange_3() {
    assert_nrange!(1.0..=3.0, f32::NAN);
}

#[test]
#[should_panic]
fn panic_when_nan_near_1() {
    assert_near!(f32::NAN, 0.0, 0.1);
}

#[test]
#[should_panic]
fn panic_when_nan_near_2() {
    assert_near!(0.0, f32::NAN, 0.1);
}

#[test]
#[should_panic]
fn panic_when_nan_near_3() {
    assert_near!(0.0, 0.0, f32::NAN);
}
