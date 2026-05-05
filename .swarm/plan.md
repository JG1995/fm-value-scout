<!-- PLAN_HASH: 2a9wpsrnkppzw -->
# FM ValueScout — CSV Parser & Scouting View
Swarm: default
Phase: 1 [PENDING] | Updated: 2026-05-05T14:03:11.234Z

---
## Phase 1: Database Foundation (Rust) [PENDING]
- [ ] 1.1: Add rusqlite (bundled), csv, encoding_rs, regex, once_cell, phf (macros feature) to Cargo.toml; add tauri-plugin-dialog to Cargo.toml; add @tauri-apps/plugin-dialog to package.json npm dependencies [MEDIUM]
- [ ] 1.2: Create src-tauri/src/db.rs with full schema [LARGE]
- [ ] 1.3: Implement database init: open SQLite connection with WAL mode [MEDIUM]

---
## Phase 2: CSV Parsing Engine (Rust) [PENDING]
- [ ] 2.1: Create src-tauri/src/parsers/mod.rs with FieldValue enum, PlayerRecord struct, Schema struct [MEDIUM]
- [ ] 2.2: Implement currency/magnitude parser [MEDIUM]
- [ ] 2.3: Implement wage denomination parser [MEDIUM]
- [ ] 2.4: Implement unit-stripping parser for cm/km suffixes [SMALL]
- [ ] 2.5: Implement nationality code mapper using phf hash map [SMALL]
- [ ] 2.6: Implement position string splitter [SMALL]
- [ ] 2.7: Implement footedness parser [SMALL]
- [ ] 2.8: Implement appearances parser from '43 (3)' format [SMALL]
- [ ] 2.9: Implement computed metrics: per-90, totals, ratios [MEDIUM]
- [ ] 2.10: Implement CSV row validator [MEDIUM]
- [ ] 2.11: Implement full row parser orchestrator [LARGE]
- [ ] 2.12: Implement position expansion for compound FM positions [MEDIUM]

---
## Phase 3: Tauri Commands & Integration (Rust) [PENDING]
- [ ] 3.1: Implement encoding detection utility: try UTF-8 decode first, fall back to Windows-1252/Latin-1; strip BOM prefix before CSV reading [SMALL]
- [ ] 3.2: Implement import_csv Tauri command: read file via dialog, detect encoding, stream CSV, parse rows through pipeline, batch insert 1000 rows per transaction [LARGE]
- [ ] 3.3: Implement get_seasons Tauri command returning all past imports [SMALL]
- [ ] 3.4: Register all Phase 3 Tauri commands in lib.rs invoke_handler [SMALL]
- [ ] 3.5: Add Tauri capabilities: dialog:default and fs:read with allowed paths [SMALL]
- [ ] 3.6: Implement set_managed_club and get_managed_club Tauri commands [SMALL]
- [ ] 3.7: Implement set_preference and get_preference Tauri commands [SMALL]

---
## Phase 4: Frontend Import UI (Svelte 5) [PENDING]
- [ ] 4.1: Create src/routes/import/+page.svelte with drop zone, file picker, in-game date input, import button, progress bar, summary card [LARGE]
- [ ] 4.2: Create src/routes/+layout.svelte with navigation shell [SMALL]
- [ ] 4.3: Update src/routes/+page.svelte as landing page [SMALL]
- [x] 4.4: Create managed club settings component [SMALL]
