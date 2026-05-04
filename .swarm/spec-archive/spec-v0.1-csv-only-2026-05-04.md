# CSV Parser Specification

## Summary

Implement the CSV data import pipeline for FM ValueScout, ingesting semicolon-delimited Football Manager player exports and persisting validated records to SQLite.

## Functional Requirements

### FR-001: Database Schema

Create SQLite schema with `players` table (immutable player identity fields), `seasons` table (import metadata), and `season_players` table (per-season player statistics with all 82+ metric columns).

### FR-002: SQLite Dependencies

Add rusqlite with bundled SQLite feature to Cargo.toml dependencies.

### FR-003: Database Initialization

Implement database connection management and automatic schema creation on app startup.

### FR-004: CSV Parsing Engine Scaffolding

Create modular parser architecture with a field parser that dispatches by column name to specialized parsers.

### FR-005: Currency & Magnitude Parser

Parse transfer values and wages with currency symbol stripping, K/M magnitude multiplication, upper-bound extraction from ranges, and decimal values before magnitude suffixes.

### FR-006: Wage Denomination Parser

Extract numeric wage value and denomination suffix (p/w, p/m, p/a) separately.

### FR-007: Unit Stripping Parser

Strip embedded units (cm, km) from numeric fields and apply field-specific precision.

### FR-008: Nationality Code Mapper

Map 3-letter nationality codes to full nation names via embedded lookup table.

### FR-009: Position Parser

Split comma-separated position strings into arrays of individual positions.

### FR-010: Footedness Parser

Map qualitative footedness strings to numeric scores while preserving raw strings.

### FR-011: Appearances Parser

Extract start count and substitute count from "43 (3)" format.

### FR-012: Computed Metrics

Compute per-90 metrics from totals, totals from per-90 values, and ratio metrics (pass/tackle/cross completion, header win %), with division-by-zero → 0 protection.

### FR-013: Row Validation

Reject rows with missing UID, missing player name, or duplicate UID within same import file. Validate numeric fields.

### FR-014: Full Row Parser

Parse a complete CSV row through all field parsers into a structured PlayerRecord, handling optional CA/PA columns.

### FR-015: Import CSV Tauri Command

Tauri command that reads CSV file, processes rows in batches with progress events, uses atomic SQLite transaction with rollback on failure. Handles UTF-8/Latin-1 encoding.

### FR-016: Import History Command

Tauri command to query past seasons/imports.

### FR-017: Command Registration

Register all Tauri commands and initialize database in the Tauri builder.

### FR-018: Import UI Page

Svelte 5 page with glassmorphism-styled: drop zone/file picker, in-game date input, import button (disabled until ready), progress bar, and summary card with success/error states.

### FR-019: App Layout

Create +layout.svelte with navigation shell for app pages.

### FR-020: Home Page Update

Update +page.svelte as landing page directing to import.

### FR-021: Rust Build Verification

`cargo build` succeeds without errors.

### FR-022: Frontend TypeCheck

`svelte-check` succeeds without errors.

## Non-Functional Requirements

- **NFR-001:** Atomic transactions — partial data never persisted on failure
- **NFR-002:** Streaming/batched processing for 100K+ row CSVs without memory exhaustion
- **NFR-003:** Glassmorphism design language consistency
- **NFR-004:** Fully offline operation — no network calls
