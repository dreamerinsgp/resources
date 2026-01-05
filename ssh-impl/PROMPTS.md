# SSH Implementation - Common Prompts

## Quick Reference Prompts

### Run Server
```
cargo run -- server
```

### Run Client
```
cargo run -- client --host localhost --port 2222 --user testuser
```

### Test Connection
1. Terminal 1: `cargo run -- server`
2. Terminal 2: `cargo run -- client --host localhost --port 2222 --user testuser`
3. Password: `testpass`

### Build Project
```
cargo build
```

### Check for Errors
```
cargo check
```

### Run Tests (if you add them)
```
cargo test
```

## Common Development Prompts

### Add a new feature
"Add [feature] to [module]. Follow the existing code style and add educational logging."

### Debug an issue
"Debug [issue] in [file]. Check [specific area] and provide detailed error messages."

### Refactor code
"Refactor [module] to [improvement]. Maintain backward compatibility and add comments."

### Explain code
"Explain how [function/module] works. Include details about [specific aspect]."

## Project-Specific Information

- **Default User**: testuser
- **Default Password**: testpass  
- **Default Port**: 2222
- **Key Storage**: ~/.ssh_edu/
- **Host Key**: ~/.ssh_edu/host_key
- **Known Hosts**: ~/.ssh_edu/known_hosts
- **User Database**: ~/.ssh_edu/users.json

