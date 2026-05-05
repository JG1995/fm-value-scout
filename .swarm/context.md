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
| read | 334 | 334 | 0 | 18ms |
| bash | 187 | 187 | 0 | 1514ms |
| glob | 109 | 109 | 0 | 19ms |
| search | 59 | 59 | 0 | 385ms |
| grep | 53 | 53 | 0 | 17ms |
| task | 50 | 50 | 0 | 122102ms |
| edit | 35 | 35 | 0 | 10ms |
| update_task_status | 28 | 28 | 0 | 15ms |
| test_runner | 19 | 19 | 0 | 4ms |
| write | 15 | 15 | 0 | 11ms |
| declare_scope | 14 | 14 | 0 | 5ms |
| syntax_check | 14 | 14 | 0 | 26ms |
| placeholder_scan | 14 | 14 | 0 | 25ms |
| todowrite | 11 | 11 | 0 | 6ms |
| imports | 8 | 8 | 0 | 4ms |
| check_gate_status | 6 | 6 | 0 | 4ms |
| build_check | 5 | 5 | 0 | 1428ms |
| pre_check_batch | 5 | 5 | 0 | 19ms |
| diff | 4 | 4 | 0 | 78ms |
| evidence_check | 3 | 3 | 0 | 11ms |
| save_plan | 3 | 3 | 0 | 20ms |
| lint | 3 | 3 | 0 | 6ms |
| todo_extract | 3 | 3 | 0 | 5ms |
| get_qa_gate_profile | 2 | 2 | 0 | 10ms |
| get_approved_plan | 2 | 2 | 0 | 14ms |
| sast_scan | 2 | 2 | 0 | 18ms |
| knowledge_recall | 2 | 2 | 0 | 5ms |
| symbols | 2 | 2 | 0 | 3ms |
| checkpoint | 2 | 2 | 0 | 19ms |
| phase_complete | 2 | 2 | 0 | 142354ms |
| write_drift_evidence | 2 | 2 | 0 | 8ms |
| set_qa_gates | 1 | 1 | 0 | 19ms |
| extract_code_blocks | 1 | 1 | 0 | 8ms |
| invalid | 1 | 1 | 0 | 3ms |
| diff_summary | 1 | 1 | 0 | 8ms |
| write_retro | 1 | 1 | 0 | 9ms |
| req_coverage | 1 | 1 | 0 | 3ms |
| completion_verify | 1 | 1 | 0 | 6ms |
