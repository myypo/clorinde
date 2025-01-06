# Adding custom types through `clorinde.toml`
This example shows how you can add custom types to be used. You need to create a new crate which implements the `FromSql` and `ToSql` traits from `postgres-types` for your custom types.

The custom type crates are imported into the Clorinde (codegen) crate for their use in the root crate.

By default, if no crates are specified but custom types are used, Clorinde will look for a crate named `ctypes` at the relative path `../ctypes`. You can override this default or add additional crates as needed:

```toml
[types.crates]
# Local crate with a relative path
ctypes = { path = "../ctypes" }

# Crate from crates.io with a specific version
custom_types = "1.0.0"

# Crate with additional configuration
types_with_features = { version = "2.0", features = ["date", "time"] }

# Complete example with all options
full_example = { 
    version = "1.2.3",
    path = "../local_types",
    features = ["custom_types"],
    default-features = false,
    optional = true
}
```

You can specify multiple crates, and each one can use any of the standard Cargo dependency specifications. This includes:
- Simple version strings for crates from crates.io
- Local crates with path dependencies
- Crates with specific features enabled
- Crates with default features disabled
- Optional dependencies
- Any combination of these options

The configuration follows the same format as dependencies in `Cargo.toml`, making it familiar and flexible to use.
