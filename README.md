![cool hat](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/docs/assets/clorinde_hat.png)

# Clorinde

Clorinde generates type-checked Rust interfaces from PostgreSQL queries, with an emphasis on compile-time safety and high performance. It is a fork of [Cornucopia](https://github.com/cornucopia-rs/cornucopia) that enhances the original with an improved architecture and expanded capabilities.

## Key Features

Building on Cornucopia's foundation:
* SQL-first approach with powerful query validation
* Sync and async driver support with optional connection pooling
* Non-allocating row mapping
* Available as both a library and CLI tool
* Native `rust-postgres` performance
* Complete support for custom PostgreSQL types (composites, domains, and enums)
* One-dimensional array handling for all supported types
* Granular type nullity control

Clorinde adds:
* Crate-based code generation architecture 
* Full wasm compatibility
* Custom Rust type mapping

## Installation

Install with:

```bash
cargo install clorinde
```

## Quick Example
Write your PostgreSQL queries with annotations and named parameters:

```sql
-- queries/authors.sql

--! authors
SELECT first_name, last_name, country FROM Authors;

--! insert_author
INSERT INTO Authors(first_name, last_name, country)
VALUES (:first_name, :last_name, :country)
```

Generate the crate with `clorinde`, then you can import it into your project after adding it to your `Cargo.toml`:
```toml
clorinde = { path = "./clorinde" }
```

```rust
use clorinde::queries{authors, insert_author};

insert_author.bind(&client, "Agatha", "Christie", "England");

let all_authors = authors().bind(&client).all();

for author in all_authors {
  println!("[{}] {}, {}", 
    author.country, 
    author.last_name.to_uppercase(), 
    author.first_name
  )
}
```

For more examples go to the [/examples](https://github.com/halcyonnouveau/clorinde/tree/main/examples) directory, or head over to the [book](https://halcyonnouveau.github.io/clorinde/) to learn more.

## MSRV

This crate uses Rust 2021 edition, which requires at least version 1.62.1.

## Similar Libraries

- [Cornucopia](https://github.com/cornucopia-rs/cornucopia) - The original project Clorinde is forked from
- [sqlc](https://github.com/sqlc-dev/sqlc) (Go) - Generate type-safe code from SQL
- [Kanel](https://github.com/kristiandupont/kanel) (TypeScript) - Generate TypeScript types from Postgres
- [jOOQ](https://github.com/jOOQ/jOOQ) (Java) - Generate typesafe SQL from your database schema

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
