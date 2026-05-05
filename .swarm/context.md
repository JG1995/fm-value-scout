# Context

Swarm: default

## Decisions

- Managed club identification: user sets club name once in app settings, stored in SQLite settings table
- 75/25 in/out-of-possession split: fixed for MVP, adjustment slider deferred to Archetype Management
- Season selection for scouting: always uses most recent imported season
- Test coverage: handled by test_engineer QA gate per-task, no dedicated test tasks in plan
- QA gates: reviewer, test_engineer, sme_enabled, critic_pre_plan, sast_enabled, drift_check all ON

## SME Cache

### sqlite, rust, data-modeling

- Wide table (not EAV) for season_players with 82+ columns
- PERCENT_RANK() window functions for percentile computation at import time
- WAL mode with PRAGMA: journal_mode=WAL, cache_size=-32000 (32MB), mmap_size=268435456 (256MB), foreign_keys=ON
- Index strategy: idx_spp_season_pos, idx_as_season_arch_score, idx_pp_lookup, idx_sp_season_mins
- Connection model: single Arc<Mutex<Connection>> in Tauri state
- Multi-position expansion: parse "D (LC), WB/M (L)" into canonical codes [DC,DL,WB_L,M_L]

### rust, csv-parsing

- csv crate with semicolon delimiter (b';'), encoding_rs for BOM + Latin-1 handling
- Parser registry pattern: HashMap<String, FieldParser> with column-name dispatch
- Currency regex: handle ranges (take upper bound), K/M/B suffixes, decimal before magnitude
- Wage regex: extract value + denomination (p/w, p/m, p/a) separately
- phf for compile-time nationality code lookup (200+ entries)
- Streaming: csv::Reader::records() is lazy, no OOM risk for 100K+ rows
- Dependencies: rusqlite (bundled), csv, encoding_rs, regex, once_cell, phf (macros), tauri-plugin-dialog

## Patterns

- Tauri command registration: generate_handler![] macro in lib.rs, commands annotated with #[tauri::command]
- Svelte 5 runes: $state() for reactive state, $state for scouting state preservation
- SvelteKit SPA mode: ssr=false in +layout.ts, adapter-static with fallback
- Glassmorphism design: translucent card panels, backdrop blur, dark gradient background
- SQLite settings table: generic key-value store for all user preferences

## Agent Activity

| Tool | Calls | Success | Failed | Avg Duration |
|------|-------|---------|--------|--------------|
| read | 463 | 463 | 0 | 16ms |
| bash | 276 | 276 | 0 | 3126ms |
| glob | 124 | 124 | 0 | 19ms |
| task | 69 | 69 | 0 | 140640ms |
| search | 66 | 66 | 0 | 645ms |
| grep | 65 | 65 | 0 | 17ms |
| edit | 65 | 65 | 0 | 9ms |
| update_task_status | 42 | 42 | 0 | 12ms |
| declare_scope | 24 | 24 | 0 | 5ms |
| test_runner | 23 | 23 | 0 | 3ms |
| todowrite | 21 | 21 | 0 | 5ms |
| syntax_check | 19 | 19 | 0 | 25ms |
| placeholder_scan | 19 | 19 | 0 | 22ms |
| write | 18 | 18 | 0 | 11ms |
| pre_check_batch | 11 | 11 | 0 | 16ms |
| check_gate_status | 11 | 11 | 0 | 3ms |
| imports | 9 | 9 | 0 | 3ms |
| diff | 6 | 6 | 0 | 69ms |
| build_check | 6 | 6 | 0 | 1483ms |
| phase_complete | 6 | 6 | 0 | 81941ms |
| save_plan | 4 | 4 | 0 | 21ms |
| lint | 4 | 4 | 0 | 6ms |
| evidence_check | 3 | 3 | 0 | 11ms |
| get_qa_gate_profile | 3 | 3 | 0 | 8ms |
| get_approved_plan | 3 | 3 | 0 | 12ms |
| todo_extract | 3 | 3 | 0 | 5ms |
| write_drift_evidence | 3 | 3 | 0 | 9ms |
| sast_scan | 2 | 2 | 0 | 18ms |
| knowledge_recall | 2 | 2 | 0 | 5ms |
| extract_code_blocks | 2 | 2 | 0 | 6ms |
| symbols | 2 | 2 | 0 | 3ms |
| checkpoint | 2 | 2 | 0 | 19ms |
| write_retro | 2 | 2 | 0 | 12ms |
| req_coverage | 2 | 2 | 0 | 4ms |
| set_qa_gates | 1 | 1 | 0 | 19ms |
| invalid | 1 | 1 | 0 | 3ms |
| diff_summary | 1 | 1 | 0 | 8ms |
| completion_verify | 1 | 1 | 0 | 6ms |
