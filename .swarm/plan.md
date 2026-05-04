<!-- PLAN_HASH: 2bb9790se0tgm -->

# FM ValueScout — CSV Parser & Scouting View

Swarm: default
Phase: 1 [COMPLETE] | Updated: 2026-05-04T19:25:16.169Z

---

## Phase 1: Database Foundation (Rust) [COMPLETE]

- [x] 1.1: Add rusqlite (bundled), csv, encoding_rs, regex, once_cell, phf (macros feature) to Cargo.toml; add tauri-plugin-dialog to Cargo.toml; add @tauri-apps/plugin-dialog to package.json npm dependencies [MEDIUM]
- [x] 1.2: Create src-tauri/src/db.rs with full schema: players (uid, name, nation, height, left_foot, right_foot), player_positions (uid, canonical_position), seasons (id, name, in_game_date, import_date, filename, player_count), season_players (id, season_id, uid, club, position_raw, age, minutes, expires + 82 metric columns), season_player_positions (season_id, uid, canonical_position), player_percentiles (season_id, uid, metric_name, pool_type, canonical_position, percent_rank), archetypes (id, name, base_position, metric_weights_json), archetype_scores (season_id, uid, archetype_id, quality_score, value_score), settings (key TEXT PK, value TEXT). Include seed_archetypes() function stub [LARGE] (depends: 1.1)
- [x] 1.3: Implement database init: open SQLite connection with WAL mode (PRAGMA journal_mode=WAL, cache_size=-32000, mmap_size=268435456, foreign_keys=ON), run create_tables, store connection as Tauri managed state via AppState struct, wire into lib.rs run() [MEDIUM] (depends: 1.2)

---

## Phase 2: CSV Parsing Engine (Rust) [PENDING]

- [ ] 2.1: Create src-tauri/src/parsers/mod.rs with FieldValue enum, PlayerRecord struct (82+ fields), Schema struct (parsers registry + column name to index HashMap) [MEDIUM]
- [ ] 2.2: Implement currency/magnitude parser: strip any currency symbol (€£$), apply K/M/B magnitude multipliers, extract upper bound from ranges (€160M-€210M to 210M), handle decimal before magnitude (€19.25K) [MEDIUM] (depends: 2.1)
- [ ] 2.3: Implement wage denomination parser: extract numeric value with magnitude multiplier and denomination suffix (p/w, p/m, p/a) as separate fields [MEDIUM] (depends: 2.1)
- [ ] 2.4: Implement unit-stripping parser for fields with cm/km suffixes; apply field-specific precision (height=int, distance=1dp, xG=2dp) [SMALL] (depends: 2.1)
- [ ] 2.5: Implement nationality code mapper using phf compile-time hash map: 3-letter codes to full names (200+ entries), preserve already-full names as-is [SMALL] (depends: 2.1)
- [ ] 2.6: Implement position string splitter: split comma-separated position string into Vec<String> [SMALL] (depends: 2.1)
- [ ] 2.7: Implement footedness parser: map Very Strong to 20, Fairly Strong to 15, Reasonable to 10, Weak to 5; store raw string; unrecognized to score=NULL [SMALL] (depends: 2.1)
- [ ] 2.8: Implement appearances parser: extract starts and substitute counts from '43 (3)' format; bare number to subs=0 [SMALL] (depends: 2.1)
- [ ] 2.9: Implement computed metrics: per-90 from totals (total/minutes*90), totals from per-90 (per90*minutes/90), ratios (completed/attempted); division by zero to 0; store negative floats for xG OP/xGP [MEDIUM] (depends: 2.1)
- [ ] 2.10: Implement CSV row validator: reject missing UID, missing player name, duplicate UID within same import, invalid numeric field values; return specific rejection reasons [MEDIUM] (depends: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8)
- [ ] 2.11: Implement full row parser orchestrating all field parsers into PlayerRecord; detect optional CA/PA columns from header; handle empty cells to NULL; includes Contract Expiry (Expires) field parsed as date string [LARGE] (depends: 2.10, 2.9)
- [ ] 2.12: Implement position expansion: map compound position strings to canonical codes. Rules: D(LC) to [DC,DL], WB/M(L) to [WB_L,M_L], D(RC)/WB(R) to [DC,DR,WB_R]. Build lookup table mapping each compound FM position to its canonical codes. Used by import_csv (3.2) for season_player_positions population [MEDIUM] (depends: 2.6)

---

## Phase 3: Tauri Commands & Integration (Rust) [PENDING]

- [ ] 3.1: Implement encoding detection utility: try UTF-8 decode first, fall back to Windows-1252/Latin-1; strip BOM prefix before CSV reading [SMALL] (depends: 1.1)
- [ ] 3.2: Implement import_csv Tauri command: read file via dialog, detect encoding, stream CSV with csv crate (delimiter b';'), parse each row through pipeline including position expansion (2.12), batch insert 1000 rows per transaction into seasons/players/season_players/season_player_positions, emit progress events, atomic full-transaction with rollback on error, return ImportSummary [LARGE] (depends: 1.3, 2.11, 2.12, 3.1)
- [ ] 3.3: Implement get_seasons Tauri command returning all past imports (id, name, in_game_date, import_date, filename, player_count) ordered by import_date desc [SMALL] (depends: 1.3)
- [ ] 3.4: Register all Phase 3 Tauri commands in lib.rs invoke_handler: import_csv, get_seasons, set_managed_club, get_managed_club, set_preference, get_preference. Remove the default greet command and its function. AppState struct is already created by 1.3 [SMALL] (depends: 3.2, 3.3, 3.6, 3.7)
- [ ] 3.5: Add Tauri capabilities: dialog:default and fs:read with allowed paths to src-tauri/capabilities/default.json for file picker and CSV reading [SMALL]
- [ ] 3.6: Implement set_managed_club and get_managed_club Tauri commands: store/retrieve user's managed club name in settings table (key='managed_club'). Used for own-club exclusion in scouting [SMALL] (depends: 1.3)
- [ ] 3.7: Implement set_preference(key, value) and get_preference(key) Tauri commands: generic key-value storage in settings table. Used by frontend for score mode, comparison pool, and other user preferences. Reuses the same settings table as managed club [SMALL] (depends: 1.3)

---

## Phase 4: Frontend Import UI (Svelte 5) [PENDING]

- [ ] 4.1: Create src/routes/import/+page.svelte: glassmorphism-styled drop zone with file picker via Tauri dialog plugin, in-game date input (three dropdowns day/month/year), import button disabled until file+date ready, progress bar showing processed/rejected counts, post-import summary card with success/error states, and managed club settings input from task 4.4. Copy per spec [LARGE] (depends: 3.4, 3.5)
- [ ] 4.2: Create src/routes/+layout.svelte with glassmorphism navigation shell: nav bar with Import and Scouting links, app header, wrapper slot for page content [SMALL]
- [ ] 4.3: Update src/routes/+page.svelte as landing page: replace default Tauri greet template with welcome message, app description, and prominent CTA button linking to /import [SMALL] (depends: 4.2)
- [ ] 4.4: Create managed club settings component: text input with label 'Your Managed Club' and save button invoking set_managed_club; loads current value via get_managed_club on mount; placed on import page or accessible via nav. Uses same glassmorphism styling [SMALL] (depends: 3.6, 4.2)

---

## Phase 5: Scouting Backend — Percentiles & Scoring (Rust) [PENDING]

- [ ] 5.0: Implement seed_archetypes() in db.rs: populate archetypes table with ~60 initial archetype definitions (name, base_position, metric_weights as JSON array). Each archetype with in/out-of-possession variants gets two rows. Run via INSERT OR IGNORE from DB init for idempotency. Modify db.rs init to call seed_archetypes() after create_tables() [LARGE] (depends: 1.3)
- [ ] 5.1: Implement percentile computation: for each metric, compute PERCENT_RANK() partitioned by canonical_position (position pool) and unpartitioned (all pool), insert into player_percentiles table; called at import completion in same transaction [LARGE] (depends: 1.3)
- [ ] 5.2: Implement quality score computation: for a given archetype, load each metric's percentile, compute weighted sum of (weight * percentile), normalize to 0-100; support inverted metrics (use 1.0-percent_rank); for dual-variant archetypes compute combined score=(0.75*in_possession)+(0.25\*out_of_possession); exclude players with all-NULL archetype metrics [MEDIUM] (depends: 5.1)
- [ ] 5.3: Implement value score computation: compute transfer value percentile across all players, apply linear multiplier 1.5 to 0.5 as value percentile goes 0 to 1; no-value players treated as cheapest percentile (x1.5); transfer value of exactly 0 also treated as cheapest [MEDIUM] (depends: 5.1)
- [ ] 5.4: Implement archetype score pre-computation: at import completion, iterate all archetypes from seeded archetypes table (5.0) times all players, compute quality (incl. 75/25 dual-variant) and value scores via 5.2/5.3, bulk insert into archetype_scores table [LARGE] (depends: 5.0, 5.2, 5.3)
- [ ] 5.5: Implement get_archetypes Tauri command: query archetypes table, parse metric_weights JSON, return all archetype definitions with name, base_position, metrics[] (name+weight+inverted+in_possession flag) [SMALL] (depends: 5.0)
- [ ] 5.6: Implement get_scouting_results Tauri command: query archetype_scores filtered by position (via season_player_positions), archetype_id, pool_type, most recent season; apply minutes>=1000 filter and own-club exclusion (using managed_club from settings table); return sorted by active score mode with tiebreaker; include top-weighted archetype metrics [LARGE] (depends: 3.6, 5.4)
- [ ] 5.7: Register Phase 5 scouting commands in lib.rs invoke_handler: add get_archetypes and get_scouting_results alongside existing Phase 3 commands. Verify all commands are registered [SMALL] (depends: 3.4, 5.5, 5.6)

---

## Phase 6: Scouting View UI — Core & Database View (Svelte 5) [PENDING]

- [ ] 6.1: Create src/routes/scouting/+page.svelte: glassmorphism-styled scouting page with mode selector segmented control (Database View / Role Search tabs), toolbar area for controls, content area for table/podium; default to Database View [MEDIUM] (depends: 4.2)
- [ ] 6.2: Implement Database View table: load all players via get_scouting_results (no archetype filter), display sortable columns with arrow indicator, color-coded percentile cells on red-yellow-green gradient, numeric sorting, row hover highlight [LARGE] (depends: 5.6, 6.1)
- [ ] 6.3: Implement comparison pool toggle (Against Position / Against All) in Database View toolbar; toggling triggers immediate percentile recalculation via backend and recolors cells with ~300ms color transition; persists via set_preference [MEDIUM] (depends: 3.7, 6.2)
- [ ] 6.4: Implement player row click navigation: clicking any player row navigates to /player/[uid] placeholder page showing basic player identity (name, club, age, position) [SMALL] (depends: 6.2)

---

## Phase 7: Scouting View UI — Role Search (Svelte 5) [PENDING]

- [ ] 7.1: Implement position dropdown to archetype dropdown dependency chain: select position loads relevant archetypes from get_archetypes populates archetype dropdown; select archetype calls get_scouting_results displays results; toolbar layout per spec [MEDIUM] (depends: 5.5, 6.1)
- [ ] 7.2: Implement Top 3 Podium: three ranked player cards (1st center elevated with gold accent border) showing name (large), age, club, quality score, value score, transfer value; ranking driven by active score mode with tiebreaker; clickable to /player/[uid]; fewer than 3 candidates shows placeholder cards [MEDIUM] (depends: 6.2, 7.1)
- [ ] 7.3: Implement results table below podium: fixed columns (Name, Age, Club, Contract Expiry, Quality Score, Value Score) + top-weighted archetype metric columns (up to 12); sortable by any column header with arrow indicator; percentile cell coloring; row click to /player/[uid] [LARGE] (depends: 7.2)
- [ ] 7.4: Implement score mode toggle (Quality/Value) with 300ms debounce, podium reorder animation (~400ms crossfade+translate), results table re-sort by new score column; persists via set_preference from task 3.7; restores from preference on mount [MEDIUM] (depends: 3.7, 7.3)
- [ ] 7.5: Implement scouting state preservation via Svelte 5 $state rune: mode, position, archetype, score mode, pool type, sort column/direction survive navigation to Player Profile and back without re-querying backend [SMALL] (depends: 7.3)
- [ ] 7.6: Implement user preference persistence: restore score mode (via get_preference) and comparison pool on Scouting tab visit; position/archetype NOT persisted (reset each visit). Backend persistence provided by set_preference/get_preference from task 3.7 [SMALL] (depends: 3.7, 6.3, 7.4)
- [ ] 7.7: Implement all empty states: no database imported (prompt with Import link), no archetype selected (prompt), zero search results (suggest adjusting filters), fewer than 3 podium candidates (placeholder cards), all rows rejected (detailed list). Copywriting per spec [SMALL] (depends: 6.1, 7.1)

---

## Phase 8: Build Verification & Polish [PENDING]

- [ ] 8.1: Run cargo build to verify all Rust code compiles without errors; fix any compilation issues [SMALL] (depends: 3.4, 5.7)
- [ ] 8.2: Run bun run check (svelte-check) for frontend TypeScript type-checking; fix any type errors [SMALL] (depends: 4.4, 7.7)
- [ ] 8.3: Run pre-check batch (lint:check + secretscan) on all changed source files; fix any findings [SMALL] (depends: 8.1, 8.2)
