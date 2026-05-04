# Specification: FM ValueScout — CSV Parser & Scouting View

## Feature Description

FM ValueScout is a desktop companion application for Football Manager players who use moneyball-style scouting — identifying players who are statistically excellent yet undervalued by the transfer market. This specification covers two features:

1. **CSV Parser (Data Import)**: Ingests semicolon-delimited player data exported from Football Manager, normalizing diverse currency formats, magnitude suffixes, embedded units, optional columns, and qualitative attributes into structured, validated player records persisted to a local database. It is the single entry point for all data in the application.

2. **Scouting View**: Surfaces a ranked shortlist of transfer candidates for any position and archetype, scored by both raw statistical quality and a value-adjusted metric that rewards undervalued talent. It provides both a full database browse mode and a targeted role search with a Top 3 podium.

---

## Functional Requirements

### FR-001: CSV File Selection

The user MUST be able to select a semicolon-delimited CSV file exported from Football Manager via a file picker dialog.

- Scenario: User clicks "Import CSV" → native file picker opens → user selects a `.csv` file → filename is displayed
- Failure: File is not a CSV or is unreadable → show error message

### FR-002: In-Game Date Input

The user MUST provide the in-game date matching their Football Manager save at the time of export. This is required to compute contract expiry status.

- Scenario: User enters day/month/year → date is stored with the import
- The import button MUST remain disabled until both file and date are provided

### FR-003: CSV Parsing — Semicolon Delimiter

The parser MUST split each CSV row on the `;` character to extract the individual player fields.

- If the file does not contain semicolons, the import MUST fail with a descriptive error

### FR-004: CSV Parsing — Header Handling

The parser MUST read the first row as column headers and map fields by name. If optional columns (CA, PA) are absent, those fields MUST be stored as empty/NULL rather than causing import failure.

- Scenario: CSV has 55 columns (missing CA and PA headers) → parser identifies available columns → CA and PA stored as NULL for all rows

### FR-005: CSV Parsing — Missing Unique ID

If a row's Unique ID field is empty or non-numeric, the parser MUST reject that row with reason "Missing UID" and continue processing remaining rows.

### FR-006: CSV Parsing — Missing Player Name

If a row's Player Name field is empty, the parser MUST reject that row with reason "Missing player name" and continue processing.

### FR-007: CSV Parsing — Duplicate UID Within Import

If two rows within the same CSV share the same Unique ID, the first occurrence MUST be imported and the second MUST be rejected with reason "Duplicate UID within import file."

### FR-008: Currency & Magnitude Parsing

The parser MUST strip any currency symbol (€, £, $, etc.) and apply magnitude multipliers (K=×1000, M=×1,000,000) to transfer values and wages.

- `€55M` → 55,000,000
- `€19.25K` → 19,250
- `€160M - €210M` → stores upper bound 210,000,000
- No magnitude suffix → raw number stored as-is

### FR-009: Wage Denomination Handling

Wage values MUST have the numeric amount and denomination suffix (p/w, p/m, p/a) stored separately. If no denomination is present, the denomination MUST be empty/NULL.

- `€74K p/w` → value=74,000, denomination="p/w"
- `€28M` → value=28,000,000, denomination=NULL

### FR-010: Unit Stripping

Fields containing embedded units (cm, km) MUST have their unit suffix stripped and the numeric value stored at the correct precision: height as integer (cm), distance as 1 decimal place (km), xG/xA as 2 decimal places, average rating as 2 decimal places.

- `199 cm` → 199 (integer)
- `312.7km` → 312.7 (1 dp)

### FR-011: Nationality Code Mapping

Three-letter nationality codes MUST be mapped to full nation names via a lookup table. Already-full names MUST be preserved as-is.

- `UKR` → "Ukraine"
- `Spain` → "Spain" (unchanged)

### FR-012: Position String Parsing

Comma-separated position strings MUST be split into an array of individual position strings.

- `D (LC), WB/M (L)` → ["D (LC)", "WB/M (L)"]

### FR-013: Footedness Scoring

Qualitative footedness strings MUST be mapped to numeric scores while preserving the raw string. Unrecognized strings MUST result in a NULL numeric score.

- `Very Strong` → score=20, `Fairly Strong` → 15, `Reasonable` → 10, `Weak` → 5
- Unknown qualitative strings → score=NULL, raw string preserved

### FR-014: Appearances Parsing

Appearances in "starts (subs)" format MUST be split into separate start count and substitute count. Values without parentheticals MUST have a substitute count of 0.

- `43 (3)` → starts=43, subs=3
- `39` → starts=39, subs=0

### FR-015: Per-90 Metric Computation

When the CSV provides a total metric but not its per-90 equivalent, the per-90 value MUST be computed as (total / minutes) × 90. If minutes is 0, the per-90 value MUST be stored as 0.

- Per-90 Goals = (total Goals / Minutes) × 90, rounded to 2 decimal places

### FR-016: Ratio Metric Computation

When the CSV provides component metrics but not the computed ratio, the ratio MUST be computed from the components. Division by zero MUST produce 0 rather than an error.

- Pass Completion Ratio = Passes Completed / Passes Attempted. If denominator is 0 → result is 0.

### FR-017: Negative Metrics

Metrics that can be negative (e.g., xG Overperformance, xGP) MUST be parsed and stored as signed floating-point values.

- Scenario: xG Overperformance field contains `-2.35` → stored as -2.35

### FR-018: Multi-Season Data Model

When a player with an existing Unique ID is imported from a new season's CSV, the new data MUST be appended as an additional season layer. Existing season data MUST never be overwritten.

- Scenario: Player 71101334 exists with 2028 season → import 2029 CSV → player now has both 2028 and 2029 season records

### FR-019: Atomic Import Transaction

The entire CSV import MUST execute as a single atomic transaction. If the application is terminated mid-import, no partial data MUST be persisted.

- Scenario: Import 10,000 rows → crash at row 5,423 → database rolls back to pre-import state

### FR-020: Import Progress Reporting

During import, the application MUST display a progress indicator showing the number of rows processed, successfully imported, and rejected.

### FR-021: Import Summary

After import completes, the application MUST display a summary showing total players imported, rows rejected with reasons, and the season label applied.

### FR-022: File Encoding Detection

The parser MUST handle both UTF-8 (with optional BOM) and Latin-1 encoded CSV files. Invalid characters due to wrong encoding MUST be rejected with a specific error reason.

- Scenario: CSV opened as UTF-8 but actually Latin-1 → parser detects and re-reads with correct encoding
- Scenario: Row contains unparseable characters → row rejected with reason "Encoding error at row N"

### FR-023: Streaming Processing

A CSV with 100,000+ rows MUST be processed in a streaming fashion without exhausting available memory. Rows SHOULD be inserted in batches.

### FR-024: Database View — Browse All Players

The Scouting View MUST provide a Database View mode that displays every player in the loaded database in a sortable table with color-coded percentile scores.

### FR-025: Database View — Column Sorting

Clicking a column header in the Database View MUST sort the table by that column. Clicking the same header again MUST reverse the sort direction. Numeric columns MUST sort numerically, not lexicographically.

### FR-026: Role Search — Position & Archetype Selection

The Scouting View MUST provide a Role Search mode with a position dropdown and an archetype dropdown. Selecting a position MUST populate the archetype dropdown with only archetypes relevant to that position. Selecting an archetype MUST automatically trigger a search.

- Scenario: User selects "Center Back" → archetype dropdown shows only Center Back archetypes
- Scenario: User selects "Ball-Playing Center Back" → results appear automatically

### FR-027: Role Search — Quality Score Computation

For the selected archetype, each player's quality score MUST be computed as the weighted sum of their metric percentiles: `score = Σ(weight_i × percentile_i)`, normalized to a 0–100 scale.

- Metrics marked as inverted (lower is better) MUST have their percentiles computed on the inverted distribution

### FR-028: Role Search — Value Score Computation

When Value Score mode is active, each player's value score MUST equal their quality score multiplied by a transfer-value-based multiplier:

- Cheapest 10% of players: multiplier = 1.5
- Most expensive 10%: multiplier = 0.5
- Between 10th and 90th percentile: linear interpolation between 1.5 and 0.5
- Players with no transfer value MUST be treated as cheapest percentile (×1.5)

### FR-029: Role Search — Top 3 Podium

Role Search results MUST display a Top 3 Podium showing the three highest-scoring players by the active score mode. The podium MUST reorder when the score mode changes between Quality and Value.

### FR-030: Role Search — Results Table

Below the podium, a sortable table MUST display all qualifying players with their scores and top-weighted archetype metrics. The table MUST be sortable by any column header.

### FR-031: Role Search — Minimum Minutes Filter

Players with fewer than 1,000 minutes in the most recent imported season MUST be excluded from Role Search results. Multi-season scouting is deferred to a future iteration.

### FR-032: Role Search — Own Club Exclusion

The user's managed club MUST be set once in application settings and stored persistently across sessions. Players belonging to the user's managed club MUST be excluded from Role Search results.

### FR-033: Comparison Pool Toggle

The Scouting View MUST provide a toggle between "Against Position" (percentiles computed against same-position players only) and "Against All Positions" (percentiles computed against the entire database). Switching MUST immediately recalculate and recolour all displayed percentiles.

### FR-034: Percentile Cell Colouring

Table cells displaying percentile values MUST be colour-coded on a red-to-yellow-to-green gradient corresponding to low-to-high percentile ranks.

### FR-035: Score Mode Toggle

The Scouting View MUST provide a toggle between Quality Score and Value Score modes. Rapid toggling MUST be debounced so only the final state triggers recomputation.

### FR-036: State Preservation

When navigating from the Scouting View to a Player Profile and back, the Scouting View MUST be restored in its previous state (same mode, same position/archetype selection, same score mode, same comparison pool, same sort order).

### FR-037: User Preference Persistence

The user's score mode and comparison pool preferences MUST persist across application sessions and be restored on the next visit.

### FR-038: Empty State — No Database

If no CSV has been imported, the Scouting tab MUST display a prompt directing the user to import data, with a button linking to the Data Import section.

### FR-039: Empty State — No Results

If a Role Search yields zero qualifying players, the application MUST display a message suggesting the user adjust their filters.

### FR-040: In-Possession / Out-of-Possession Variants

Archetypes with both in-possession and out-of-possession variants MUST compute a combined score using a fixed 75/25 split (75% in-possession, 25% out-of-possession). A user-adjustable split is deferred to Archetype Management (out of scope for this feature set).

### FR-041: Tiebreaking

If two players have identical scores on the podium, the tie MUST be broken by the secondary score (Value Score breaks ties by Quality Score, and vice versa).

### FR-042: Transfer Value of Zero

A player with a transfer value of exactly zero (distinct from no value listed) MUST be treated as cheapest percentile for value score computation.

### FR-043: Podium — Fewer Than Three Candidates

If the selected position has fewer than 3 eligible players, the podium MUST display only the available players. Empty podium slots MUST show a placeholder indicating insufficient players.

---

## Success Criteria

### SC-001: CSV Import Completeness

Given a valid FM CSV file with 10,000 rows, all 10,000 players are parsed and imported with 0 parse errors on correctly-formatted rows. End-to-end measured by comparing row count in imported database against CSV row count minus rejected rows.

### SC-002: Currency Edge Cases

Given CSV rows containing `€160M - €210M`, `€19.25K`, `£5M`, and `€0`, all four parse to correct numeric values (210,000,000 / 19,250 / 5,000,000 / 0) with 100% accuracy.

### SC-003: Wage Parsing Accuracy

Given wage values `€74K p/w`, `€36.5K p/w`, and `€28M`, all three have correctly separated numeric values and denominations with 100% accuracy.

### SC-004: Unit Stripping Precision

Given height `199 cm`, distance `312.7km`, xG `0.45`, all are stored at correct precision (199 as integer, 312.7 as 1dp, 0.45 as 2dp).

### SC-005: Multi-Season Append

Importing the same player (UID=71101334) from a second CSV adds a new season layer. The player has two season records after import. Existing records are unmodified.

### SC-006: Rejected Row Count

Given a CSV where 5% of rows have missing UIDs, import completes with those rows rejected and rejection reasons logged. Successfully parsed rows are unaffected.

### SC-007: Atomic Rollback

Terminating the import process mid-way leaves the database with zero records from the incomplete import — verified by querying season_players count before and after forced termination.

### SC-008: Streaming Memory

Processing a CSV with 150,000 rows does not exceed 200MB of memory usage. Memory is measured by process RSS before and during import.

### SC-009: Database View — All Players Loaded

Given a database with N players, the Database View table renders N rows within 2 seconds of entering the Scouting tab.

### SC-010: Sort Responsiveness

Clicking a column header sorts the Database View table and displays results within 500ms for up to 10,000 rows.

### SC-011: Archetype Scoring Accuracy

Given a known player data set and a defined archetype with weights [metric_A: 3, metric_B: 2, metric_C: 1], the quality score computation matches a hand-calculated result to within 0.1 points.

### SC-012: Value Score Multiplier

Given 100 players with transfer values evenly distributed €0–€100M, the cheapest player gets ×1.5 multiplier and the most expensive gets ×0.5 multiplier. Middle-ranked players get interpolated values accurate to within 0.01.

### SC-013: Comparison Pool Toggle

Toggling from "Against Position" to "Against All Positions" recolours all percentile cells within 500ms for up to 5,000 displayed players.

### SC-014: Podium Reorder Animation

Toggling Quality/Value score mode triggers a podium reorder animation completing within 400ms.

### SC-015: State Preservation

Navigating from Role Search (with Center Back / Ball-Playing CB / Value / Against All Position selected) to a Player Profile and back restores the exact same state — verified by screenshot comparison.

### SC-016: Preference Persistence

Setting Value Score + Against All Positions, closing the application, and reopening restores those preferences on the Scouting tab. Verified across multiple restart cycles.

### SC-017: Empty State Display

With an empty database, the Scouting tab shows the correct empty-state message and an active "Import CSV" link button.

---

## Key Entities

- **Player**: Immutable identity — Unique ID, Name, Nation (mapped from code), Second Nationality, Height, Left Foot (raw + score), Right Foot (raw + score)
- **Season**: Import metadata — In-Game Date, Import Timestamp, Source Filename, Player Count
- **Season Player**: Per-season statistics — Club, Age, Position, Appearances (starts + subs), Minutes, and 80+ metric fields across categories: Attacking/Finishing, Creativity/Chance Creation, Transition/Ball Progression, Defensive Actions, Aerial Presence, Goalkeeping/Shot Stopping, Discipline/Error Margins, Match Impact/Availability
- **Archetype**: Role definition — Name, Base Position, Weighted Metrics List (with inversion flags), Quality Score Formula, In-Possession and Out-of-Possession variants
- **Archetype Score**: Pre-computed per-player-per-archetype — Quality Score (0–100), Value Score (quality × value multiplier)
- **Percentile**: Per-player-per-metric rank — Percent Rank (0.0–1.0), Pool Type (Against Position or Against All), Canonical Position (for position-pool percentiles)

## Edge Cases & Failure Modes

- **Corrupt binary file**: Import fails immediately with "not a valid FM CSV" error
- **Wrong delimiter**: If file uses comma or tab instead of semicolon, first-row validation catches it
- **Empty CSV (header only)**: Import "succeeds" with 0 players imported message
- **All rows rejected**: Import completes with detailed rejection list, 0 players imported
- **Duplicate UID in same file**: First instance imported, second rejected
- **Negative metrics**: xG Overperformance, xGP stored as signed floats
- **Transfer value of exactly €0**: Treated as cheapest percentile (×1.5), not as missing value
- **0 minutes played**: Player imported with per-90 metrics as 0 (valid for unused substitutes)
- **Missing contract expiry**: Stored as NULL, contract status unset
- **Position with <3 players**: Podium shows only available players with placeholder for missing slots
- **All players excluded by minutes filter**: Empty results message shown
- **Wage without denomination**: Stored with denomination=NULL
