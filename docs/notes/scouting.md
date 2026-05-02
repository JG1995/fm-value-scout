# Scouting

The Scouting module is the core Moneyball tool — the reason the app exists. It replaces the manual export-to-spreadsheet workflow that FM moneyball players have historically relied on, bringing data-driven signing recommendations directly into the game loop.

## Two Modes

The Scouting tab presents two entry points:

### Database View

The "browse everything" mode. Displays every player in the loaded database in a dense, sortable table resembling the FM player list. Each row shows color-coded percentile scores for key metrics. Clicking a row navigates to the player's full profile.

Percentile benchmarks are calculated from one of two comparison pools, selectable by the user:

- **Against Position** — percentile is computed only against players listed as playing the same position(s)
- **Against All Positions** — percentile is computed against the full player database

Both modes calculate the same metric values; only the comparison denominator changes. Users can toggle between them at any time.

### Role Search

The "find a player for this role" mode. The user selects a position and archetype, and the app surfaces a ranked shortlist of candidates. This is the primary workflow for targeted recruitment.

In the MVP, the user selects position and archetype from dropdown controls. Results appear automatically upon selection, updating when the user changes their selection.

---

## Value-Adjusted Scoring

Moneyball is not just about finding the best players — it is about finding the best players relative to what they cost. Every player is scored two ways:

### Quality Score

The raw weighted score derived from the selected archetype's metric weights. Normalized to a 0–100 scale.

### Value Score

A value-adjusted version of the quality score that rewards players who punch above their cost. Players are bucketed into value percentiles based on their transfer value:

| Transfer Value Percentile | Multiplier Applied to Quality |
| ------------------------- | ----------------------------- |
| 0–10 (cheapest)           | ×1.5                          |
| 90–100 (most expensive)   | ×0.5                          |
| Intervals between         | Linear interpolation          |

**Important:** This multiplier applies uniformly across all positions. A €0 left-back receives the same value boost as a €0 striker, relative to their quality score.

Players with no listed transfer value (e.g. free agents, players with expired contracts) fall into the lowest value percentile and receive the ×1.5 multiplier. This reflects the moneyball principle that zero-cost talent is inherently high-value — but because the multiplier is position-agnostic, it is most meaningful for positions where cheap/free talent is realistic.

The two scores are always displayed together. The user chooses which one drives the ranking.

---

## Results Presentation

### Top 3 Podium

A hero section above the results table presents the three best signings for the selected archetype, displayed as a podium (1st, 2nd, 3rd). The ordering is driven by whichever score the user has selected as active — Quality Score or Value Score.

Users can toggle between the two scores, and the podium reorders accordingly. This is the primary decision-support visual.

### Results Table

Below the podium, a browsable table shows all qualifying players for the selected archetype. The table is not ranked — the ranking work is done by the podium. The table supports:

- **Sorting** by any visible column
- **Fixed columns** always shown: Name, Age, Club, Current Contract Expiry
- **Score columns**: Quality Score and Value Score for the selected archetype
- **Archetype metrics**: the N metrics weighted most heavily by the selected archetype, where N is the maximum that fits the layout (approximately 10–12 metrics in typical layouts)
- **Click-through**: clicking any row navigates to that player's full profile

---

## Filters

Role search applies sensible defaults to surface realistic candidates:

| Filter                                      | Default          | User-Adjustable? |
| ------------------------------------------- | ---------------- | ---------------- |
| Minimum minutes played (most recent season) | 1,000            | No (MVP)         |
| Own club players                            | Excluded         | No               |
| Season                                      | Most recent      | No (MVP)         |
| Comparison pool                             | Against Position | Yes              |

The 1,000-minute threshold filters out players with negligible game time, whose statistics are not statistically meaningful. Players from the user's own managed club are excluded from results entirely, as they are not available for transfer.

---

## MVP Scope

The scouting MVP covers the following end-to-end flow:

1. CSV data is loaded into the database
2. User selects a position and archetype via dropdown
3. App computes quality and value scores against the selected archetype
4. Results display as a Top 3 podium and a browsable table
5. User can toggle between Quality and Value score ordering
6. User can toggle between same-position and all-positions percentile comparison
7. Clicking a row navigates to the player profile

**Out of scope for MVP:**

- League filter or OPTA league strength ranking
- Watchlist / saved players
- Player-to-player comparison
- Archetype metric preview on selection
- Multiple season selection
- Formation sync from the Squad tab
- Customizable visible columns in the results table
- Real-time updates as CSV data changes

These are all planned for future releases.

---

## Future Considerations

The following are identified enhancements not yet committed to a release:

- **Watchlist**: ability to bookmark players from scouting results for later comparison
- **Formation pitch in role search**: a visual pitch (initially synced from the Squad tab) replacing the MVP dropdown, where clicking a slot triggers the search for that slot's archetype
- **League filter**: using OPTA League Strength Rankings as a basis for "realistic signing range" around the user's club's league
- **Archetype preview**: showing which metrics an archetype weights before committing to a search
- **Player comparison**: side-by-side view of 2–3 players from scouting results
- **Multiple season selection**: ability to run role search against any previously loaded season
- **Position-specific value multipliers**: adjusting the value multiplier based on position scarcity (e.g., cheap goalkeepers vs cheap strikers)
