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
    ($left:expr, $right:expr $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_gt!($($arg)*); })
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
    ($left:expr, $right:expr $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_ge!($($arg)*); })
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
    ($left:expr, $right:expr $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_lt!($($arg)*); })
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
    ($left:expr, $right:expr $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_le!($($arg)*); })
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_range!($($arg)*); })
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
    ($left:expr, $right:expr $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_nrange!($($arg)*); })
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
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => ({
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
    ($left:expr, $right:expr, $epsilon:expr, $($arg:tt)+ $(,)?) => ({
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
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_near!($($arg)*); })
}

/// Asserts that the expression is true
///
/// # Examples
/// ```
/// use all_asserts::assert_true;
/// let a = false;
/// #[cfg(should_panic)]
/// assert_true!(a);
/// let a = true;
/// assert_true!(a);
/// ```
///
#[macro_export]
macro_rules! assert_true {
    ($cond:expr $(,)?) => ({
        assert!($cond)
    });
    ($cond:expr, $($arg:tt)+) => ({
        assert!($cond, $($arg)+)
    });
}

/// This is a debug-only variant of the [`assert_true`] macro
///
/// [`assert_true`]: ./macro.assert_true.html
#[macro_export]
macro_rules! debug_assert_true {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_true!($($arg)*); })
}

/// Asserts that the expression is false
///
/// # Examples
/// ```
/// use all_asserts::assert_false;
/// let a = true;
/// #[cfg(should_panic)]
/// assert_false!(a);
/// let a = false;
/// assert_false!(a);
/// ```
///
#[macro_export]
macro_rules! assert_false {
    ($cond:expr $(,)?) => ({
        assert!(!$cond)
    });
    ($cond:expr, $($arg:tt)+) => ({
        assert!(!$cond, $($arg)+)
    });
}

/// This is a debug-only variant of the [`assert_false`] macro
///
/// [`assert_false`]: ./macro.assert_false.html
#[macro_export]
macro_rules! debug_assert_false {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_false!($($arg)*); })
}
