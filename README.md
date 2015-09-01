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
use validate::Schema;
let schema = Schema::new().email().length(1..100);
assert!(schema.validate(&"test@domain.com").is_ok());
assert!(schema.validate(&"notvalidemail").is_err());