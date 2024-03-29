# env_assert

- [crates.io link](https://crates.io/crates/env_assert)
- [docs.rs link](https://docs.rs/env_assert/)

`env_assert` is a incredibly simple Rust library that allows you to only run an `assert!` when an the `RUST_ENV_ASSERT` environmental variable is set to `true`.

For example:

```rust
use env_assert::env_assert;
fn main() {
    let res = expensive_func_that_should_return_positive();
    env_assert!(res.is_positive()); // "sanity check"

    println!("We got here because the environmental variable was not set!");
}

fn expensive_func_that_should_return_positive() -> i8 {
    // do some really hard things here

    // oh no! our expensive function messed up and is going to return a negative value
    -42
}
```

```text
$ cargo run
We got here because the environmental variable was not set!
```

Now lets set our variable and then run

```text
$ RUST_ENV_ASSERT=true cargo run
thread 'main' panicked at 'assertion failed: res.is_positive()', src/main.rs:4:5
```

## What problem does this solve?

Sometimes, the performance increase for running in release mode is significant, but I still want asserts.
However, some of those asserts are in a sense debug asserts, and I would rather the program continue than crash when it is deployed.
This library lets you have asserts while in release mode, without negatively impacting performance for end users.

## Should I use this?

Eh, probably not.
This crate is good for simple testing and pet projects, but if this behavior is desired you should probably now use a [Cargo profile](https://doc.rust-lang.org/cargo/reference/profiles.html) to enable `debug_assert!()` and optimizations at the same time.
