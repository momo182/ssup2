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
        let mut r: usize;
        if let Some(index) = host.find(PASS_SEPARATOR) {
            r = index.try_into().unwrap();
            r = r + PASS_SEPARATOR.len();
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
        let mut r: usize;
        if let Some(index) = host.find(TUBE_NAME_SEPARATOR) {
            r = index.try_into().unwrap();
            r = r + TUBE_NAME_SEPARATOR.len();
        } else {
            return None;
        }
        return Some(r);
    } else {
        return None;
    }
}

pub fn find_tube_name_end(host: &str) -> Option<usize> {
    if contains_one(host, &[TUBE_NAME_SEPARATOR.to_string()]) {
        let r: usize;
            r = host.len();
        return Some(r);
    } else {
        return None;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_password_start() {
        let host = "user@hostname | password > tube_name";
        let result = find_password_start(host);
        assert_eq!(result, Some(16));

        let host = "user_is_longer@hostname | long_password > tube_name";
        let result = find_password_start(host);
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_find_password_end() {
        let host = "user@hostname | password > tube_name";
        let result = find_password_end(host);
        assert_eq!(result, Some(24));

        let host2 = "user@hostname22 | password > tube_name";
        let result2 = find_password_end(host2);
        assert_eq!(result2, Some(26));
    }

    #[test]
    fn test_find_tube_name_start() {
        let host = "user@hostname | password > tube_name";
        let result = find_tube_name_start(host);
        assert_eq!(result, Some(27));

        let host = "user@hostname22 | password > tube_name";
        let result = find_tube_name_start(host);
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_find_tube_name_end() {
        let host = "user@hostname | password > tube_name";
        let result = find_tube_name_end(host);
        assert_eq!(result, Some(36));
    }

}