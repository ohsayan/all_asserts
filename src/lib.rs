//! # `all_asserts`
//!
//! The `all_asserts` crate provides multiple assert macros which aren't there in the
//! standard library. These macros provide extremely useful outputs and can be used for
//! writing better tests.

/// Asserts that the left hand side expression is greater
/// than the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = 100; let b = 100;
/// #[cfg(should_panic)]
/// assert_gt!(a, b);
/// ```
///
#[macro_export]
macro_rules! assert_gt {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    panic!(r#"assertion failed: `(left > right) but here (left <= right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_gt!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    panic!(r#"assertion failed: `(left > right) but here (left <= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                    }
                }
            }
        });
    }

/// This is a debug-only variant of the [`assert_gt`] macro
///
/// [`assert_gt`]: ./macro.assert_gt.html
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_gt!($($arg)*); })
}
/// Asserts that the left hand side expression is greater
/// than or equal to the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = 200; let b = 100;
/// #[cfg(should_panic)]
/// assert_ge!(a, b);
/// ```
///
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    panic!(r#"assertion failed: `(left >= right) but here (left < right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_ge!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    panic!(r#"assertion failed: `(left >= right) but here (left < right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_ge`] macro
///
/// [`assert_ge`]: ./macro.assert_ge.html
#[macro_export]
macro_rules! debug_assert_ge {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_ge!($($arg)*); })
}

/// Asserts that the left hand side expression is lesser
/// than the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = 100; let b = 100;
/// #[cfg(should_panic)]
/// assert_lt!(a, b); // This would panic
/// ```
///
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    panic!(r#"assertion failed: `(left < right) but here (left >= right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_lt!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    panic!(r#"assertion failed: `(left < right) but here (left >= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_lt`] macro
///
/// [`assert_lt`]: ./macro.assert_lt.html
#[macro_export]
macro_rules! debug_assert_lt {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_lt!($($arg)*); })
}

/// Asserts that the left hand side expression is lesser than
/// or equal to the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = 200; let b = 100;
/// #[cfg(should_panic)]
/// assert_le!(a, b); // This would panic
/// ```
///
#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    panic!(r#"assertion failed: `(left <= right) but here (left > right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_le!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    panic!(r#"assertion failed: `(left <= right) but here (left > right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_le`] macro
///
/// [`assert_le`]: ./macro.assert_le.html
#[macro_export]
macro_rules! debug_assert_le {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_le!($($arg)*); })
}

/// Asserts that the right hand side expression is
/// within the range on the left hand side
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts::assert_range;
/// assert_range!((100..200), 101);
/// ```
///
#[macro_export]
macro_rules! assert_range {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val.contains(right_val)) {
                    panic!(
                        r#"assertion failed: `{} is not in range of {:?}` - it should have been in this range"#,
                        right_val,
                        left_val
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_range!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(left_val.contains(right_val)) {
                    panic!(
                        r#"assertion failed: `{} is not in range of {:?}` - it should have been in this range: {}"#,
                        right_val,
                        left_val,
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_range`] macro
///
/// [`assert_range`]: ./macro.assert_range.html
#[macro_export]
macro_rules! debug_assert_range {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_range!($($arg)*); })
}

/// Asserts that the right hand side expression is not
/// within the range on the left hand side
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts::assert_nrange;
/// assert_nrange!((100..200), 1000);
/// ```
///
#[macro_export]
macro_rules! assert_nrange {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val.contains(right_val)) {
                    panic!(
                        r#"assertion failed: `{} is in range of {:?}` - it should not have been in this range"#,
                        right_val,
                        left_val
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr,) => {
        assert_nrange!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val.contains(right_val)) {
                    panic!(
                        r#"assertion failed: `{} is in range of {:?}` - it should not have been in this range: {}"#,
                        right_val,
                        left_val,
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_nrange`] macro
///
/// [`assert_nrange`]: ./macro.assert_nrange.html
#[macro_export]
macro_rules! debug_assert_nrange {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_nrange!($($arg)*); })
}

/// Asserts that the right hand side expression is
/// within the range on the left hand side
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts::assert_near;
/// assert_near!(100.0, 100.9, 1.0);
/// assert_near!(100.9, 100.0, 1.0);
/// ```
///
#[macro_export]
macro_rules! assert_near {
    ($left:expr, $right:expr, $epsilon:expr) => ({
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon_val) => {
                if (*left_val > (*right_val + *epsilon_val)) || (*left_val < (*right_val - *epsilon_val)) {
                    panic!(
                        r#"assertion failed: `{} is not within epsilon {} of {}`"#,
                        right_val,
                        epsilon_val,
                        left_val
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $epsilon:expr,) => {
        assert_near!($left, $right, $epsilon)
    };
    ($left:expr, $right:expr, $epsilon:expr, $($arg:tt)+) => ({
        match (&($left), &($right), &($epsilon)) {
            (left_val, right_val, epsilon_val) => {
                if (*left_val > (*right_val + *epsilon_val)) || (*left_val < (*right_val - *epsilon_val)) {
                    panic!(
                        r#"assertion failed: `{} is not within epsilon {} of {}`"#,
                        right_val,
                        epsilon_val,
                        left_val,
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// This is a debug-only variant of the [`assert_near`] macro
///
/// [`assert_near`]: ./macro.assert_near.html
#[macro_export]
macro_rules! debug_assert_near {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_near!($($arg)*); })
}

/// Asserts that the expression is true
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = false;
/// #[cfg(should_panic)]
/// assert_true!(a);
///
/// ```
/// # #[macro_use] use all_asserts;
/// let a = true;
/// assert_true!(a);
/// ```
///
#[macro_export]
macro_rules! assert_true {
    ($cond:expr $(,)?) => ({
        assert!($cond)
    });
    ($cond:expr,) => {
        assert!($cond)
    };
    ($cond:expr, $($arg:tt)+) => ({
        assert!($cond, $($arg)+)
    });
}

/// This is a debug-only variant of the [`assert_true`] macro
///
/// [`assert_true`]: ./macro.assert_true.html
#[macro_export]
macro_rules! debug_assert_true {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_true!($($arg)*); })
}

/// Asserts that the expression is false
///
/// # Examples
/// ```
/// # #[macro_use] use all_asserts;
/// let a = true;
/// #[cfg(should_panic)]
/// assert_false!(a);
///
/// ```
/// # #[macro_use] use all_asserts;
/// let a = false;
/// assert_false!(a);
/// ```
///
#[macro_export]
macro_rules! assert_false {
    ($cond:expr $(,)?) => ({
        assert!(!$cond)
    });
    ($cond:expr,) => {
        assert!(!$cond)
    };
    ($cond:expr, $($arg:tt)+) => ({
        assert!(!$cond, $($arg)+)
    });
}

/// This is a debug-only variant of the [`assert_false`] macro
///
/// [`assert_false`]: ./macro.assert_false.html
#[macro_export]
macro_rules! debug_assert_false {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_false!($($arg)*); })
}

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
    assert_range!((0..=10), 10);
    assert_range!((1.0..2.0), 1.5);
    assert_range!((1.0..2.0), 1.3);
}
#[test]
#[should_panic]
fn test_assert_range_fail() {
    assert_range!((0..10), 11);
    assert_range!((1.0..=2.0), 2.1);
}
#[test]
fn test_assert_nrange_pass() {
    assert_nrange!((10..20), 20);
    assert_nrange!((1.0..2.0), 2.0);
}
#[test]
#[should_panic]
fn test_assert_nrange_fail() {
    assert_nrange!((10..=20), 20);
    assert_nrange!((1.0..=2.0), 2.0);
}
#[test]
#[should_panic]
fn test_assert_range_with_fail_msg() {
    // Assert with a message
    assert_range!(
        (1.0..=2.0),
        2.1,
        "Failed to assert that 2.1 is in the interval [1.0,2.0]"
    );
}
#[test]
#[should_panic]
fn test_assert_nrange_with_fail_msg() {
    // Assert with a message
    assert_nrange!((1.0..2.0), 1.5, "Oops! 1.5 is in the interval [1.0,2.0)")
}
#[test]
fn test_assert_near_pass() {
    assert_near!(0.0, 0.0, 0.1);
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON), f32::EPSILON);
    assert_near!(1.0f32, (1.0f32 - f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(expected = "assertion failed: `1.0000002 is not within epsilon 0.00000011920929 of 1")]
fn test_assert_near_fail_high() {
    assert_near!(1.0f32, (1.0f32 + f32::EPSILON + f32::EPSILON), f32::EPSILON);
}
#[test]
#[should_panic(expected = "assertion failed: `0.99999976 is not within epsilon 0.00000011920929 of 1")]
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
    let a = 3; let b = 28;
    assert_true!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn do_not_panic_when_true_and_do_not_emit_message() {
    let a = 3; let b = 27;
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
    let a = 3; let b = 27;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}
#[test]
fn do_not_panic_when_false_and_do_not_emit_message() {
    let a = 3; let b = 28;
    assert_false!(a + b == 30, "a = {}, b = {}", a, b);
}
