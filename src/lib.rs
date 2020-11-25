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
        match env::var(KEY) {
            Ok("true") => if v == "true" {
                assert!($cond)
            }
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        match env::var(KEY) {
            Ok("true") = if v == "true" {
                let s = format!($($arg)+);
                assert!($cond, s)            }
        }

    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::env_assert!(true);
    }
}
