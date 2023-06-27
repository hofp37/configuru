#[cfg(test)]
mod tests {
    use serde_derive::Deserialize;
    use serde_json;
    use configuru::{Loader, Hidden, configuru};

    #[derive(Debug, Deserialize)]
    struct Customer {
        email: String,
        age: i64,
    }

    #[derive(Debug)]
    struct Config {
        port: i64,
        host: String,
        password: Hidden<String>,
        customer: Customer,
    }

    impl From<Loader> for Config {
        fn from(loader: Loader) -> Self {
            Config {
                port: loader.i64("SERVER_PORT"),
                host: loader.str("SERVER_HOST"),
                password: loader.hidden().str("PASSWORD"),
                customer: loader.custom("CUSTOMER", |str| serde_json::from_str(&str).unwrap()),
            }
        }
    }

    #[test]
    fn test() {
        let config: Config = configuru(".env.json");
        println!("{:?}", config);
        assert_eq!("<redacted>", config.password.to_string());
        assert_eq!("testtest", config.password.val());
    }
}