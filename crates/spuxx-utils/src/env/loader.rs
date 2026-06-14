/// Loads a required environment variable.
///
/// Returns the value of the environment variable as a [`String`].
///
/// # Panics
///
/// Panics if the environment variable is not set.
/// ```
pub fn required(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Missing required environment variable: {key}"))
}

/// Loads an optional environment variable, falling back to a default value.
///
/// Returns the value of the environment variable if set, otherwise returns
/// `default` as a [`String`].
/// ```
pub fn optional(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[cfg(test)]
mod tests {
    use crate::env::loader::{optional, required};

    #[test]
    fn required_returns_value() {
        temp_env::with_var("FOO", Some("bar"), || {
            let foo = required("FOO");
            assert_eq!(foo, "bar");
        });
    }

    #[test]
    #[should_panic]
    fn required_panics_if_value_is_null() {
        required("FOO");
    }

    #[test]
    fn optional_returns_value() {
        temp_env::with_var("FOO", Some("bar"), || {
            let foo = optional("FOO", "baz");
            assert_eq!(foo, "bar");
        });
    }

    #[test]
    fn optional_falls_back_to_default() {
        let foo = optional("FOO", "baz");
        assert_eq!(foo, "baz");
    }
}
