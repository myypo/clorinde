# Using Clorinde
You can use Clorinde as a CLI or a library API depending on your needs. The CLI is simpler and allows you to use the same binary for all your projects, but the library can be used to integrate or automate Clorinde as part of your own project.

## Workflows
Clorinde is flexible with how it can be used. Here are some useful workflows when developing with Clorinde:

### Basic
This is the simplest workflow. Create a `queries/` directory containing your PostgreSQL queries at the root of your crate. Then, run the CLI to generate a `clorinde` crate in your directory. Note that the CLI will require the path to one or more PostgreSQL schemas to build against. 

You're done! Now you can add the `clorinde` crate to your `Cargo.toml`.

```toml
[dependencies]
clorinde = { path = "clorinde" }
```

And import your generated query items from it. When you modify your queries or schemas, you can re-run the CLI to update the generated code.

### Automatic query rebuild
The setup is the same as the basic workflow, but instead of using the CLI to generate your queries, create a `build.rs` build script that invokes Clorinde's API. Build scripts have built-in functionalities that allow them to be re-executed every time your queries or schema(s) change. See this [example](https://github.com/halcyonnouveau/clorinde/tree/main/examples/auto_build) for details.

### Self-managed database
With this workflow, you don't need a container manager at all. Set up your database in the state you want, then use Clorinde's `live` functionality to build directly against this database using a connection URL. Both the CLI and API support this.
