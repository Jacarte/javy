# HTTP API

- We use reqwest crate since it can compile to Wasm-WASI
- The config file should have the whitelist of allowed domains, methods and endpoints

```yaml
rules:
      # in regex
    - domain_pattern: google\.com
      endpoint_pattern: ".*"
      method: "GET|POST"    
```