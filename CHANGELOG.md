# Changelog

## Version 2.3.0 [2021-09-13]

> This release doesn't introduce breaking changes

## Additions

- Added the `assert_true`, `assert_false`, `assert_near` macros

All changes in this project will be noted in this file.

## Version 2.2.0 [2020-11-10]

> This release doesn't introduce breaking changes

Added `debug_*` variants for macros

## Version 2.1.0 [2020-06-25]

> This release doesn't introduce breaking changes

Enable adding messages to the `assert_range!` and `assert_nrange!` outputs.

Example:

```rust
assert_nrange!((1.0..2.0), 1.5, "Oops! 1.5 is in the interval [1.0,2.0)")
```

## Version 2.0.1 [2020-06-25]

> This release doesn't introduce breaking changes

Fix documentation issues

## Version 2.0.0 [2020-06-25]

> This release doesn't introduce breaking changes

Added new macros: `assert_range!` , `assert_nrange!`

## Version 1.0.1 [2020-06-18]

> This release doesn't introduce breaking changes

Fix issues with ownership and licensing

## Version 1.0.0 [2019-11-16]

> This release doesn't introduce breaking changes

First stable release

## Version 0.x.y

> These releases don't introduce any breaking changes
