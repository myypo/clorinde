# Clorinde Benchmarks
Benchmarking suite for code generation and code execution, heavily based on [the diesel benchmarking suite](https://github.com/diesel-rs/diesel/tree/master/diesel_bench).

Note that the benchmarks use the `diesel` crate which links directly against `libpq`, so you will need to install it.
On debian-based distros, the package is `libpq-dev`, on RHEl-based distros, it is named `libpq-devel`, and for Arch-based distros, it is included with `postgresql-libs`.

## Running
```bash
cargo bench
```

## Results
These results are from benchmarks run locally on `2025-02-15` and may not reflect production environment performance. See the full Criterion report [here](https://halcyonnouveau.github.io/clorinde/benchmarks/2025-02-15/report).

### System
- CPU: AMD Ryzen 7 9800X3D
- RAM: 64GB DDR5-6400
- OS: EndeavourOS

### Versions
- Rust: 1.86.0
- PostgreSQL: 17.2
- Libraries:
  - postgres: 0.19.9
  - tokio-postgres: 0.7.12
  - diesel: 2.2.6
  - sqlx: 0.8.3
  - clorinde: 0.12.0

### Trivial Query
Measures performance of `SELECT * FROM users`

![Trivial Query Benchmark](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/benches/results/2025-02-15/trivial_query.svg)

### Medium Complex Query
Measures performance of a LEFT JOIN between users and posts

![Medium Complex Query Benchmark](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/benches/results/2025-02-15/medium_complex_query.svg)

### Batch Insert
Measures performance of inserting multiple rows (1, 100, 1000 rows) in a single transaction

![Batch Insert Benchmark](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/benches/results/2025-02-15/insert.svg)

### Loading Associations
Measures performance of loading users with their associated posts and comments

![Loading Associations Benchmark](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/benches/results/2025-02-15/loading_associations_sequentially.svg)

## Disclaimer
These benchmarks are meant to provide a rough comparison of different approaches in a local development environment. Real-world performance can vary significantly based on many factors including network latency, concurrent access patterns, and specific use cases.
