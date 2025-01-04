# Automatic rebuilding with `build.rs`
This example shows how you can regenerate queries automatically when you build your crate, but only if your schema or queries are modified.

Add more queries and observe how they are added to the generated file when you rebuild the crate. Rebuilding the crate without modifying queries or schema should be instant as the build script does not need to be rerun.
