use std::collections::HashSet;
use regex::Regex;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct HttpAccessRule {
    method: String,
    domain_pattern_str: String,
    endpoint_pattern_str: String,
    domain_pattern: Regex,
    endpoint_pattern: Regex,
    method_pattern: Regex,
}

impl Eq for HttpAccessRule {}

impl PartialEq for HttpAccessRule {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method &&
        self.domain_pattern_str == other.domain_pattern_str &&
        self.endpoint_pattern_str == other.endpoint_pattern_str
    }
}

impl Hash  for HttpAccessRule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.method.hash(state);
        self.domain_pattern_str.hash(state);
        self.endpoint_pattern_str.hash(state);
    }
}

impl HttpAccessRule {
    pub fn new(method: &str, domain_pattern: &str, endpoint_pattern: &str) -> Option<Self> {
        let domain_regex = Regex::new(domain_pattern).unwrap();
        let endpoint_regex = Regex::new(endpoint_pattern).unwrap();
        let method_regex = Regex::new(method).unwrap();
        
        Some(HttpAccessRule {
            method: method.to_uppercase(),
            domain_pattern_str: domain_pattern.to_string(),
            endpoint_pattern_str: endpoint_pattern.to_string(),
            domain_pattern: domain_regex,
            endpoint_pattern: endpoint_regex,
            method_pattern: method_regex,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HTTPConfig {
    pub allowed_rules: HashSet<HttpAccessRule>,
}

impl Default for HTTPConfig {
    fn default() -> Self {
        let mut config = HTTPConfig::new();
        // config.allow_access("GET", ".*", ".*");
        // config.allow_access("POST", ".*", ".*");
        // config.allow_access("PUT", ".*", ".*");
        // config.allow_access("DELETE", ".*", ".*");
        config
    }
}

impl HTTPConfig {
    pub fn new() -> Self {
        HTTPConfig {
            allowed_rules: HashSet::new(),
        }
    }

    pub fn allow_access(&mut self, method: &str, domain_pattern: &str, endpoint_pattern: &str) {
        if let Some(rule) = HttpAccessRule::new(method, domain_pattern, endpoint_pattern) {
            self.allowed_rules.insert(rule);
        }
    }

    pub fn can_access(&self, method: &str,endpoint: &str) -> bool {
        // Separate endpoint and domain
        let mut endpoint_parts = endpoint.splitn(2, "/");
        let domain = endpoint_parts.next().unwrap_or("");
        let endpoint = endpoint_parts.next().unwrap_or("");
        // make method uppercase
        let method = method.to_uppercase();
        for rule in &self.allowed_rules {
            if rule.method_pattern.is_match(&method) &&
               rule.domain_pattern.is_match(domain) &&
               rule.endpoint_pattern.is_match(endpoint) {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests{
    // #[test]
    pub fn test(){
            let mut permissions = PermissionLayer::new();

        permissions.allow_access("GET", r"\w+\.example\.com", r"/api/.*");

        assert_eq!(permissions.can_access("GET", "sub.example.com/api/data"), true);
        assert_eq!(permissions.can_access("POST", "sub.example.com/api/data"), false);
        assert_eq!(permissions.can_access("GET", "notexample.com/api/data"), false);
        assert_eq!(permissions.can_access("GET", "sub.example.com/notapi/data"), false);
    }
}