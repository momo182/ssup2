use crate::entity::const_values::{PASS_SEPARATOR, TUBE_NAME_SEPARATOR};

fn contains_one(s: &str, subs: &[String]) -> bool {
    for sub in subs {
        if s.contains(sub) {
            return true;
        }
    }
    false
}

pub fn find_password_start(host: &str) -> Option<usize> {
    // PASS_SEPARATOR
    if contains_one(host, &[PASS_SEPARATOR.to_string()]) {
        // find index of the first PASS_SEPARATOR 
        let r: usize;
        if let Some(index) = host.find(PASS_SEPARATOR) {
            r = index.try_into().unwrap();
        } else {
            return None;
        }
        return Some(r);
    } else {
        return None;
    }
}

pub fn find_password_end(host: &str) -> Option<usize> {
    // TUBE_NAME_SEPARATOR.to_string()
    if contains_one(host, &[PASS_SEPARATOR.to_string()]) {
        // find index of the first PASS_SEPARATOR 
        let r: usize;
        if let Some(index) = host.find(TUBE_NAME_SEPARATOR) {
            r = index;
        } else {
            return None;
        }
        return Some(r);
    } else {
        return None;
    }
}

pub fn find_tube_name_start(host: &str) -> Option<usize> {
    // TUBE_NAME_SEPARATOR
    if contains_one(host, &[TUBE_NAME_SEPARATOR.to_string()]) {
        // find index of the first PASS_SEPARATOR 
        let r: usize;
        if let Some(index) = host.find(TUBE_NAME_SEPARATOR) {
            r = index.try_into().unwrap();
        } else {
            return None;
        }
        return Some(r);
    } else {
        return None;
    }
}

pub fn find_tube_name_end(host: &str) -> Option<usize> {
    let result = host.len();
    Some(result)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_password_start() {
        let host = "user:password@hostname";
        let result = find_password_start(host);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_find_password_end() {
        let host = "user:password@hostname#tube";
        let result = find_password_end(host);
        assert_eq!(result, Some(20));

        let host2 = "user:password@hostname";
        let result2 = find_password_end(host2);
        assert_eq!(result2, Some(20));
    }

    #[test]
    fn test_find_tube_name_start() {
        let host = "user:password@hostname#tube";
        let result = find_tube_name_start(host);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_find_tube_name_end() {
        let host = "user:password@hostname#tube";
        let result = find_tube_name_end(host);
        assert_eq!(result, Some(25));
    }

}