# AmimeWatch

A [Telegram](https://telegram.org/) bot to watch animes.

Re-written version of [AmimeWatch](https://github.com/AmanoTeam/AmimeWatch/) in Rust.

## Preparing

Create a file named `config.toml` and fill with your data:

```toml
[telegram]
api_id = 1
api_hash = ""

[bot]
token = ""
prefixes = ["/"]
```

After it, create a database [with](src/database/database.sql) and put your MySQL URL in the environment like:

```bash
export DATABASE_URL="mysql://root:toor@localhost:3306/amime"
```

## Running

Just do:

```bash
cargo run --release
```
