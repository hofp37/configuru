use std::env;

/// .todo add docs
struct Loader {
    defaultPath: String
}

/// .todo add docs
enum ValueType {
    STRING = 1,
    FLOAT = 2,
    INTEGER = 3,
    BOOLEAN = 4,
    JSON = 5,
}

/// todo add docs
struct StringValue {
    var_name: String,
}

trait ValueBuilder<T> {
    /// todo add docs
    fn hidden() -> T;
    /// .todo add docs
    fn get_value() -> T;
}

impl ValueBuilder<String> for StringValue {
    /// todo add docs
    fn hidden() -> String {
        // todo implement
        return "<hidden>".to_owned()
    }
    /// .todo add docs
    fn get_value() -> String {
        // todo implement based on type
        return "".to_owned()
    }
}

/// todo add docs
impl Loader {
    /// .todo add docs
    fn load<T>(var_name: String, var_type: ValueType) -> ValueBuilder<T> {
        match var_type {
            ValueType::STRING => StringValue { var_name },
            _ => panic!("")
        }
    }
    /// .todo add docs
    fn load_custom<F, R>(var_name: String, f: F) -> R where F: Fn(String) -> R {
        // todo implement
        return f("test".to_owned())
    }
}

/// todo add docs
fn create_loader() -> Loader {
    // todo implement factory

    return Loader {
        defaultPath: ".env.jsonc".to_owned()
    }
}

/// todo add docs
fn create_config_storage(filePath: String) {
    // todo implement file loading
}

/// todo add docs
fn get_file_path(loader: Loader) -> String {
    let load_result = env::var("CFG_JSON_PATH");
    if load_result.is_err() {
        loader.defaultPath
    } else {
    load_result.unwrap()
    }
}
