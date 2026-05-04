# Player Profile

The Player Profile is a dedicated page for deep-diving a single player's data. It is reached by clicking a player row in the Scouting table or Squad view. The page is not URL-addressable — navigation into it always originates from a browsable context.

The profile is divided into three primary zones: a **bio panel** (upper left), a **role fit matrix** (upper right), and a **statistics area** (the main body). A **career timeline** runs down the left edge of the page.

---

## Bio Panel

The top-left panel contains all non-metric information about the player, drawn directly from the CSV data.

**Fields displayed:**

| Field                       | Notes                                                                                                 |
| --------------------------- | ----------------------------------------------------------------------------------------------------- |
| Photo                       | User-configured path to FM player images. A placeholder silhouette is shown if the image is not found |
| Name                        | Full name                                                                                             |
| Age                         | Current age; updates to reflect age at the selected season when viewing historical data               |
| Nationality                 | Mapped from the 3-letter code to an actual country name                                               |
| Second nationality          | Displayed alongside the primary nationality                                                           |
| Club                        | Current club                                                                                          |
| Height                      | Displayed with "cm" unit label                                                                        |
| Footedness                  | Both the qualitative string (e.g. "Very Strong") and a numeric representation                         |
| Positions                   | All listed positions as badges/tags                                                                   |
| Wage                        | Converted to the user's preferred currency and payment frequency (e.g. weekly, monthly, yearly)       |
| Transfer value              | Normalized and displayed with currency symbol                                                         |
| Contract expiry             | Both the expiry date and "expires in X months"                                                        |
| Current / Potential Ability | Only shown if CA/PA data is present in the CSV; omitted otherwise                                     |

When viewing a historical season via the timeline, the bio panel updates to reflect the player's state at that point in time — age, club, contract, and other time-sensitive fields update accordingly.

---

## Role Fit Matrix

The upper-right corner displays a visual pitch showing the player's suitability for each position in the user's formation.

**What is shown:**

- A formatted pitch with the user's current formation
- One slot per position in the formation
- Each slot displays the player's **Quality Score** for the archetype assigned to that role in the user's squad setup
- A hover tooltip on each slot reveals the specific archetype name (e.g. "Ball-Playing CB", "Goalscoring Winger")

**If the user has not set up their squad**, default archetypes are applied. These defaults are pre-defined per formation and position, providing a reasonable baseline without requiring the user to configure anything first.

**Multiple positions:** If a player is listed with more than one position in the CSV, the role fit matrix accounts for all of them. The exact interaction between multi-position eligibility and archetype scoring (e.g., should a player rated for multiple positions score differently per position) is not yet finalized.

**Coming later:** Visual color thresholds (green/yellow/red) to indicate quality tiers.

---

## Career Timeline

A vertical timeline runs down the left edge of the page, allowing the user to step back through a player's career history across imported seasons.

**Behavior:**

- Timeline runs **oldest at the bottom, newest at the top**
- Each season is represented by a dot on the timeline
- **Dashed connectors** are used between seasons where the player has no data in an imported season, making gaps visually distinct from continuity
- The timeline remains visible even for single-season players, showing one dot

**Interaction:**

- Clicking a timeline dot updates the **entire page** — bio panel, statistics, and role fit matrix — to reflect that season
- Arrow buttons allow sequential stepping through seasons without clicking dots

**Hover tooltip:** Hovering a timeline dot shows:

- Season label (e.g. "2024/25")
- Club the player was at in that season

---

## Statistics Area

The main body of the profile is dedicated to metrics — presented in a consistent visual language that matches the Scouting table.

### Display Modes

Each metric is available in four variants:

| Variant               | Description                                                  |
| --------------------- | ------------------------------------------------------------ |
| Total                 | Raw cumulative value for the season                          |
| Per-90                | Normalized to a per-90-minutes basis                         |
| Percentile (all)      | Percentile rank against the full player database             |
| Percentile (position) | Percentile rank against players in the same position(s) only |

The user can toggle between **Against Position** and **Against All Positions** comparison pools. This preference is persisted locally.

Metrics that are not meaningful on a per-90 basis (e.g. appearances, cards) show only the total variant. The per-90 option is not displayed for these metrics.

### Percentile Display

Percentile scores use the same color-coding as the scouting table — a visual scale indicating relative performance within the comparison pool.

### Charts and Diagrams

The statistics area also includes charts and diagrams summarizing the player's profile. The app ships with a **curated default set** of charts, chosen to provide a strong starting point.

Users can **customize which charts and metrics** appear on the page. Preferences are saved locally. On first visit to any profile, the default set is shown; subsequent visits restore the user's last configuration.

**Customization scope:** The user chooses whether a customization applies to all profiles globally, or only to profiles of players in the same position (e.g., keep striker chart preferences separate from midfielder chart preferences).

---

## Metric Categories

All 80+ metrics are organized into the eight standard categories:

1. Attacking & Finishing
2. Creativity & Chance Creation
3. Transition & Ball Progression
4. Defensive Actions
5. Aerial Presence
6. Goalkeeping & Shot Stopping
7. Discipline & Error Margins
8. Match Impact & Availability

Whether all eight categories are visible simultaneously or paginated/scrollable is determined during layout implementation.

---

## Out of Scope

The following are planned for future releases and are explicitly not included in the initial scope:

- **Direct URL addressing** — profiles are not bookmarkable or linkable; they are reached through browsing contexts only
- **Player comparison** — side-by-side comparison of 2–3 players is a future feature
- **Player search** — a dedicated search box to jump directly to a profile by name is a future feature
- **Visual role-fit thresholds** — color-coded quality tiers for the role fit matrix (green/yellow/red)
- **Multi-position scoring nuance** — finalizing how multi-position eligibility interacts with role scoring
- **FM image sourcing** — reading player photos from an FM installation path; feasibility is unconfirmed
