use std::collections::HashMap;

use crate::gateways::logger::Logger;


pub fn parse_env(s: &str) -> Option<HashMap<String, String>> {
    let l = Logger::new("uc::env_parser::parse_env");
    l.log("will check if string is empty");
    if s.is_empty() {
        l.log("string indeed is empty");
        return None;
    }

    l.log("will create empty HashMap");
    let mut result: HashMap<String, String> = HashMap::new();

    // check if string is empty

    // split on spaces
    l.log("will split on spaces");
    l.log(format!("string to split: {:?}", s.split(" ")));
    for elem in s.split(" ") {
        l.log(format!("inspecting elem: {:?}", elem));
        // get the key and value from the env variable
        let mut key_val = elem.split("=");
        let key = key_val.next().unwrap().to_string();
        let val = key_val.next().unwrap().to_string();
        // add the key and value to the HashMap
        result.insert(key, val);
    }
    return Some(result);
}
