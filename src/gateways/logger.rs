use std::collections::hash_map;

pub fn logger_func(input: &str) {
    // add Log function
    let env: hash_map::HashMap<String, String> = std::env::vars().collect();
    let contains_debug = env.contains_key("DEBUG");
    if contains_debug {
        println!("{}",input);
    }
}


