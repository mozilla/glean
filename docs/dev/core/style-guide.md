# Style Guide

## Code

All Rust code is formatted using [`rustfmt`](https://github.com/rust-lang/rustfmt).
Run `make fmt-rust` to format code in your local checkout.
This is enforced in CI.

### Strings in code

Multi-line strings in Rust code should use double-quotes where possible, or raw string markers (`r#" "#`) if needed.

When indentation doesn't matter, the double-quote should be on its own line and the start of the string indented by 4 spaces below.
The closing double-quote should be aligned with the identifier.
Unfortunately `rustfmt` will not enforce the intended formatting.

**Good**:

```rust
let query = "
    SELECT *
    FROM table
    WHERE id IS NOT NULL
";
```


**Bad**:

```rust
let query = "SELECT
* FROM table
  WHERE id IS NOT NULL
  ";
```

When the multi-line string is within a macro (e.g. `format!`), the opening and closing double-quote go on their own line, indented by 4 spaces below the identifier.
`rustfmt` will enforce the quote on its own line, but not the indentation.

**Good**:

```rust
let query = format!(
    "
    SELECT *
    FROM {table}
    WHERE id IS NOT NULL
    "
);
```


**Bad**:

```rust
let query = format!("SELECT
* FROM {table}
  WHERE id IS NOT NULL
  ");
```

## Documentation

See [Documentation Guidelines](documentation-guidelines.md) for details.
