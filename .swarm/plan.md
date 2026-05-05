<!-- PLAN_HASH: 1t2pj7uwfxoa7 -->
# FM ValueScout — CSV Parser & Scouting View
Swarm: default
Phase: 1 [COMPLETE] | Updated: 2026-05-05T10:47:52.661Z

---
## Phase 1: Database Foundation (Rust) [COMPLETE]
- [x] 1.1: Add rusqlite (bundled), csv, encoding_rs, regex, once_cell, phf (macros feature) to Cargo.toml; add tauri-plugin-dialog to Cargo.toml; add @tauri-apps/plugin-dialog to package.json npm dependencies [MEDIUM]
- [x] 1.2: Create src-tauri/src/db.rs with full schema [LARGE]
- [x] 1.3: Implement database init: open SQLite connection with WAL mode [MEDIUM]

---
## Phase 2: CSV Parsing Engine (Rust) [COMPLETE]
- [x] 2.1: Create src-tauri/src/parsers/mod.rs with FieldValue enum, PlayerRecord struct, Schema struct [MEDIUM]
- [x] 2.2: Implement currency/magnitude parser [MEDIUM]
- [x] 2.3: Implement wage denomination parser [MEDIUM]
- [x] 2.4: Implement unit-stripping parser for cm/km suffixes [SMALL]
- [x] 2.5: Implement nationality code mapper using phf hash map [SMALL]
- [x] 2.6: Implement position string splitter [SMALL]
- [x] 2.7: Implement footedness parser [SMALL]
- [x] 2.8: Implement appearances parser from '43 (3)' format [SMALL]
- [x] 2.9: Implement computed metrics: per-90, totals, ratios [MEDIUM]
- [x] 2.10: Implement CSV row validator [MEDIUM]
- [x] 2.11: Implement full row parser orchestrator [LARGE]
- [x] 2.12: Implement position expansion for compound FM positions [MEDIUM]

---
## Phase 3: Tauri Commands & Integration (Rust) [PENDING]
- [x] 3.1: Implement encoding detection utility: try UTF-8 decode first, fall back to Windows-1252/Latin-1; strip BOM prefix before CSV reading [SMALL]
- [ ] 3.2: Implement import_csv Tauri command: read file via dialog, detect encoding, stream CSV, parse rows through pipeline, batch insert 1000 rows per transaction [LARGE] (depends: 1.3, 2.11, 2.12, 3.1)
- [ ] 3.3: Implement get_seasons Tauri command returning all past imports [SMALL] (depends: 1.3)
- [ ] 3.4: Register all Phase 3 Tauri commands in lib.rs invoke_handler [SMALL] (depends: 3.2, 3.3, 3.6, 3.7)
- [ ] 3.5: Add Tauri capabilities: dialog:default and fs:read with allowed paths [SMALL]
- [x] 3.6: Implement set_managed_club and get_managed_club Tauri commands [SMALL] (depends: 1.3)
- [ ] 3.7: Implement set_preference and get_preference Tauri commands [SMALL] (depends: 1.3)

---
## Phase 4: Frontend Import UI (Svelte 5) [PENDING]
- [ ] 4.1: Create src/routes/import/+page.svelte with glassmorphism-styled drop zone, file picker, in-game date input, import button, progress bar, summary card [LARGE] (depends: 3.4, 3.5)
- [ ] 4.2: Create src/routes/+layout.svelte with glassmorphism navigation shell [SMALL]
- [ ] 4.3: Update src/routes/+page.svelte as landing page [SMALL] (depends: 4.2)
- [ ] 4.4: Create managed club settings component [SMALL] (depends: 3.6, 4.2)

---
## Phase 5: Scouting Backend — Percentiles & Scoring (Rust) [PENDING]
- [ ] 5.0: Implement seed_archetypes() in db.rs [LARGE] (depends: 1.3)
- [ ] 5.1: Implement percentile computation [LARGE] (depends: 1.3)
- [ ] 5.2: Implement quality score computation [MEDIUM] (depends: 5.1)
- [ ] 5.3: Implement value score computation [MEDIUM] (depends: 5.1)
- [ ] 5.4: Implement archetype score pre-computation [LARGE] (depends: 5.0, 5.2, 5.3)
- [ ] 5.5: Implement get_archetypes Tauri command [SMALL] (depends: 5.0)
- [ ] 5.6: Implement get_scouting_results Tauri command [LARGE] (depends: 3.6, 5.4)
- [ ] 5.7: Register Phase 5 scouting commands in lib.rs invoke_handler [SMALL] (depends: 3.4, 5.5, 5.6)

---
## Phase 6: Scouting View UI — Core & Database View (Svelte 5) [PENDING]
- [ ] 6.1: Create src/routes/scouting/+page.svelte [MEDIUM] (depends: 4.2)
- [ ] 6.2: Implement Database View table [LARGE] (depends: 5.6, 6.1)
- [ ] 6.3: Implement comparison pool toggle [MEDIUM] (depends: 3.7, 6.2)
- [ ] 6.4: Implement player row click navigation [SMALL] (depends: 6.2)

---
## Phase 7: Scouting View UI — Role Search (Svelte 5) [PENDING]
- [ ] 7.1: Implement position-archetype dependency chain [MEDIUM] (depends: 5.5, 6.1)
- [ ] 7.2: Implement Top 3 Podium [MEDIUM] (depends: 6.2, 7.1)
- [ ] 7.3: Implement results table below podium [LARGE] (depends: 7.2)
- [ ] 7.4: Implement score mode toggle with debounce and animation [MEDIUM] (depends: 3.7, 7.3)
- [ ] 7.5: Implement scouting state preservation via Svelte 5 $state rune [SMALL] (depends: 7.3)
- [ ] 7.6: Implement user preference persistence [SMALL] (depends: 3.7, 6.3, 7.4)
- [ ] 7.7: Implement all empty states [SMALL] (depends: 6.1, 7.1)

---
## Phase 8: Build Verification & Polish [PENDING]
- [ ] 8.1: Run cargo build to verify all Rust code compiles; fix any issues [SMALL] (depends: 3.4, 5.7)
- [ ] 8.2: Run bun run check for frontend type-checking; fix errors [SMALL] (depends: 4.4, 7.7)
- [ ] 8.3: Run pre-check batch on all changed source files [SMALL] (depends: 8.1, 8.2)
