# AmimeWatch

A [Telegram](https://telegram.org/) bot to watch animes.

Re-written version of [AmimeWatch](https://github.com/AmanoTeam/AmimeWatch/) in Rust.

## Preparing

### Configuration

Create a file named `config.toml` and fill with your data:

```toml
[telegram]
api_id = 1
api_hash = ""

[bot]
token = ""
prefixes = ["/"]
```

### Database

Create the database [with](src/database/database.sql) and put your MySQL URL in the environment like:

```bash
export DATABASE_URL="mysql://root:toor@localhost:3306/amime"
```

### Logging

```bash
export RUST_LOG=info cargo run
```

## Running

Just do:

```bash
cargo run --release
```
