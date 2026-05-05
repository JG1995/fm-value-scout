<!-- PLAN_HASH: 1cg72ch4h61yf -->
# FM ValueScout — CSV Parser & Scouting View
Swarm: default
Phase: 1 [COMPLETE] | Updated: 2026-05-05T09:56:22.655Z

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
- [ ] 3.6: Implement set_managed_club and get_managed_club Tauri commands [SMALL] (depends: 1.3)
- [ ] 3.7: Implement set_preference and get_preference Tauri commands [SMALL] (depends: 1.3)
