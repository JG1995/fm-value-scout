# Feature: Scouting View

## 1. Executive Summary

**High-Level Goal:** The Scouting View is the core moneyball workflow — it replaces the manual spreadsheet process FM players use to find statistically excellent, undervalued talent. It surfaces a ranked shortlist of transfer candidates for any position and archetype, scored by both raw quality and a value-adjusted metric that rewards players who punch above their cost.

**Context:** This is the Moneyball Scouting feature described in the [Concept Document](../CONCEPT.MD). It consumes parsed player data from the CSV Parser, reads archetype definitions (default and custom), and computes quality and value scores. It is the primary reason the application exists.

## 2. Information Architecture (The "Memory Bank")

### User Inputs

- **Mode selection:** The user chooses between Database View ("browse everything") and Role Search ("find a player for this role") via a tab or segmented control.
- **Position selector (Role Search):** A dropdown listing all standard Football Manager positions. Selecting a position populates the available archetypes.
- **Archetype selector (Role Search):** A dropdown listing archetypes for the selected position. Selecting an archetype triggers the search automatically. Both in-possession and out-of-possession variants are listed for each role.
- **Score mode toggle:** A toggle or segmented control switching between "Quality" (raw weighted metric score) and "Value" (quality adjusted by transfer value percentile).
- **Comparison pool toggle:** A toggle switching between "Against Position" (percentiles computed against same-position players only) and "Against All Positions" (percentiles against the full database).

### Displayed Data

- **Database View:** A dense, sortable table of every player in the loaded database with color-coded percentile scores for key metrics. Each row is clickable, navigating to that player's full profile.
- **Top 3 Podium (Role Search):** A hero section above the results table displaying the three highest-scoring players for the selected archetype as a ranked podium (1st, 2nd, 3rd). Ordering is driven by the active score mode.
- **Results Table (Role Search):** Below the podium, a sortable table of all qualifying players. Fixed columns: Name, Age, Club, Contract Expiry, Quality Score, Value Score. Additional columns show the top-weighted metrics for the selected archetype (approximately 10–12 columns). Rows are clickable and navigate to the player's full profile.
- **Empty state:** If no archetype is selected, a prompt reads: "Select a position and archetype to find players." If a search yields zero results, a message reads: "No players match this archetype. Try adjusting your filters."

### Persistent Data

Local SQLite persistence for user preferences across sessions:

- **Score mode preference:** Whether the user last used Quality or Value score. Restored on next visit.
- **Comparison pool preference:** Whether the user last used Against Position or Against All Positions. Restored on next visit.
- **The system must NOT persist** the selected position/archetype — these reset to unselected on each visit to the Scouting tab, as the user's intent changes between sessions.

## 3. The User Journey (Step-by-Step)

### Journey A: Database View (Browse Mode)

1. **Entry Point:** The user opens the Scouting tab. The Database View is the default active mode.
2. **System Response 1:** A table loads displaying every player in the database, with color-coded percentile scores across visible metrics. The table is fully populated — no position or archetype selection is needed.
3. **Action 1:** The user toggles the comparison pool between "Against Position" and "Against All Positions."
4. **System Response 2:** Percentile scores across the entire table recalculate and recolour immediately. The table does not reload.
5. **Action 2:** The user clicks a column header to sort the table.
6. **System Response 3:** Rows reorder. A sort indicator (arrow) appears on the active column header.
7. **Action 3:** The user clicks a player row.
8. **Success State:** The app navigates to that player's full Player Profile page.

### Journey B: Role Search (Targeted Recruitment)

1. **Entry Point:** The user switches to the Role Search mode from the segmented control.
2. **System Response 1:** The position and archetype dropdowns appear. The results area shows the empty-state prompt: "Select a position and archetype to find players."
3. **Action 1:** The user selects a position (e.g., "Center Back") from the position dropdown.
4. **System Response 2:** The archetype dropdown populates with the archetypes available for that position (e.g., "Traditional Center Back, Ball-Playing Center Back" — each with in-possession and out-of-possession variants).
5. **Action 2:** The user selects an archetype (e.g., "Ball-Playing Center Back — In Possession").
6. **System Response 3:** The app computes quality and value scores for every player against the selected archetype's weighted metrics, applies the comparison pool (default: Against Position), and displays results: a Top 3 podium and a sortable results table below.
7. **Action 3:** The user toggles the score mode from "Quality" to "Value."
8. **System Response 4:** The podium reorders to show the three highest value scores. The results table updates the Value Score column ordering.
9. **Action 4:** The user clicks the 1st-place player on the podium.
10. **Success State:** The app navigates to that player's full Player Profile page.

## 4. Logical Constraints (The "Rules of the Road")

### Scoring

- **IF** a player's archetype score is computed, **THEN** each metric in the archetype contributes its weighted share: `score = sum(weight_i × percentile_i)` for all weighted metrics, normalized to a 0–100 scale.
- **IF** a metric is inverted (lower is better, e.g., Possession Lost, Fouls Made), **THEN** its percentile is computed on the inverted distribution so that the lowest raw value receives the highest percentile.
- **IF** the archetype has both in-possession and out-of-possession variants, **THEN** the combined score is `(0.75 × in_possession_score) + (0.25 × out_of_possession_score)` by default. This split is user-adjustable.
- **IF** a player has 0 minutes played in the most recent season, **THEN** per-90 metrics are 0, and the player's score reflects only total metrics (which will also be 0 for counting stats).

### Value Score

- **IF** the user has selected Value Score mode, **THEN** each player's value score = Quality Score × value_multiplier, where the multiplier is determined by the player's transfer value percentile:
  - Transfer value in the cheapest 10%: multiplier = 1.5
  - Transfer value in the most expensive 10%: multiplier = 0.5
  - Transfer value between 10th and 90th percentile: multiplier = linear interpolation between 1.5 and 0.5
- **IF** a player has no listed transfer value (e.g., free agent), **THEN** they fall into the cheapest percentile and receive the ×1.5 multiplier.
- **IF** the value multiplier is applied, **THEN** it is position-agnostic — the same transfer value percentile receives the same multiplier regardless of position.

### Comparison Pool

- **IF** the comparison pool is "Against Position," **THEN** percentiles for each metric are computed using only players who share at least one position with the target player.
- **IF** the comparison pool is "Against All Positions," **THEN** percentiles are computed against the entire player database.
- **IF** the user switches the comparison pool, **THEN** all scores and percentiles recalculate immediately.

### Results Filtering

- **IF** a player has fewer than 1,000 minutes in the most recent season, **THEN** they are excluded from Role Search results.
- **IF** a player belongs to the user's managed club, **THEN** they are excluded from Role Search results.
- **IF** the database contains no players matching the selected archetype after filtering, **THEN** the empty-state message is shown.

### Results Table

- **IF** the user clicks a column header, **THEN** the table sorts by that column. Clicking the same header again reverses the sort direction.
- **IF** sorting by a numeric column, **THEN** values are compared numerically, not lexicographically.
- **IF** the selected archetype has N weighted metrics, **THEN** the table displays the top-weighted metrics up to the maximum that fits the layout (approximately 10–12).
- **IF** a table cell displays a percentile value, **THEN** the cell background is color-coded on a gradient: red (low percentile) through yellow (median) to green (high percentile).

### Top 3 Podium

- **IF** the active score mode is Quality, **THEN** the podium ranks by Quality Score descending.
- **IF** the active score mode is Value, **THEN** the podium ranks by Value Score descending.
- **IF** two players have identical scores, **THEN** the tie is broken by the secondary score (Value breaks ties by Quality, Quality breaks ties by Value).

### Navigation

- **IF** the user clicks a player row in the results table or a player card on the podium, **THEN** the app navigates to that player's full profile.
- **IF** the user navigates back from the Player Profile, **THEN** the Scouting View is restored in its previous state (same mode, same position/archetype selection, same score mode, same comparison pool, same sort state). State is held in a Svelte 5 `$state` store so it survives route navigation without re-querying.

## 5. Negative Paths & Edge Cases (The "What-Ifs")

### Error Handling

- **Database empty:** If no CSV has been imported, the Scouting tab shows a prompt: "No player data loaded. Import a CSV to begin scouting." with a button linking to the Data Import page.
- **Score computation failure:** If a player's score cannot be computed (e.g., all archetype metrics are NULL for that player), the player is excluded from results rather than shown with a score of 0.
- **Archetype with zero-weight metrics:** If all metrics in an archetype have a weight of 0, the score is 0 for all players. The podium still displays (showing three players with 0 scores) rather than erroring.

### Empty States

- **No archetype selected:** "Select a position and archetype to find players."
- **No results:** "No players match this archetype. Try adjusting your filters." Shown when the combination of position, archetype, and filters yields zero qualifying players.
- **Database View with no data:** The table renders with zero rows and a message: "No players in database. Import a CSV to get started."

### Boundary Cases

- **Position with few players:** If the selected position has fewer than 3 players in the database, the podium shows only the available players (e.g., 1 or 2 instead of 3). Empty podium slots display a placeholder card reading "Not enough players at this position."
- **All players excluded by minutes filter:** If every player at the selected position has fewer than 1,000 minutes, the empty-state message is shown.
- **Single-metric archetype:** If a custom archetype weights only one metric, scoring and display still work — the results table shows only that one archetype metric column.
- **Extremely large database:** With 200,000+ players, scoring and sorting must remain responsive. Percentile computation is done once per comparison-pool switch and cached. Table rendering uses virtual scrolling to avoid DOM overload.
- **Toggling score mode rapidly:** Debounced at 300ms — the user can flip the toggle back and forth without triggering redundant score recomputations. Only the final state after settling triggers the recompute.

### Interruptions

- **Navigation away mid-search:** If the user navigates away (e.g., to the Squad tab) while a Role Search is active, the search is not cancelled — results are held in the `$state` store and restored when the user returns.
- **CSV imported while scouting:** If a new CSV is imported while the Scouting tab is open (unlikely since import is a separate flow, but possible via multi-window), the scores are not automatically refreshed. The user must reselect the archetype or toggle the score mode to trigger recomputation against the updated database.

### Score Edge Cases

- **Transfer value of 0:** A player with a transfer value of exactly €0 (distinct from NULL/no value listed) is treated as cheapest percentile (×1.5 multiplier).
- **All players have identical transfer values:** The percentile bucketing still works — all players fall into the same transfer value percentile, and all receive the same multiplier. Value scores are proportional to Quality scores.
- **Player appears in multiple positions:** A player listed as `D (LC), WB/M (L)` counts for both Center Back and Wing Back positions. In "Against Position" mode, their percentiles include players from both position groups.

## 6. Interface & Interaction (The "Look and Feel")

### Visual Style

Consistent with the glassmorphism design language. The scouting interface uses translucent card panels with backdrop blur over the app's dark gradient background. Percentile scores use a red-to-yellow-to-green heatmap gradient on table cells.

### Layout

- **Mode selector:** A segmented control at the top of the view with two segments: "Database View" and "Role Search." The active segment uses the app's accent colour; the inactive segment is translucent.
- **Database View:** A full-width table occupying the main content area. A comparison pool toggle sits in the toolbar above the table.
- **Role Search:** Below the mode selector: a toolbar row with the position dropdown (left), archetype dropdown (center), score mode toggle (right), and comparison pool toggle (far right). Below the toolbar: the Top 3 Podium (three cards in a row, 1st place center and elevated slightly, 2nd left, 3rd right). Below the podium: a full-width sortable results table.
- **Podium cards:** Each card shows the player name (large), age, club, nation flag, quality score, value score, and transfer value. The 1st place card is visually distinct — larger, with a subtle gold accent border.
- **Table:** Compact rows with alternating subtle transparency. Sortable column headers have a small arrow indicator. Percentile cells use the heatmap gradient. Rows highlight on hover.

### Animations

- **Podium reorder:** When the score mode toggles, podium cards animate smoothly into their new positions (crossfade + translate, ~400ms).
- **Table sort:** Rows reorder with a subtle staggered transition.
- **Percentile recalculation:** Cell colours transition smoothly to their new values when the comparison pool changes (~300ms color tween).

### Copywriting

- **Mode selector — Database View:** "Database"
- **Mode selector — Role Search:** "Role Search"
- **Position dropdown label:** "Position"
- **Archetype dropdown label:** "Archetype"
- **Score mode toggle — Quality:** "Quality"
- **Score mode toggle — Value:** "Value"
- **Comparison pool toggle — Position:** "vs Position"
- **Comparison pool toggle — All:** "vs All"
- **Empty — no archetype:** "Select a position and archetype to find players."
- **Empty — no results:** "No players match this archetype. Try adjusting your filters."
- **Empty — no database:** "No player data loaded. Import a CSV to begin scouting."
- **Podium — rank labels:** "1st", "2nd", "3rd"
- **Table — contract column:** "Contract"
- **Table — quality score column:** "Quality"
- **Table — value score column:** "Value"

## 7. Acceptance Criteria (The "Mission Accomplished" Checklist)

- [ ] Database View displays every player in the loaded database with color-coded percentile scores.
- [ ] Database View supports sorting by any visible column header (click to sort asc, click again to sort desc).
- [ ] Role Search populates the archetype dropdown with only archetypes relevant to the selected position.
- [ ] Selecting an archetype triggers automatic score computation and displays the Top 3 Podium and results table.
- [ ] Quality Score mode ranks the podium and sorts results by raw weighted archetype score (0–100).
- [ ] Value Score mode applies the transfer value percentile multiplier (×1.5 cheapest, ×0.5 most expensive, linear interpolation between) and reorders the podium accordingly.
- [ ] Players with no transfer value are treated as cheapest percentile (×1.5 multiplier).
- [ ] Comparison pool toggle recalculates percentiles against the selected pool (same-position or all-positions) and recolours the table.
- [ ] The 1,000-minute minimum threshold excludes players below that threshold from Role Search results.
- [ ] Players from the user's managed club are excluded from Role Search results.
- [ ] Both score mode and comparison pool preferences persist across sessions and are restored on next visit.
- [ ] Clicking a player row in the results table or a player card on the podium navigates to that player's Player Profile.
- [ ] Returning from the Player Profile restores the Scouting View in its previous state (mode, selection, scores, sort order).
- [ ] Empty database, no archetype selected, and zero-results states each display the correct empty-state message.
- [ ] Scores compute correctly for archetypes with inverted metrics (lower raw value = higher percentile).
- [ ] Scores compute correctly for combined in-possession/out-of-possession archetypes using the 75/25 default split.
- [ ] Ties on the podium are broken by the secondary score.
- [ ] Rapid score mode toggling is debounced and does not cause redundant recomputations.
- [ ] The UI matches the glassmorphism design language: translucent panels, backdrop blur, dark gradient background, accent-coloured primary controls.
- [ ] Percentile cells use a red-to-yellow-to-green heatmap gradient.
- [ ] Podium reorders with smooth animation when the score mode changes.
