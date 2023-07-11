<div align="center">

<h1>Configuru</h1>

</div>

Configuru is a library for configuration management. It is JSON/JSONC configuration loader and a clone project of  [AckeeCZ/configuru](https://github.com/AckeeCZ/configuru) tool built for Node.js apps.

## Getting started

1. Install

```bash
cargo add configuru
```

2. Create `.env.jsonc` in root of your project, add defaults or placeholders.

```jsonc
{
  // Database secrets
  "PASSWORD": "testtest",
  // Sever secrets
  "SERVER_HOST": "localhost",
  "SERVER_PORT": 3000,
  // Entities
  "CUSTOMER": "{\"age\": 25, \"email\":\"test@example.org\"}" // customer related secrets
}
```

3. _(optional)_ As a developer (or environment), create a custom override file (e.g. `~/.env/my-project.jsonc`) and save the path in your `CFG_JSON_PATH`.

4. Usage

```rust
// Import dependencies
use serde_derive::Deserialize;
use configuru::{Loader, Hidden, configuru};
use serde_json;

// Define data structures you want to load from configuration file. Data types must match variables in configuration file.
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

// Implement a conversion from Loader to your Config
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
```

5. Use your configuration params throughout your app

```rust
fn main() {
  // Usage how to load config  
    let config: Config = configuru(".env.jsonc");
    println!("Example: {:?}", config) // Example: Config { port: 3000, host: "localhost", password: <redacted>, customer: Customer { email: "test@example.org", age: 25 } }
}
```

## License

This project is licensed under [MIT](./LICENSE).