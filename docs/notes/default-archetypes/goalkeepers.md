# Goalkeeper

## In-Possession

### Traditional Goalkeeper

tags: none
The Goalkeeper can distribute the ball in different ways depending on the team's playing strategy.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.30   | false     |
| Pass Completion Ratio     | 0.25   | false     |
| Passes Attempted per 90   | 0.20   | false     |
| xA per 90                 | 0.15   | false     |
| Key Passes per 90         | 0.10   | false     |

### Ball-Playing Goalkeeper

tags: expressive, roaming
The Ball-Playing Goalkeeper looks to play an active role in their team's build-up play, often moving out of their goal to be in a position to recieve the ball.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.30   | false     |
| Pass Completion Ratio     | 0.25   | false     |
| Passes Attempted per 90   | 0.20   | false     |
| xA per 90                 | 0.15   | false     |
| Key Passes per 90         | 0.10   | false     |

### No-Nonsense Goalkeeper

tags: careful, holds position
The No-Nonsense Goalkeeper adopts a cautious approach, preferring to stay nearer to their goal and taking as few risks as possible with the ball at their feet when put under pressure.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Pass Completion Ratio     | 0.35   | false     |
| Passes Attempted per 90   | 0.25   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| Possession Lost per 90    | 0.10   | true      |
| Fouls Made per 90         | 0.10   | true      |

## Out-of-Possession

### Traditional Goalkeeper

tags: none
The Goalkeeper role focuses on protecting their goal and making saves from shots and crosses. They are the last line of defence and are relied upon to help the team keep clean sheets.

| METRIC_NAME           | WEIGHT | INVERTED? |
| --------------------- | ------ | --------- |
| xGP per 90            | 0.30   | false     |
| Save Ratio            | 0.25   | false     |
| Saves Held per 90     | 0.15   | false     |
| Clean Sheets Ratio    | 0.15   | false     |
| Goals Conceded per 90 | 0.15   | true      |

### Sweeper Keeper

tags: come off line
The Sweeper Keeper is a proactive goalkeeper, regularly looking to come off their line in an attempt to intercept opposition passes or to engage opposition strikers early.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| xGP per 90              | 0.25   | false     |
| Interceptions per 90    | 0.25   | false     |
| Distance Covered per 90 | 0.20   | false     |
| High Intensity Sprints  | 0.15   | false     |
| Blocks per 90           | 0.15   | false     |

### Line-Holding Keeper

tags: stays on line
The Line-Holding Keeper is a goalkeeper who primarily looks to stay within the confines of their own box, leaving the job of defending higher up the pitch to their teammates.

| METRIC_NAME              | WEIGHT | INVERTED? |
| ------------------------ | ------ | --------- |
| xGP per 90               | 0.35   | false     |
| Save Ratio               | 0.25   | false     |
| Saves Held per 90        | 0.15   | false     |
| Aerial Challenges per 90 | 0.15   | false     |
| Clearances per 90        | 0.10   | false     |
