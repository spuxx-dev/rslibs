/// A trait for types that represent a validated, fully-loaded environment configuration.
///
/// Implement this trait on a struct whose fields are populated from environment variables.
/// The [`load`](EnvConfig::load) method is the single entry point for reading all relevant
/// variables — typically via [`loader::required`](crate::env::loader::required) and
/// [`loader::optional`](crate::env::loader::optional) — and constructing the config value.
///
/// The `Send + Sync + 'static` bounds make it safe to store the loaded config in a
/// global or shared context (e.g. `OnceLock`, `Arc`, Axum state, etc.).
///
/// # Panics
///
/// Implementations are expected to panic on startup if a required variable is missing or
/// malformed, making misconfiguration a hard, immediate failure rather than a silent runtime
/// error.
///
/// # Example
///
/// ```no_run
/// use spuxx_utils::env::config::EnvConfig;
/// use spuxx_utils::env::loader::{optional, required};
/// use std::sync::LazyLock;
///
/// pub struct Config {
///     pub database_url: String,
///     pub port: u16,
/// }
///
/// impl EnvConfig for Config {
///     fn load() -> Self {
///         Self {
///             database_url: required::<String>("DATABASE_URL"),
///             port: optional::<u16>("PORT", 8080),
///         }
///     }
/// }
///
/// pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);
/// println!("Connecting to {}", CONFIG.database_url);
/// ```
pub trait EnvConfig: Sized + Send + Sync + 'static {
    /// Loads and constructs the configuration from environment variables.
    ///
    /// This method is the single entry point for reading all relevant environment
    /// variables and building the config value. It is typically called once at
    /// application startup and the result stored in a global or shared context.
    fn load() -> Self;
}

#[cfg(test)]
mod tests {
    use crate::env::config::EnvConfig;
    use crate::env::loader::{optional, required};
    use std::sync::LazyLock;

    #[test]
    fn can_access_env_variables() {
        temp_env::with_vars([("FOO", Some("bar")), ("NUMBER", Some("42"))], || {
            struct Config {
                foo: String,
                optional: String,
                number: i32,
            }
            impl EnvConfig for Config {
                fn load() -> Self {
                    Self {
                        foo: required::<String>("FOO"),
                        optional: optional::<String>("OPTIONAL", "default".to_string()),
                        number: required::<i32>("NUMBER"),
                    }
                }
            }
            pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);
            assert_eq!(CONFIG.foo, "bar");
            assert_eq!(CONFIG.optional, "default");
            assert_eq!(CONFIG.number, 42);
        });
    }
}
