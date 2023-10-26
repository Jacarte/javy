# HTTP API

- We use reqwest crate since it can compile to Wasm-WASI
- The config file should have the whitelist of allowed domains, methods and endpoints

```yaml
rules:
    # The regex needs to be escaped
    - domain_pattern: "(www\\.)?google\\.com"
      endpoint_pattern: ".*"
      method: "GET|POST"    
```