# Squad

The Squad module ties the user's own team into the broader scouting ecosystem. It provides a visual tactics board for setting formation and archetypes per position, a lineup optimizer, and a table view of the squad with color-coded metric performance.

Before accessing the squad, the user creates a "save" — analogous to a Football Manager save file. Each save corresponds to one club and its associated data. The user can have multiple saves active simultaneously.

---

## Save and Club Setup

When creating a new save, the user:

1. Names the save
2. Selects their club from a searchable list populated from the loaded database
3. Optionally uploads a squad-specific CSV for that season (see below)

The save concept is persistent — switching between saves changes which club, squad, and tactics board is active throughout the app. This mirrors how FM itself handles multiple concurrent saves.

---

## Data: Squad CSV vs Scouting CSV

The app distinguishes between two import types:

**Scouting CSV** — a full database export containing all players in the game world. This feeds the scouting module.

**Squad CSV** — an export containing only the user's own squad. The columns are identical to a scouting CSV; the difference is scope.

Both import types use the same underlying database. When the same player appears in both (e.g., a player from the user's squad also appearing in a full database export), the newest entry by in-game date wins. The user inputs the in-game date when uploading any CSV.

A squad CSV can be uploaded at any time — it merges with and updates the existing squad data, replacing only the fields for players that appear in the new upload. This allows the squad to stay current as the season progresses.

Uploading a new season's squad CSV adds it as a new season layer in the same way a scouting CSV does.

---

## Tactics Board

The central feature of the squad module. A visual representation of a football pitch where the user configures their formation and assigns archetypes to each position slot.

### Formation

The user builds their formation by positioning role slots on the pitch. Movement is free-form but constrained to standard position zones (GK zone, CB zone, etc.), similar to how FM itself works. Attempting to place a slot outside its valid zone shows a warning but does not block the action — the user can override it if desired.

The user can name their formation (e.g., "4-3-3 Gegenpress", "Low Block 4-4-2").

One formation is active at a time. The user can toggle between an in-possession and an out-of-possession formation, allowing different structures for different phases of play.

A reset button clears the current formation entirely.

### Role Slots

Each slot on the tactics board represents one position. Clicking a slot opens a panel where the user:

- Assigns a player from the database to the slot
- Selects an archetype for the slot

**Player assignment** is flexible and organizational. Any player from the full database can be assigned to any slot, including players from rival clubs. The squad table uses clear color-coding to distinguish current squad players from proposed signings.

**Archetype selection** sets which metrics the slot is evaluated against. Each slot requires an archetype assignment.

### Archetypes Per Slot

Each slot has one archetype assignment that covers either:

- A single phase (in-possession only, or out-of-possession only)
- Both phases simultaneously (a combined archetype that defines both)

Alternatively, the user can assign separate archetypes for each phase per slot, giving fine-grained control.

The active archetype for a slot determines which metrics are highlighted in the squad table view when that slot is selected.

---

## Lineup Optimizer

The tactics board includes a lineup suggestion feature. Pressing the generate button runs a constrained optimization to find the best available XI for the current formation and archetype setup.

**How it works:**

- Each slot is filled with the eligible player from the database who scores highest for that slot's archetype
- Players are assigned to only one slot — no duplication
- Only players eligible for a position (based on CSV data) are considered for that slot
- The optimizer respects locked slots — a locked slot is not modified

**Locked slots:** The user can lock individual slots, preventing the optimizer from replacing the player in that position. This is useful for players the user wants to keep regardless of metrics (e.g., a captain).

**Output:** The result is presented in a modal containing the populated tactics board, showing the optimizer's suggested XI. The user can accept changes, dismiss, or adjust and regenerate.

**CA weighting:** The optimizer uses only metric-based archetype scores. In-game ability (CA) is not factored into lineup suggestions at this stage.

---

## Squad Overview Table

A tabular view of the squad, designed for familiarity with FM's own squad view.

**Sort order:** Grouped by position by default, consistent with FM's layout. Columns are user-sortable.

**Age tabs:** The user can filter by Senior Squad, U21, U18, or all players. Age is computed from the player's birthdate against the season's in-game date.

**Visible metrics:** The columns shown depend on which slot is actively selected on the tactics board. Selecting a slot highlights the metrics relevant to that slot's archetype. Unselected slots do not influence visible columns.

**Player rows:** Each row shows the player's name, age, and color-coded percentile scores for the relevant metrics. Empty slots appear as empty rows.

**Multi-position players:** How to represent a player eligible for multiple positions — one row, multiple rows, or color-coded by currently assigned slot — is not yet finalized.

---

## Archetype Management

Custom archetypes are created and managed from a dedicated Archetype Management page, accessible from the sidebar and relevant views.

### Creating a Custom Archetype

The user can create a custom archetype:

- **From scratch** — choose metrics and set weights from scratch
- **Derived from existing** — copy an existing archetype (default or custom) as a starting point

**Weights** are expressed as percentages that must sum to 100%. The user inputs via sliders and/or numeric fields.

### Editing and Propagation

When a custom archetype is edited, the changes propagate immediately to all squad slots using that archetype. The default archetypes are immutable.

Custom archetypes are independent copies — editing a derived archetype has no effect on its parent, and vice versa.

### Deletion

Archetypes in use by squad slots cannot be deleted. The user must first reassign those slots to a different archetype before deletion is allowed.

---

## Out-of-Scope (MVP)

The following features are identified but deferred:

- Formation import from FM
- Multiple concurrent formations (in-possession and out-of-possession are toggled, not shown simultaneously)
- CA-weighted lineup suggestions
- Fine-grained phase control on archetypes (setting one phase in a combined archetype without creating separate entries)
- Visual role-fit color thresholds in the tactics board
- Drag-to-reorder rows in the squad table
- Multi-position player representation in the squad table

---

## Unfinalized

The following are noted areas where direction is still open:

- **Multi-position players in squad table** — visual representation TBD
- **Locked slot visual indicator** — whether locked slots are highlighted on the board
- **Archetype phase override** — UX for overriding one phase of a combined archetype without branching
- **Suggested lineup — partial fills** — handling cases where the optimizer cannot fill all slots (insufficient eligible players)
- **Squad CSV vs scouting CSV merge UX** — whether the user is notified when squad data is overwritten by a newer scouting import
