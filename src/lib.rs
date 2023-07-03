use std::{fmt::{Display, Debug}, env, ffi::OsString, borrow::Borrow, fs::read_to_string};
use regex::Regex;
use serde_json::{Value, Map};

pub struct Loader {
    json: Value
}

pub struct HiddenLoader<'a> {
    loader: &'a Loader
}

impl <'a> HiddenLoader<'a> {
    pub fn i64(&self, name: &str) -> Hidden<i64> {
        Hidden(self.loader.i64(name))
    }
    pub fn u64(&self, name: &str) -> Hidden<u64> {
        Hidden(self.loader.u64(name))
    }
    pub fn f64(&self, name: &str) -> Hidden<f64> {
        Hidden(self.loader.f64(name))
    }
    pub fn str(&self, name: &str) -> Hidden<String> {
        Hidden(self.loader.str(name))
    }
    pub fn bool(&self, name: &str) -> Hidden<bool> {
        Hidden(self.loader.bool(name))
    }
    pub fn json(&self, name: &str) -> Hidden<Value> {
        Hidden(self.loader.json(name))
    }
    pub fn custom<T, F>(&self, name: &str, f: F) -> Hidden<T> where F: Fn(String) -> T {
        Hidden(self.loader.custom(name, f))
    }
}

pub struct Hidden<T>(T);

impl <T> Hidden<T> {
    pub fn val(self) -> T {
        self.0
    }
}

impl <T>Display for Hidden<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<redacted>")
    }
}

impl <T> Debug for Hidden<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("<redacted>").finish()
    }
}

impl Loader {
    pub fn i64(&self, name: &str) -> i64 {
        match env::var_os(name) {
            Some(v) => v.into_string().unwrap().parse::<i64>().unwrap(),
            None => self.json[name].as_i64().unwrap(),
        }
    }
    pub fn u64(&self, name: &str) -> u64 {
        match env::var_os(name) {
            Some(v) => v.into_string().unwrap().parse::<u64>().unwrap(),
            None => self.json[name].as_u64().unwrap(),
        }
    }
    pub fn f64(&self, name: &str) -> f64 {
        match env::var_os(name) {
            Some(v) => v.into_string().unwrap().parse::<f64>().unwrap(),
            None => self.json[name].as_f64().unwrap(),
        }
    }
    pub fn str(&self, name: &str) -> String {
        match env::var_os(name) {
            Some(v) => v.into_string().unwrap(),
            None => self.json[name].as_str().unwrap().to_owned(),
        }
    }
    pub fn bool(&self, name: &str) -> bool {
        match env::var_os(name) {
            Some(v) => v.into_string().unwrap().parse::<bool>().unwrap(),
            None => self.json[name].as_bool().unwrap(),
        }
    }
    pub fn json(&self, name: &str) -> Value {
        let s = match env::var_os(name) {
            Some(v) => v.into_string().unwrap(),
            None => self.json[name].as_str().unwrap().to_owned(),
        };
        serde_json::from_str(s.as_str()).unwrap()
    }
    pub fn custom<T, F>(&self, name: &str, f: F) -> T where F: Fn(String) -> T {
        f(self.str(name))
    }
    pub fn hidden(&self) -> HiddenLoader {
        HiddenLoader { loader: self }
    }
}

impl From<Value> for Loader {
    fn from(value: Value) -> Self {
        Loader {
            json: value,
        }
    }
}

fn remove_comments(jsonc_content: &str) -> String {
  let regex = Regex::new(r"//[^\n]*").unwrap();
  regex.replace_all(jsonc_content, "").to_string()
}

pub fn configuru<T>(default_path: &str) -> T where T: From<Loader> {
    let path = match env::var_os("CFG_JSON_PATH") {
        Some(v) => v.into_string().unwrap(),
        None => default_path.to_owned(),
    };
    let root_dir = env::current_dir().expect("Failed to get current directory");
    let json_file_path = root_dir.join(path);
    let contents = if let Some(extension) = json_file_path.extension() {
      let file_contents = read_to_string(&json_file_path)
          .expect(format!("File {} does not exist", json_file_path.to_str().unwrap()).as_str());

      match extension.to_str() {
          Some("json") => file_contents,
          Some("jsonc") => remove_comments(&file_contents),
          _ => panic!("Unsupported file extension"),
      }
      } else {
      panic!("File extension not found");
    };
    let x: serde_json::Value = serde_json::from_str(contents.as_str()).unwrap();
    return T::from(Loader::from(x))
}