# all_asserts [![Build Status](https://travis-ci.org/sntdevco/all_asserts.svg?branch=master)](https://travis-ci.org/sntdevco/all_asserts) [![all_asserts crate](https://img.shields.io/crates/v/all_asserts.svg?style=flat-square)](https://crates.io/crates/all_asserts) [![Crates.io](https://img.shields.io/crates/d/all_asserts.svg?color=%234527A0)](https://crates.io/crates/all_asserts) [![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE)
### A crate for multiple types of asserts that don't exist in the standard library
# Rationale
There are several kinds of problems that I had faced while writing tests, especially when writing comparator tests.
The output from the existing macros, were providing _vague_ outputs while debugging which made me write this crate.
I had created an RFC, but neverthless it was rejected as there were not enough use cases. I am not too sure about others, but I find myself writing multiple tests that make use of these asserts.

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

# Further enhancements
- [ ] Provide better output for arrays instead of printing the length
- [ ] Provide better output for strings instead of printing the length

# Building

Simply run:
```shell
$ git clone https://github.com/sntdevco/all_asserts.git
$ cd all_asserts
$ cargo build
```
And for testing:
```shell
$ cargo test
```

# Contributing
Your welcome to! 
> "No man is an island!"

And I always stick to that belief! Please help me out in better formatting the output as mentioned in the [Further Enhancements](#further-enhancements) section. If you find an issue, go ahead a create one! (All doubts, questions, ideas are welcome)
