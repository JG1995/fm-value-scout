# FM ValueScout — CSV Parser & Scouting View

Desktop companion for Football Manager moneyball-style scouting. Built with Tauri 2 + Rust + SvelteKit 5.

## Project Structure

- `src-tauri/src/` — Rust backend (Tauri commands, SQLite DB, CSV parser pipeline)
- `src/` — Svelte 5 frontend
- `src-tauri/capabilities/` — Tauri capability permissions

## Tauri Commands

| Command            | Parameters                                  | Returns           |
| ------------------ | ------------------------------------------- | ----------------- |
| `set_managed_club` | `club_name: String`                         | `()`              |
| `get_managed_club` | —                                           | `Option<String>`  |
| `set_preference`   | `key: String`, `value: String`              | `()`              |
| `get_preference`   | `key: String`                               | `Option<String>`  |
| `get_seasons`      | —                                           | `Vec<SeasonInfo>` |
| `import_csv`       | `file_path: String`, `in_game_date: String` | `ImportResult`    |

### SeasonInfo

```json
{
	"id": 1,
	"name": "2029 Season",
	"in_game_date": "15.6.2029",
	"import_date": "2026-01-01 12:00:00",
	"filename": "players.csv",
	"player_count": 1420
}
```

### ImportResult

```json
{
	"season_id": 1,
	"season_name": "2029 Season",
	"players_imported": 1420,
	"rows_rejected": [{ "row_number": 42, "reasons": ["Missing UID"] }]
}
```

### CSV Import Pipeline

`import_csv` runs the full pipeline:

1. Read file → encoding detection (UTF-8 + BOM stripping, Windows-1252 fallback)
2. Parse CSV (semicolon delimiter) via `csv` crate
3. Validate rows via parser pipeline
4. Expand compound FM positions into canonical codes
5. Batch insert into SQLite in a single atomic transaction (RAII)
6. Return `ImportResult` with season ID and per-row rejection details

## Build & Run

```bash
bun install
cd src-tauri && cargo build
cd .. && bun tauri dev
```

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
