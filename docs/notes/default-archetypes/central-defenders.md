# Central Defender

## In-Possession

### Centre-Back

tags: holds position
The main job of the Centre-Back is to provide protection to the team, being in a position where they can be ready to react to attacking threats when the ball is turned over.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Headers Won Ratio     | 0.30   | false     |
| Interceptions per 90  | 0.25   | false     |
| Possession Won per 90 | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Clearances per 90     | 0.10   | false     |

### Advanced Centre-Back

tags: moves to DM, expressive
The main job of the Centre-Back is to provide protection to the team, being in a position where they can be ready to react to attacking threats when the ball is turned over. It differs from Centre-Back in that the Advanced Centre-Back is generally positioning higher up the field, ready to be involved in build-up play.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.30   | false     |
| Passes Attempted per 90   | 0.25   | false     |
| Key Passes per 90         | 0.20   | false     |
| Distance Covered per 90   | 0.15   | false     |
| Dribbles Made per 90      | 0.10   | false     |

### Ball-Playing Centre-Back

tags: expressive, holds position
The two principle tasks of the Ball-Playing Centre-Back are to provide protection to the team whilst also playing line-breaking passes and through balls from deep to progress the ball up the pitch.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.40   | false     |
| Pass Completion Ratio     | 0.25   | false     |
| Passes Attempted per 90   | 0.20   | false     |
| xA per 90                 | 0.10   | false     |
| Key Passes per 90         | 0.05   | false     |

### No-Nonsense Centre-Back

tags: holds position, careful
The No-Nonsense Centre-Back is focused on being the last outfield protection a team has, being in a position to be ready to defend at any moment, while also looking to minimize the amount of risks they take with the ball at their feet.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Pass Completion Ratio     | 0.35   | false     |
| Headers Won Ratio         | 0.20   | false     |
| Possession Lost per 90    | 0.20   | true      |
| Fouls Made per 90         | 0.15   | true      |
| Progressive Passes per 90 | 0.10   | false     |

### Wide Centre-Back

tags: holds posiiton, stays wide
Alongside provioding defensive protection to the team, the Wide Centre-Back is encouraged to provide width in possession and support to their wide attacking teammates ahead of them, often offering a recycling passing option from deep.

| METRIC_NAME                        | WEIGHT | INVERTED? |
| ---------------------------------- | ------ | --------- |
| Pass Completion Ratio              | 0.30   | false     |
| Progressive Passes per 90          | 0.25   | false     |
| Open Play Crosses Completed per 90 | 0.20   | false     |
| Distance Covered per 90            | 0.15   | false     |
| Passes Attempted per 90            | 0.10   | false     |

### Overlapping Centre-Back

tags: gets further forward, stays wide
The Overlapping Centre-Back combines the responsibilities of a Centre-Back and a Winger, looking to provide protection to the team during early phases of attack, while also getting into crossing positions by making overlapping runs in the final third of the pitch.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Distance Covered per 90   | 0.25   | false     |
| Progressive Passes per 90 | 0.25   | false     |
| Crosses Completed per 90  | 0.20   | false     |
| Dribbles Made per 90      | 0.15   | false     |
| Key Passes per 90         | 0.15   | false     |

## Out-of-Possession

### Centre-Back

tags: none
The main job of the Centre-Back is to stop the opposing attackers from playing and to clear the ball from danger when required.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Headers Won Ratio     | 0.30   | false     |
| Interceptions per 90  | 0.25   | false     |
| Possession Won per 90 | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Clearances per 90     | 0.10   | false     |

### Stopping Centre-Back

tags: steps out, aggressive
As a Stopper, the Centre-Back is asked to more willing to step out from their position in order to hassle and engage opposition attackers, in an attempt to cut out attacks at their source.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| Pressures Completed per 90    | 0.35   | false     |
| Tackles Completed per 90      | 0.25   | false     |
| Possession Won per 90         | 0.20   | false     |
| Interceptions per 90          | 0.15   | false     |
| High Intensity Sprints per 90 | 0.05   | false     |

### Covering Centre-Back

tags: holds position, careful
As a Cover, the Centre-Back is asked to adopt a more cautious approach, holding their line and waiting to react to the opposition attacker's movements.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Interceptions per 90    | 0.35   | false     |
| Tackle Completion Ratio | 0.25   | false     |
| Headers Won Ratio       | 0.20   | false     |
| Possession Won per 90   | 0.15   | false     |
| Distance Covered per 90 | 0.05   | false     |

### Wide Centre-Back

tags: none
The main job of the Wide Centre-Back is to stop the opposing attackers from playing and to clear the ball from danger when required.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Headers Won Ratio     | 0.25   | false     |
| Interceptions per 90  | 0.20   | false     |
| Blocks per 90         | 0.20   | false     |
| Clearances per 90     | 0.20   | false     |
| Possession Won per 90 | 0.15   | false     |

### Wide Stopping Centre-Back

tags: steps out, aggressive
As a Stopper, the Wide Centre-Back is asked to more willing to step out from their position in order to hassle and engage opposition attackers, in an attempt to cut out attacks at their source.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.30   | false     |
| Pressures Completed per 90    | 0.30   | false     |
| Tackles Completed per 90      | 0.20   | false     |
| Headers Won Ratio             | 0.10   | false     |
| Possession Won per 90         | 0.10   | false     |

### Wide Covering Centre-Back

tags: holds position, careful
As a Cover, the Wide Centre-Back is asked to adopt a more cautious approach, holding their line and waiting to react to the opposition attacker's movements.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| Interceptions per 90  | 0.30   | false     |
| Headers Won Ratio     | 0.25   | false     |
| Possession Won per 90 | 0.20   | false     |
| Blocks per 90         | 0.15   | false     |
| Clearances per 90     | 0.10   | false     |
