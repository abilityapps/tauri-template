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

3. `sqlx database create` creates your new database. Run in `src-tauri/`.

    (`sqlx database drop` will delete the database specified in `DATABASE_URL`)
4. `sqlx migrate add <name>` creates a migration file in `migrations/<timestamp>-<name>.sql` . Add your database schema changes here.
5. `sqlx migrate run` applies your migrations

Go [here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#reverting-migrations) for information on migration reverts.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) 

Install all the recommendations from `.vscode/extensions.json`. This should be an automatic popup.