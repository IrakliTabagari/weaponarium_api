# Run migration

```cmd
sqlx migrate run
sqlx migrate info
```

# Data Seeding

this runs binary from /src/bin/seed folder which contains seeding logic.
it can be run in CI/CD pipeline to avoid running it from actual code on every instance creation.

```cmd
cargo run --bin seed
```
