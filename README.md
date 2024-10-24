# Tauri Template

This template should help get you started developing completely local, cross-platform desktop apps.

## Getting Started

## Starting the app

1. `bun i`
2. `bunx tauri dev`

## Database Setup

1. Choose between `SQLite` or `PostgreSQL`
2. Set `DATABASE_URL` in `src-tauri/.env`:

    `sqlite:<name>.db` for SQLite

    `postgres://postgres@localhost/<name>` for Postgres

3. `sqlx database create` creates your new database. Run in `src-tauri/`

    (`sqlx database drop` will delete the database specified in `DATABASE_URL`)
4. `sqlx migrate add <name>` creates a migration file in `migrations/<timestamp>-<name>.sql` . Add your database schema changes here.
5. `sqlx migrate run` applies your migrations

Go [here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#reverting-migrations) for information on migration reverts.

Rust does not support cetain database types. You must create serializers for those data types.
Read here for [Postgres](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html) and here for  [Sqlite](https://docs.rs/sqlx/latest/sqlx/sqlite/types/).

You can look in [this repo](https://github.com/shouryan01/weekability/blob/main/src-tauri/src/db/schema.rs) for an example on how to serialize Postgres `Numeric` and `Date` types in Rust using the `time` and `rust_decimal` crates.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) 

Install all the recommendations from `.vscode/extensions.json`. This should be an automatic popup.

## Stack

### Backend
- [Tauri](http://tauri.app/)
- [Postgres](https://www.postgresql.org) | [SQLite](https://www.sqlite.org) with [sqlx](https://github.com/launchbadge/sqlx)

    Easiest way to use Postgres on Mac is [Postgres.app](https://postgresapp.com)

### Frontend
- [React](http://react.dev/)
- [Bun](https://bun.sh) + [Vite](https://vite.dev)
- [Tanstack Router](https://tanstack.com/router/latest)
- [Tailwindcss](https://tailwindcss.com)
- [shadcn/ui](https://ui.shadcn.com)
- [Biomejs](https://biomejs.dev)
