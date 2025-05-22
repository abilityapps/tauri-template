# Tauri Template

This template should help get you started developing completely local, cross-platform desktop apps.

## Starting the app

1. `bun i`
2. `bunx tauri dev`

## Database Setup

By default, `tauri-template` comes with an embedded SQLite database named `tauri-template.db`.
You can override this with the following steps:

1. Choose between `SQLite` or `PostgreSQL`
2. Set `DATABASE_URL` in `src-tauri/.env`:

   `sqlite:<name>.db` for SQLite

   `postgres://postgres@localhost/<name>` for Postgres (or cloud-hosted URL)

1. `sqlx migrate add <name>` creates a migration file in `migrations/<timestamp>-<name>.sql`. Run this command in `src-tauri/` Add your database schema changes here. 
2. Next time you restart your app, your migrations will be applied. `sqlx migrate run` applies your migrations manually. 

Go [here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#reverting-migrations) for information on migration reverts.

Rust does not support cetain database types. You must create serializers for those data types.
Read here for [Sqlite](https://docs.rs/sqlx/latest/sqlx/sqlite/types/) and here for [Postgres](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html).

You can look [here](https://github.com/shouryan01/weekability/blob/267f160b8ff751fe193beda9bc11a1d23b6a3c44/src-tauri/src/db/schema.rs) for an example on how to serialize Postgres `Numeric` and `Date` types in Rust using the `time` and `rust_decimal` crates.

## Building the app

`bun run tauri build`

## Recommended IDE Setup

[Rust Rover](https://www.jetbrains.com/rust/) - this is the default recommendation. It is free for non-commercial use.

[VS Code](https://code.visualstudio.com/) - install all the recommendations from `.vscode/extensions.json`. This should be an automatic popup.

## Stack

### Backend
- [Tauri](http://tauri.app/)
- [SQLite](https://www.sqlite.org) | [Postgres](https://www.postgresql.org) with [sqlx](https://github.com/launchbadge/sqlx)

Easiest way to use Postgres on Mac is [Postgres.app](https://postgresapp.com)

### Frontend
- [React](http://react.dev/)
- [Bun](https://bun.sh) + [Vite](https://vite.dev)
- [Tanstack Router](https://tanstack.com/router/latest)
- [Tailwindcss](https://tailwindcss.com)
- [shadcn/ui](https://ui.shadcn.com)
- [Biomejs](https://biomejs.dev)

This template was manually created. For future project creations, you can use the new (and excellent!) `create-tsrouter-app@latest` with your favorite package manager!

