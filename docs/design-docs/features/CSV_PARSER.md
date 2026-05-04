# Feature: CSV Parser

## 1. Executive Summary

**High-Level Goal:** The CSV Parser ingests player data exported from Football Manager, transforming semicolon-delimited CSV files into structured, validated player records persisted to the local SQLite database. It is the single entry point for all data in the application — without it, no scouting, squad management, or player analysis is possible.

**Context:** This is the Data Import feature described in the [Concept Document](../CONCEPT.MD). Each CSV represents a season snapshot of the game world. The parser normalises variable currency formats, magnitude suffixes, embedded units, optional columns, and qualitative footedness strings into a consistent, queryable schema. It is the foundation that every other feature reads from.

## 2. Information Architecture (The "Memory Bank")

### User Inputs

- **CSV file:** A semicolon-delimited file exported from Football Manager containing 82 columns of player data. The user selects this file through the app's file picker.
- **In-game date:** A date input (day/month/year) representing the current date in the user's Football Manager save at the time of export. Required for computing contract expiry status and other date-dependent calculations.

### Displayed Data

- **Pre-import:** The file picker UI, an in-game date input field, and a confirmation/import button.
- **During import:** A progress indicator (the parser processes potentially thousands of rows) and a running count of successfully parsed vs. rejected rows.
- **Post-import:** A summary screen showing total players imported, rows rejected (with reasons), and the season label applied. A confirmation that data is now available for scouting.

### Persistent Data

The parser writes to the local SQLite database:

- **Player records:** One row per player, keyed by the in-game Unique ID. If a player already exists in the database (from a prior season import), the new season's data is appended as an additional season layer — existing records are never overwritten. This enables longitudinal career timeline analysis.
- **Season metadata:** The in-game date provided by the user, the import timestamp, and the filename of the source CSV.
- **The system must remember** which players belong to which season so that the user can filter analysis by season and scroll through a player's career history.

## 3. The User Journey (Step-by-Step)

1. **Entry Point:** The user navigates to the Data Import section (accessed from the main navigation) and clicks "Import CSV."
2. **Action 1:** The user selects a CSV file from their filesystem via the native file picker dialog.
3. **System Response 1:** The app reads the filename and displays it alongside an in-game date input field, defaulting to today's date. The "Import" button remains disabled until both the file is selected and a date is provided.
4. **Action 2:** The user enters the in-game date (matching the date in their FM save at the time the CSV was exported) and clicks "Import."
5. **System Response 2:** The parser begins processing the CSV row by row. A progress bar or counter updates as rows are parsed. The parser:
   - Reads the header row to identify available columns (handling optional CA/PA columns).
   - Parses each data row through the full pipeline: split on semicolons → parse each field → validate → enrich with computed metrics → prepare for database insertion.
   - Rejects rows that fail validation (e.g., missing Unique ID) and logs the rejection reason.
6. **Success State:** A summary card appears showing "Import Complete" with the number of players imported, the number of rows rejected (if any), and the season label (derived from the in-game date). A "Go to Scouting" button navigates the user to the scouting view where the imported players are immediately searchable.

## 4. Logical Constraints (The "Rules of the Road")

### Parsing Rules

- **IF** the CSV uses semicolons as delimiters, **THEN** the parser splits each row on `;` to extract 82 fields.
- **IF** a column header is missing from the CSV (e.g., CA or PA), **THEN** the parser treats that column as absent for all rows and stores `NULL` rather than failing.
- **IF** a row's Unique ID field is empty or non-numeric, **THEN** the row is rejected with reason "Missing UID."
- **IF** a row's Unique ID matches a player already in the database, **THEN** the new data is appended as a new season layer for that player (the existing season data is never overwritten).

### Currency & Magnitude Parsing

- **IF** a transfer value or wage contains a magnitude suffix (K for thousands, M for millions), **THEN** the parser multiplies the numeric portion accordingly (e.g., `€55M` → 55,000,000).
- **IF** a transfer value contains a range (e.g., `€160M - €210M`), **THEN** only the upper bound (210,000,000) is stored.
- **IF** a transfer value has no magnitude suffix, **THEN** the raw number is stored as-is.
- **IF** the currency symbol varies from the sample data (e.g., `£` instead of `€`), **THEN** the parser handles whatever currency the user's CSV provides without assuming a specific symbol.
- **IF** a wage includes a denomination suffix (`p/w`, `p/m`, `p/a`), **THEN** both the numeric value and the denomination are stored separately, because denomination affects downstream calculations (e.g., annual wage vs. weekly wage comparisons).

### Unit & Precision Handling

- **IF** a field contains an embedded unit (e.g., `176 cm`, `312.7km`), **THEN** the parser strips the unit and stores the numeric value at the field's designated precision:
  - Height: integer (cm stripped)
  - Distance covered: 1 decimal place (km stripped)
  - Per-90 metrics: 1 or 2 decimal places as specified per field
  - xG, xA, xGP: 2 decimal places (may be negative)
  - Average Rating: 2 decimal places
- **IF** a field that expects a number contains non-numeric characters beyond the expected unit, **THEN** the row is rejected with reason "Invalid numeric value in field [field name]."

### Nationality & Position Parsing

- **IF** the Nation field is a 3-letter code (e.g., `BRA`, `ESP`), **THEN** the parser maps it to the full nation name via a lookup table embedded in the application.
- **IF** the 2nd Nationality field is a full name (e.g., `Spain`, `St Kitts & Nevis`), **THEN** it is stored as-is.
- **IF** the Position field contains a comma-separated list (e.g., `D (LC), WB/M (L)`), **THEN** the parser splits it into an array of position strings.

### Footedness Parsing

- **IF** the Left Foot or Right Foot field contains a qualitative string (e.g., `Very Strong`, `Fairly Strong`, `Reasonable`, `Weak`), **THEN** the parser optionally maps these to a numeric score. This mapping is performed at parse time but the raw string is also preserved.
- **IF** the qualitative string is unrecognised, **THEN** the raw string is stored and the numeric score is set to `NULL`.

### Appearances Parsing

- **IF** the Appearances field is in the format `43 (3)`, **THEN** the parser extracts both values: 43 starts and 3 substitute appearances. Both are stored separately.
- **IF** the Appearances field has no parenthetical (e.g., `39`), **THEN** the substitute count is 0.

### Computed Metrics

- **IF** a metric's "per 90" variant is not directly in the CSV, **THEN** the parser computes it from the total value divided by (Minutes / 90), rounded to the field's specified decimal places.
- **IF** a metric's total variant is not directly in the CSV (indicated by `COMPUTED` in the metrics specification), **THEN** the parser computes it from the per-90 value multiplied by (Minutes / 90).
- **IF** a ratio metric is not directly in the CSV, **THEN** the parser computes it from the component metrics (e.g., Pass Completion Ratio = Passes Completed / Passes Attempted).
- **IF** the denominator for any computed ratio is zero, **THEN** the ratio is stored as `NULL` rather than causing a division-by-zero error.

### Validation

- **IF** the Minutes field is 0, **THEN** per-90 metrics cannot be computed and are stored as `NULL`. The row is still imported — a player with 0 minutes is valid (unused substitute).
- **IF** the player Name field is empty, **THEN** the row is rejected with reason "Missing player name."
- **IF** the Club field is empty, **THEN** the row is still imported; the player may be a free agent.

## 5. Negative Paths & Edge Cases (The "What-Ifs")

### Error Handling

- **Corrupt CSV:** If the file cannot be parsed at all (e.g., binary file, wrong encoding, no semicolons found), the import fails immediately with a user-facing error: "This file does not appear to be a valid Football Manager CSV export. Please check the file and try again."
- **Missing header row:** If the first row does not contain the expected column headers, the parser attempts to match by position. If the column count is wildly wrong (fewer than 40 columns), the import is aborted.
- **Encoding issues:** The parser handles both UTF-8 and Latin-1 encoded CSVs. If a row contains unparseable characters, the row is rejected with reason "Encoding error at row N."

### Empty States

- **Empty CSV:** If the file contains only a header row with no data rows, the import "succeeds" with a message: "No player data found in this file. 0 players imported."
- **All rows rejected:** If every row fails validation, the import completes with: "0 players imported. All N rows were rejected. See details below." The rejection reasons are displayed in a scrollable list.

### Boundary Cases

- **Extremely large CSV:** A CSV with 100,000+ rows is processed in a streaming fashion — rows are parsed and inserted in batches to avoid memory exhaustion. The progress indicator reflects batch progress.
- **Single-row CSV:** A CSV with only one player imports normally.
- **Duplicate Unique IDs within the same CSV:** If two rows in the same file share a Unique ID (data error in the export), the first row is imported and the second is rejected with reason "Duplicate UID within import file."
- **Negative metrics:** Several metrics can be negative (xG Overperformance, xGP). The parser must accept and store negative floating-point values.
- **Unusually formatted values:** Values like `€19.25K p/w` (decimal before magnitude suffix) and `€36.5K p/w` must parse correctly. The parser extracts the full numeric portion before applying the magnitude multiplier.
- **Wage magnitudes without currency:** Some wages may appear as just `€28M` (no p/w, p/m, p/a). The parser treats missing denomination as `NULL` and flags it for the user.

### Interruptions

- **App closed mid-import:** The import is an atomic transaction. If the app closes or crashes during import, the database is rolled back to its pre-import state. No partial data is persisted.
- **File changed mid-import:** The file is read entirely into a buffer before parsing begins. If the user replaces the file on disk during import, it has no effect.
- **Second import during first:** The UI disables the import button while an import is in progress. A second import cannot be initiated until the first completes.

### Time & Date Edge Cases

- **In-game date in the future:** The user may enter any date. No validation is performed against the system clock. The date is stored as-provided.
- **Contract already expired:** If the contract expiry date is before the provided in-game date, the contract status is computed as "Expired" — this is valid (the player is on a month-to-month or has already left).

## 6. Interface & Interaction (The "Look and Feel")

### Visual Style

Consistent with the glassmorphism design language of the application. The import interface uses translucent card panels with subtle backdrop blur, set against the app's dark gradient background.

### Layout

- **File selection:** A large dashed-border drop zone with the text "Drop your FM CSV export here or click to browse." Below it, the selected filename is displayed with a small file icon.
- **Date input:** A date picker or three dropdowns (day/month/year) with the label "In-game date at time of export." Helper text below: "This is used to calculate contract status and time-based metrics."
- **Import button:** A prominent glass-style button labelled "Import Season Data." Disabled (greyed out) until both file and date are provided. When enabled, uses the app's accent colour.
- **Progress:** A determinate progress bar with the label "Processing row N of M..." and a count of rejected rows appearing below it.
- **Summary:** A card with a large checkmark icon, the count of imported players, and two action buttons: "View in Scouting" (primary) and "Import Another" (secondary).

### Copywriting

- **Drop zone:** "Drop CSV here or click to browse"
- **Date label:** "In-game date"
- **Date helper text:** "Match this to the date in your FM save when you exported the CSV."
- **Import button:** "Import Season Data"
- **Importing state:** "Importing..." (button disabled, shows spinner)
- **Success title:** "Import Complete"
- **Success detail:** "[N] players imported. [M] rows skipped."
- **Error - bad file:** "This doesn't look like a valid FM CSV export. Check the file and try again."
- **Error - no rows:** "No player data found in this file."
- **Skipped rows toggle:** "Show skipped rows"

## 7. Acceptance Criteria (The "Mission Accomplished" Checklist)

- [ ] I can select a valid FM CSV file and provide an in-game date, and all 82 fields are parsed correctly across every row.
- [ ] Transfer values with ranges (e.g., `€160M - €210M`) store only the upper bound with magnitude multiplier applied.
- [ ] Wages with denominations (`p/w`, `p/m`, `p/a`) store both the numeric value and the denomination separately.
- [ ] Currency symbols are handled dynamically — the parser does not assume `€`.
- [ ] Optional CA/PA columns are handled gracefully: present columns are parsed, absent columns result in `NULL` values with no errors.
- [ ] Embedded units (`cm`, `km`) are stripped and numeric values stored at the correct precision.
- [ ] 3-letter nationality codes are mapped to full nation names.
- [ ] Position strings are parsed into arrays of individual positions.
- [ ] Qualitative footedness strings are stored raw with an optional numeric score mapping.
- [ ] Appearances in `43 (3)` format extract both start and substitute counts.
- [ ] Per-90 metrics are computed from totals where the CSV provides only totals.
- [ ] Total metrics are computed from per-90 values where the CSV provides only per-90 values.
- [ ] Ratio metrics are computed from component metrics where not directly in the CSV.
- [ ] Division-by-zero during ratio computation produces `NULL`, not an error.
- [ ] Rows with missing Unique ID or empty Name are rejected with a specific reason.
- [ ] Duplicate UIDs within the same import file are rejected on the second occurrence.
- [ ] Negative floating-point values (xG Overperformance, xGP) are parsed and stored correctly.
- [ ] Decimal values with magnitude suffixes (e.g., `€19.25K`) parse correctly.
- [ ] An already-imported player (same UID, different season) appends a new season layer rather than overwriting existing data.
- [ ] If the app is terminated mid-import, no partial data persists (atomic transaction rollback).
- [ ] A 100,000-row CSV is processed without memory exhaustion (streaming/batched insert).
- [ ] The UI matches the glassmorphism design language: translucent panels, backdrop blur, dark gradient background, accent-coloured primary button.
