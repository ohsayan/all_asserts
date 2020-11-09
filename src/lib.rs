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

#[macro_export]
macro_rules! debug_assert_nrange {
    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { assert_nrange!($($arg)*); })
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
