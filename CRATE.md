# Importing and using

Add these lines to your rust application:
```rust
#[macro_use]
extern crate all_asserts;
```
And now you can use the asserts wherever you like!
On Panic you should get something like this (the example is from `assert_lt!`):
```
thread 'main' panicked at 'assertion failed: `(left <= right)`
  left: `100`,
 right: `200`', src/main.rs:79:5
```
Isn't that much better than what the current macros provide? Well, I'll leave you to decide!

# Usage of the macros
The name of the assert pretty much tells you everything:
- `assert_gt!(a, b)` -> Will panic if a is not greater than b
- `assert_ge!(a, b)` -> Will panic if a is not greater than or equal to b
- `assert_lt!(a, b)` -> Will panic if a is not less than b
- `assert_le!(a, b)` -> Will panic if a is not greater than or equal to b
