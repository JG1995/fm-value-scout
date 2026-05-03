# Defensive Midfielder

## In-Possession

### Defensive Midfielder

tags: holds position, careful
The Defensive Midfielder's main responsibility is to position themselves in front of the team's defence and help protect them from counter-attacking threats.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Pass Completion Ratio   | 0.35   | false     |
| Possession Won per 90   | 0.25   | false     |
| Passes Attempted per 90 | 0.20   | false     |
| Possession Lost per 90  | 0.10   | true      |
| Fouls Made per 90       | 0.10   | true      |

### Deep-Lying Playmaker

tags: holds position, expressive
Operating in the space between the defence and the midfield, the Deep-Lying Playmaker aims to initiate attacking moves via pinpoint passes to players positioned higher up the pitch.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.40   | false     |
| Key Passes per 90         | 0.25   | false     |
| xA per 90                 | 0.20   | false     |
| Pass Completion Ratio     | 0.10   | false     |
| Passes Attempted per 90   | 0.05   | false     |

### Box-to-Box Midfielder

tags: moves to AMC, goal threat
The non-stop dynamism of the Box-to-Box Midfielder enables them to contribute greatly to both attacking play during the build-up and in the final third.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xG per 90                 | 0.30   | false     |
| Progressive Passes per 90 | 0.25   | false     |
| Distance Covered per 90   | 0.20   | false     |
| Shots per 90              | 0.15   | false     |
| Pass Completion Ratio     | 0.10   | false     |

### Half-Back

tags: moves to DC, careful
A defensive midfielder who looks to drop into their team's defensive line as they progress the ball up the pitch, the Half-Back acts as an additional central defender.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Interceptions per 90      | 0.30   | false     |
| Pass Completion Ratio     | 0.25   | false     |
| Headers Won Ratio         | 0.20   | false     |
| Progressive Passes per 90 | 0.15   | false     |
| Possession Won per 90     | 0.10   | false     |

### Box-to-Box Playmaker

tags: moves to AMC, creative threat
The Box-to-Box Playmaker is the heartbeat of their team, looking to be a creative outlet from both deep and advanced areas of the pitch.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.30   | false     |
| xG per 90                 | 0.20   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| Key Passes per 90         | 0.15   | false     |
| Distance Covered per 90   | 0.15   | false     |

## Out-of-Possession

### Defensive Midfielder

tags: none
The Defensive Midfielder's main responsibility out of possession is to position themselves in front of the team's defence and help protect them from attacking threats.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Tackles per 90        | 0.30   | false     |
| Possession Won per 90 | 0.25   | false     |
| Interceptions per 90  | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Fouls Made per 90     | 0.10   | true      |

### Dropping Defensive Midfielder

tags: moves to DC
As a Dropper, the defensive midfielder will look to move back into the defensive line as their team are pinned back into their own defensive third, acting as an additional central defender.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Headers Won Ratio     | 0.30   | false     |
| Interceptions per 90  | 0.25   | false     |
| Possession Won per 90 | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Clearances per 90     | 0.10   | false     |

### Pressing Defensive Midfielder

tags: steps out, aggressive
As a Presser, the defensive midfielder is tasked with stepping forward from their position to support the team's press high up the pitch.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| Pressures Completed per 90    | 0.35   | false     |
| High Intensity Sprints per 90 | 0.25   | false     |
| Possession Won per 90         | 0.20   | false     |
| Tackles per 90                | 0.15   | false     |
| Interceptions per 90          | 0.05   | false     |

### Screening Defensive Midfielder

tags: holds position, careful
As a Screener, the defensive midfielder's main responsibility is to defend the space in front of their central defenders.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Tackles per 90        | 0.30   | false     |
| Interceptions per 90  | 0.25   | false     |
| Possession Won per 90 | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Pass Completion Ratio | 0.10   | false     |

### Wide Covering Defensive Midfielder

tags: covers flanks
As a Wide Cover, the defensive midfielder is asked to provide defensive support in wide areas, moving across from their typical position to help their team defend against wide threats.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Distance Covered per 90 | 0.30   | false     |
| Interceptions per 90    | 0.25   | false     |
| Possession Won per 90   | 0.20   | false     |
| Tackles per 90          | 0.15   | false     |
| Blocks per 90           | 0.10   | false     |
