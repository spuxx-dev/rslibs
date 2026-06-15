/// Loads a required environment variable and parses it into type `T`.
///
/// # Panics
///
/// Panics if the environment variable is not set, or if its value cannot be parsed into `T`.
pub fn required<T>(key: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let type_name = std::any::type_name::<T>();
    let value = std::env::var(key)
        .unwrap_or_else(|_| panic!("Missing required environment variable: {key}"));
    value.parse().unwrap_or_else(|_| {
        panic!("Required environment variable {key} must be of type {type_name}")
    })
}

/// Loads an optional environment variable and parses it into type `T`, falling back to
/// `default` if the variable is not set.
///
/// # Panics
///
/// Panics if the variable is set but its value cannot be parsed into `T`.
pub fn optional<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let type_name = std::any::type_name::<T>();
    match std::env::var(key) {
        Ok(value) => value.parse().unwrap_or_else(|_| {
            panic!("Required environment variable {key} must be of type {type_name}")
        }),
        Err(_) => default,
    }
}

#[cfg(test)]
mod tests {
    use crate::env::loader::{optional, required};

    #[test]
    fn required_returns_value() {
        temp_env::with_var("FOO", Some("bar"), || {
            let foo = required::<String>("FOO");
            assert_eq!(foo, "bar");
        });
    }

    #[test]
    fn required_supports_non_strings() {
        temp_env::with_vars([("i32", Some("42")), ("bool", Some("true"))], || {
            let i32 = required::<i32>("i32");
            let bool = required::<bool>("bool");
            assert_eq!(i32, 42);
            assert!(bool);
        });
    }

    #[test]
    #[should_panic]
    fn required_panics_if_value_is_null() {
        temp_env::with_var("FOO", None::<&str>, || {
            required::<String>("FOO");
        });
    }

    #[test]
    #[should_panic]
    fn required_panics_if_type_mismatch() {
        temp_env::with_var("FOO", None::<&str>, || {
            required::<i32>("FOO");
        });
    }

    #[test]
    fn optional_returns_value() {
        temp_env::with_var("FOO", Some("bar"), || {
            let foo = optional::<String>("FOO", "baz".to_string());
            assert_eq!(foo, "bar");
        });
    }

    #[test]
    fn optional_supports_non_strings() {
        temp_env::with_vars([("i32", Some("6"))], || {
            let i32 = optional::<i32>("i32", 7);
            let bool = optional::<bool>("bool", false);
            assert_eq!(i32, 6);
            assert!(!bool);
        });
    }

    #[test]
    fn optional_falls_back_to_default() {
        temp_env::with_var("FOO", None::<&str>, || {
            let foo = optional("FOO", "baz".to_string());
            assert_eq!(foo, "baz");
        });
    }
}
