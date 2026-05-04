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

| Tool                    | Calls | Success | Failed | Avg Duration |
| ----------------------- | ----- | ------- | ------ | ------------ |
| read                    | 414   | 414     | 0      | 8ms          |
| bash                    | 138   | 138     | 0      | 3767ms       |
| glob                    | 94    | 94      | 0      | 15ms         |
| edit                    | 64    | 64      | 0      | 5ms          |
| task                    | 36    | 36      | 0      | 103064ms     |
| write                   | 16    | 16      | 0      | 7ms          |
| grep                    | 15    | 15      | 0      | 9ms          |
| update_task_status      | 12    | 12      | 0      | 9ms          |
| declare_scope           | 9     | 9       | 0      | 3ms          |
| syntax_check            | 9     | 9       | 0      | 11ms         |
| save_plan               | 8     | 8       | 0      | 12ms         |
| todowrite               | 7     | 7       | 0      | 3ms          |
| pre_check_batch         | 7     | 7       | 0      | 8ms          |
| get_approved_plan       | 6     | 6       | 0      | 3ms          |
| set_qa_gates            | 4     | 4       | 0      | 5ms          |
| search                  | 4     | 4       | 0      | 707ms        |
| test_runner             | 4     | 4       | 0      | 3ms          |
| get_qa_gate_profile     | 4     | 4       | 0      | 3ms          |
| placeholder_scan        | 3     | 3       | 0      | 6ms          |
| apply_patch             | 2     | 2       | 0      | 5ms          |
| submit_council_verdicts | 2     | 2       | 0      | 5ms          |
| lint                    | 2     | 2       | 0      | 5ms          |
| lint_spec               | 2     | 2       | 0      | 3ms          |
| sast_scan               | 2     | 2       | 0      | 7ms          |
| diff                    | 2     | 2       | 0      | 37ms         |
| imports                 | 2     | 2       | 0      | 87ms         |
| build_check             | 2     | 2       | 0      | 959ms        |
| question                | 1     | 1       | 0      | 19450ms      |
| secretscan              | 1     | 1       | 0      | 6ms          |
| webfetch                | 1     | 1       | 0      | 959ms        |
| knowledge_recall        | 1     | 1       | 0      | 2ms          |
| detect_domains          | 1     | 1       | 0      | 3ms          |
| check_gate_status       | 1     | 1       | 0      | 3ms          |
| symbols                 | 1     | 1       | 0      | 2ms          |
| checkpoint              | 1     | 1       | 0      | 5ms          |
| write_retro             | 1     | 1       | 0      | 6ms          |
| req_coverage            | 1     | 1       | 0      | 3ms          |
| write_drift_evidence    | 1     | 1       | 0      | 6ms          |
| knowledge_query         | 1     | 1       | 0      | 3ms          |
| phase_complete          | 1     | 1       | 0      | 129833ms     |
