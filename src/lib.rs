/// Asserts that the left hand side expression is greater
/// than the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] extern crate all_asserts;
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
                if !(*left_val > *right_val) || (*left_val == *left_val) {
                    panic!(r#"assertion failed: `(left <= right)`
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
                    panic!(r#"assertion failed: `(left <= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                    }
                }
            }
        });
    }
/// Asserts that the left hand side expression is greater
/// than or equal to the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] extern crate all_asserts;
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
                    panic!(r#"assertion failed: `(left < right)`
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
                    panic!(r#"assertion failed: `(left < right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}
/// Asserts that the left hand side expression is lesser
/// than the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] extern crate all_asserts;
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
                    panic!(r#"assertion failed: `(left >= right)`
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
                    panic!(r#"assertion failed: `(left >= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}
/// Asserts that the left hand side expression is lesser than
/// or equal to the right hand side expression
///
/// # Examples
/// ```
/// # #[macro_use] extern crate all_asserts;
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
                    panic!(r#"assertion failed: `(left > right)`
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
                if !(*left_val <= *right_val) {
                    panic!(r#"assertion failed: `(left > right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
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
