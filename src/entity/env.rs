use indexmap::IndexMap; // Use IndexMap to preserve insertion order from YAML
use serde::Deserialize;
use std::fmt;
use crate::gateways::logger::Logger;
use crate::usecase::inventory_tools::{resolve_shell,is_shell};

/// Represents an environment variable key-value pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // Added common derives
pub struct EnvVar {
    // Fields are typically public in structs used like this
    pub key: String,
    pub value: String,
}

// Implement Display trait for easy string conversion (like Go's String())
impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl EnvVar {
    /// Creates a new EnvVar.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        EnvVar {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the environment variable as a bash export statement.
    /// Note: This performs simple string embedding. For robust shell compatibility,
    /// consider more sophisticated escaping if values can contain special characters.
    pub fn as_export(&self) -> String {
        // Use raw string literal r#""# for clarity if needed, but format! handles quotes well here.
        format!(r#"export {}="{}";"#, self.key, self.value)
    }
}

/// Represents a list of environment variables, deserialized from a YAML map
/// while preserving the original order.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(transparent)] // Deserialize directly into the 'store' field
pub struct EnvList {
    // Use IndexMap<String, String> to preserve insertion order.
    store: IndexMap<String, String>,
}

impl EnvList {
    /// Creates an empty EnvList.
    pub fn new() -> Self {
        EnvList::default()
    }
    
    /// Gets the value associated with a key, if it exists.
    /// Returns an Option<&str> which is idiomatic Rust for potentially missing values.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(|s| s.as_str()) // .map converts Option<&String> to Option<&str>
    }
    
    /// Returns an iterator over the keys in their original insertion order.
    pub fn keys(&self) -> indexmap::map::Keys<'_, String, String> {
        self.store.keys()
    }
    
    /// Returns a Vec<String> containing clones of the keys in order.
    pub fn keys_owned(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }
    
    /// Returns an iterator over the values in their original insertion order.
    pub fn values(&self) -> indexmap::map::Values<'_, String, String> {
        self.store.values()
    }
    
    /// Returns an iterator over the (key, value) pairs in their original insertion order.
    pub fn iter(&self) -> indexmap::map::Iter<'_, String, String> {
        self.store.iter()
    }
    
    /// Returns the list of environment variables as a Vec<String>
    /// in the format "KEY=VALUE", preserving the original order.
    pub fn to_string_vec(&self) -> Vec<String> {
        self.store
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect()
    }
    
    /// Sets or updates a key-value pair in the list.
    /// Takes ownership of the key and value strings.
    pub fn set(&mut self, key: String, value: String) {
        let l = Logger::new("entity::env::set");
        // The check for nil map in Go is not needed because `store` is initialized.
        l.log(format!("Setting {} = {}", key, value).as_str()); // Use the log crate
        self.store.insert(key, value); // IndexMap::insert replaces the value if key exists
    }
    
    /// Returns all environment variables as a single string of bash export statements.
    /// Statements are generated in the original insertion order.
    pub fn as_export_string(&self) -> String {
        if self.store.is_empty() {
            return String::new();
        }
        
        let mut exports = String::new();
        for (key, value) in self.store.iter() {
            // Avoid creating an intermediate EnvVar struct just for formatting
            exports.push_str(&format!(r#"export {}="{}"; "#, key, value));
        }
        // Remove the trailing space and semicolon
        exports.pop(); // remove trailing space
        // exports.pop(); // remove trailing semicolon - actually the go code didn't remove this
        
        exports
    }
    
    /// Returns the number of environment variables.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    
    /// Returns true if the list contains no environment variables.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Resolves all environment variables that contain shell syntax in their values
    /// and updates them in-place.
    pub fn resolve_all(&mut self) {
        let l = Logger::new("entity::env::resolve_all");
        l.log("resolving values in env list");
        let store = &mut self.store;
        for (key, value) in store.clone().iter() {
            l.log(format!("Resolving ${} = {}", key, value).as_str());
            if is_shell(value) {
                l.log(format!("Found shell syntax in ${}", key).as_str());
                let resolved = match resolve_shell(value) {
                    Ok(resolved) => {
                        l.log(format!("Resolved ${} = {}", key, resolved).as_str());
                        resolved
                    },
                    Err(e) => {
                        l.log(format!("Error resolving ${}: {}", key, e).as_str());
                        std::process::exit(1);
                    }
                };
                store.swap_remove(key);
                store.insert(key.clone(), resolved);
            }
        } 
    }
}


// Example Usage (typically in main.rs or tests)
/*
fn main() {
// Initialize logger (e.g., env_logger::init();)

let yaml_data = r#"
FIRST: "hello"
SECOND: "world"
REF: "${FIRST}_${SECOND}" # NOTE: Variable substitution is NOT handled by this code
"#;

// Deserialize
let env_list: Result<EnvList, serde_yaml::Error> = serde_yaml::from_str(yaml_data);

match env_list {
Ok(list) => {
println!("Deserialized List: {:?}", list);

println!("Get SECOND: {:?}", list.get("SECOND")); // Some("world")
println!("Get MISSING: {:?}", list.get("MISSING")); // None

println!("Keys: {:?}", list.keys().collect::<Vec<_>>()); // ["FIRST", "SECOND", "REF"]
println!("String Vec: {:?}", list.to_string_vec()); // ["FIRST=hello", "SECOND=world", "REF=${FIRST}_${SECOND}"]

println!("Export String: {}", list.as_export_string());
// export FIRST="hello"; export SECOND="world"; export REF="${FIRST}_${SECOND}";

}
Err(e) => {
eprintln!("Failed to parse YAML: {}", e);
}
}
}
*/

// Unit tests would go here or in a separate tests module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_env_var_display() {
        let ev = EnvVar::new("KEY", "VALUE");
        assert_eq!(ev.to_string(), "KEY=VALUE");
    }
    
    #[test]
    fn test_env_var_as_export() {
        let ev = EnvVar::new("MY_VAR", "some data");
        assert_eq!(ev.as_export(), r#"export MY_VAR="some data";"#);
    }
    
    #[test]
    fn test_deserialize_and_order() {
        // Initialize logger only once
        // let _ = env_logger::builder().is_test(true).try_init();
        
        let yaml_data = r#"
        B_VAR: "2"
        A_VAR: "1"
        C_VAR: "3"
        "#;
        let list: EnvList = serde_yaml::from_str(yaml_data).expect("Should deserialize");
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.get("A_VAR"), Some("1"));
        
        // Check order using keys_owned()
        let keys = list.keys_owned();
        assert_eq!(keys, vec!["B_VAR".to_string(), "A_VAR".to_string(), "C_VAR".to_string()]);
        
        // Check order using to_string_vec()
        let string_vec = list.to_string_vec();
        assert_eq!(string_vec, vec![
            "B_VAR=2".to_string(),
            "A_VAR=1".to_string(),
            "C_VAR=3".to_string(),
            ]);
            
            // Check order using as_export_string()
            let export_str = list.as_export_string();
            assert_eq!(export_str, r#"export B_VAR="2"; export A_VAR="1"; export C_VAR="3";"#); // Note: Go version had extra trailing space
        }
        
        #[test]
        fn test_set_and_get() {
            // let _ = env_logger::builder().is_test(true).try_init();
            let mut list = EnvList::new();
            assert!(list.is_empty());
            
            list.set("FIRST".to_string(), "one".to_string());
            list.set("SECOND".to_string(), "two".to_string());
            
            assert_eq!(list.len(), 2);
            assert_eq!(list.get("FIRST"), Some("one"));
            assert_eq!(list.get("SECOND"), Some("two"));
            assert_eq!(list.get("THIRD"), None);
            
            // Overwrite
            list.set("FIRST".to_string(), "uno".to_string());
            assert_eq!(list.get("FIRST"), Some("uno"));
            assert_eq!(list.len(), 2); // Length should remain 2
            
            // Check order after sets
            let keys = list.keys_owned();
            assert_eq!(keys, vec!["FIRST".to_string(), "SECOND".to_string()]);
        }
}
    