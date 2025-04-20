use std::collections::HashMap;

pub fn parse_env(s: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    // split on spaces
    for elem in s.split(" ") {
        // get the key and value from the env variable
        let mut key_val = elem.split("=");
        let key = key_val.next().unwrap().to_string();
        let val = key_val.next().unwrap().to_string();
        // add the key and value to the HashMap
        result.insert(key, val);
    }
    return result;
}
