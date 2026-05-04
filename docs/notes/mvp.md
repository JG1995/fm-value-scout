# FM ValueScout — MVP Specification

## Purpose

This document defines the Minimum Viable Product for FM ValueScout. It serves as the single source of truth for what is being built in the first release, intended to keep development aligned and resist scope creep.

## Overview

FM ValueScout is a companion application for Football Manager players who apply moneyball-style scouting — identifying players who are statistically excellent but undervalued relative to their transfer cost.

The MVP delivers the core scouting workflow: import a player database, select a role and archetype, and surface a ranked shortlist of candidates with quality and value-adjusted scores.

---

## In Scope

### Data Import

- Single CSV import per session
- User provides the in-game date at time of import
- Semicolon-delimited CSV with 80+ columns parsed and persisted to local SQLite database
- Wage and value normalization with magnitude suffixes (K, M) handled
- Transfer value ranges stored as upper bound
- Optional CA/PA columns handled gracefully when absent
- All parsed players available for search immediately after import

### Scouting View — Two Entry Points

**Database View**
A table of all players in the loaded database, with color-coded percentile scores per metric. Every player is shown. Rows are clickable, but do not navigate in MVP (click behavior deferred to Player Profile).

**Role Search**
The primary MVP workflow. User selects a position from a dropdown, then selects an archetype. Results display automatically upon selection.

### Role Search Results

**Top 3 Podium**
A hero section above the results table presenting the three highest-scoring players for the selected archetype, displayed as a ranked podium (1st, 2nd, 3rd). Ordering is driven by the active score mode.

**Score Mode Toggle**
The user toggles between two ranking drivers:

- Quality Score — raw weighted metric score, normalized 0–100
- Value Score — quality score adjusted by transfer value percentile (cheapest players receive up to ×1.5 multiplier; most expensive receive ×0.5; linear interpolation between)

**Comparison Pool Toggle**
The user toggles between two percentile comparison pools:

- Against Position — percentile computed against players in the same position(s) only
- Against All Positions — percentile computed against the full database

Both toggles are persistent within the session.

### Results Table

- Unranked — ranking is handled by the podium
- Fixed columns: Name, Age, Club, Contract Expiry, Quality Score, Value Score
- Archetype metrics: the top N metrics by archetype weight (approximately 10–12 columns)
- Sortable by any visible column
- Color-coded percentile display consistent with database view

### Filters

There are no active filters in the MVP. The scouting CSV export from Football Manager does not include the user's own club, so own-club exclusion is handled implicitly by the data source. The 1,000-minute minimum-played threshold is deferred to a post-MVP release.

### User Preferences (Persisted Locally)

- Score mode preference (Quality vs Value)
- Comparison pool preference (same-position vs all-positions)

---

## Out of Scope (Not in MVP)

The following are planned for post-MVP releases:

- **Player Profile page** — click-through from scouting results deferred
- **Squad module** — tactics board, lineup optimizer, squad overview table
- **Club / save setup** — club selection, save management
- **Custom archetypes** — archetype creation and editing
- **League filter** — filtering by OPTA league strength or similar
- **Watchlist / saved players** — bookmarking candidates
- **Multiple season selection** — querying against historical seasons
- **Formation sync** — linking scouting pitch to squad formation
- **Player comparison** — side-by-side candidate comparison
- **Player search** — direct name-based lookup
- **Scouting CSV vs Squad CSV distinction** — single import type in MVP
- **Archetype metric preview** — showing which metrics an archetype weights
- **1,000-minute threshold** — minimum minutes filter deferred

---

## Technical Constraints

- Fully offline — no external server communication
- Local SQLite persistence
- Tauri v2 + Svelte 5 frontend
- Single-user, single-machine

---

## Success Criteria

The MVP is complete when:

1. A user can import a valid FM CSV and query any position/archetype combination
2. Role search returns a Top 3 podium ordered by the selected score mode
3. Comparison pool toggle changes percentile benchmarks correctly
4. Value score multiplier is applied to transfer value percentiles as specified
5. Results table is sortable and reflects the selected archetype's metrics
6. User preferences (score mode, comparison pool) persist across page reloads
