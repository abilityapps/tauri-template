# Tauri Template

This template should help get you started developing completely local, cross-platform desktop apps.

## Starting the app

1. `bun i`
2. `bunx tauri dev`

## Database Setup

1. `sqlx migrate add <name>` creates a migration file in `migrations/<timestamp>-<name>.sql`. Run this command in `src-tauri/` Add your database schema changes here. 
2. Next time you restart your app, your migrations will be applied. `sqlx migrate run` applies your migrations manually. 

Go [here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#reverting-migrations) for information on migration reverts.

Rust does not support cetain database types. You must create serializers for those data types.
Read [here](https://docs.rs/sqlx/latest/sqlx/sqlite/types/) for more.

You can look in [this repo](https://github.com/shouryan01/weekability/blob/main/src-tauri/src/db/schema.rs) for an example on how to serialize Postgres `Numeric` and `Date` types in Rust using the `time` and `rust_decimal` crates.

## Recommended IDE Setup

[Rust Rover](https://www.jetbrains.com/rust/) - this is the default recommendation. It is free for non-commercial use.

[VS Code](https://code.visualstudio.com/) - install all the recommendations from `.vscode/extensions.json`. This should be an automatic popup.

## Stack

### Backend
- [Tauri](http://tauri.app/)
- [SQLite](https://www.sqlite.org) with [sqlx](https://github.com/launchbadge/sqlx)

### Frontend
- [React](http://react.dev/)
- [Bun](https://bun.sh) + [Vite](https://vite.dev)
- [Tanstack Router](https://tanstack.com/router/latest)
- [Tailwindcss](https://tailwindcss.com)
- [shadcn/ui](https://ui.shadcn.com)
- [Biomejs](https://biomejs.dev)
