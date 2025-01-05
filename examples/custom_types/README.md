# Adding custom types through `clorinde.toml`
This example shows how you can add custom types to be used. You need to create a new crate (in this case `ctypes`) which includes the types you want to add and which implements the `FromSql` and `ToSql` traits from `postgres-types`.

The `ctypes` crate is imported into the Clorinde (codegen) crate for its use in the root crate.

You can change the name and path of the `ctypes` crate, but then you must tell Clorinde what/where it is with these lines of config.

```toml
[types.crate]
# name of the custom types crate
name = "ctypes"
# path of the custom types crate (relative to the Clorinde (codegen) crate)
path = "../ctypes"
```
