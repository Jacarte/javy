use std::collections::HashSet;
use glob::Pattern;

#[derive(Debug, Clone)]
pub struct FSConfig {
    read_whitelist: HashSet<Pattern>,
    read_blacklist: HashSet<Pattern>,
    write_whitelist: HashSet<Pattern>,
    write_blacklist: HashSet<Pattern>,
}

impl Default for FSConfig {
    fn default() -> Self {
        Self {
            read_whitelist: HashSet::new(),
            read_blacklist: HashSet::new(),
            write_whitelist: HashSet::new(),
            write_blacklist: HashSet::new(),
        }
    }
}

impl FSConfig {

    pub fn add_to_read_whitelist(&mut self, pattern: &str) {
        if let Ok(p) = Pattern::new(pattern) {
            self.read_whitelist.insert(p);
        }
    }

    pub fn add_to_read_blacklist(&mut self, pattern: &str) {
        if let Ok(p) = Pattern::new(pattern) {
            self.read_blacklist.insert(p);
        }
    }

    pub fn add_to_write_whitelist(&mut self, pattern: &str) {
        if let Ok(p) = Pattern::new(pattern) {
            self.write_whitelist.insert(p);
        }
    }

    pub fn add_to_write_blacklist(&mut self, pattern: &str) {
        if let Ok(p) = Pattern::new(pattern) {
            self.write_blacklist.insert(p);
        }
    }

    pub fn can_read(&self, path: &str) -> bool {
        let whitelisted = self.read_whitelist.iter().any(|pattern| pattern.matches(path));
        let blacklisted = self.read_blacklist.iter().any(|pattern| pattern.matches(path));
        whitelisted && !blacklisted
    }

    pub fn can_write(&self, path: &str) -> bool {
        let whitelisted = self.write_whitelist.iter().any(|pattern| pattern.matches(path));
        let blacklisted = self.write_blacklist.iter().any(|pattern| pattern.matches(path));
        whitelisted  && !blacklisted
    }
}
