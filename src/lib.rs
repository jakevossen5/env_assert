//! `env_assert` is a incredibly simple Rust library that allows you to only run an `assert!` when an the `RUST_ENV_ASSERT` environmental variable is set to `true`.
//!
//! For example:
//!
//! ```no_run
//! use env_assert::env_assert;
//! fn main() {
//!     let res = expensive_func_that_should_return_positive();
//!     env_assert!(res.is_positive()); // "sanity check"
//!
//!     println!("We got here because the environmental variable was not set!");
//! }
//!
//! fn expensive_func_that_should_return_positive() -> i8 {
//!     // do some really hard things here
//!
//!     // oh no! our expensive function messed up and is going to return a negative value
//!     -42
//! }
//! ```
//!
//! ```text
//! $ cargo run
//! We got here because the environmental variable was not set!
//! ```
//!
//! Now lets set our variable and then run
//!
//! ```text
//! $ RUST_ENV_ASSERT=true cargo run
//! thread 'main' panicked at 'assertion failed: res.is_positive()', src/main.rs:4:5
//! ```
//!
//! ## What problem does this solve?
//!
//! Sometimes, the performance increase for running in release mode is significant, but I still want asserts.
//! However, some of those asserts are in a sense debug asserts, and I would rather the program continue than crash when it is deployed.
//! This library lets you have asserts while in release mode, without negatively impacting performance for end users.
//!
//! ## Should I use this?
//!
//! Eh, probably not.
//! This crate is good for simple testing and pet projects, but if this behavior is desired you should probably now use a [Cargo profile](https://doc.rust-lang.org/cargo/reference/profiles.html) to enable `debug_assert!()` and optimizations at the same time.
#[macro_export]
macro_rules! env_assert {
    ($cond:expr) => {{
        const KEY: &'static str = "RUST_ENV_ASSERT";
        match std::env::var(KEY) {
            Ok(v) => if v == "true" {
                assert!($cond)
            }
            _ => ()
        }
    }};
    ($cond:expr,) => {{
        const KEY: &'static str = "RUST_ENV_ASSERT";
        match std::env::var(KEY) {
            Ok(v) => if v == "true" {
                assert!($cond)
            }
            _ => ()
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        const KEY: &'static str = "RUST_ENV_ASSERT";
        match std::env::var(KEY) {
            Ok(v) => if v == "true" {
                let s = format!($($arg)+);
                assert!($cond, s)
            }
            _ => ()
        }

    }};
}

// Note, tests should be run with the environmental variable set, aka RUST_ENV_ASSERT=true cargo test
#[cfg(test)]
mod tests {
    const KEY: &'static str = "RUST_ENV_ASSERT";

    fn set_var_to_true() {
        std::env::set_var(KEY, "true");
    }

    fn remove_var() {
        std::env::remove_var(KEY);
    }

    #[test]
    fn just_true() {
        set_var_to_true();
        super::env_assert!(true);
    }

    #[test]
    fn true_with_comma() {
        set_var_to_true();
        super::env_assert!(true,);
    }

    #[test]
    fn true_with_fmt() {
        set_var_to_true();
        super::env_assert!(true, "didn't crash with {}", 5);
    }

    #[test]
    #[should_panic(expected = "false assert is panic")]
    fn test_panic_var_true() {
        set_var_to_true();
        super::env_assert!(false, "false assert is panic");
    }

    #[test]
    fn assert_when_var_is_not_set() {
        remove_var();
        assert!(std::env::var(KEY).is_err());
        super::env_assert!(true, "asserting with true");
        super::env_assert!(false, "asserting with false");
    }
}
