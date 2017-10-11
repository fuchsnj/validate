# Validate

A library to easily validate user input

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
validate = "*"
```

and this to your crate root:

```rust
extern crate validate;
```

## Example

```rust
use validate::*;

assert!(email().validate(&"test@domain.com").is_ok());
assert!(bound(..3).validate(&2).is_ok())
