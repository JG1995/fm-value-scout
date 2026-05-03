# Wide Midfielders

## In-Possession

### Wide Midfielder

tags: holds position, stays wide
A wide player who tends to support the attack from deeper than a typical attacking wide player, the Wide Midfielder prefers to play dangerous passes and crosses from deep instead of trying to beat an opposition full-back, trusting their technical ability more than dribbling or pace.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Progressive Passes per 90 | 0.30   | false     |
| Key Passes per 90         | 0.25   | false     |
| Pass Completion Ratio     | 0.20   | false     |
| Crosses Completed per 90  | 0.15   | false     |
| Distance Covered per 90   | 0.10   | false     |

### Winger

tags: stays wide, creative threat
Providing width throughout the attack, the Winger stretches the opposition defence by attacking from close to the sidelines. The role aims to provide support to their team's forwards, playing dangerous passes and crosses in the final third to create chances for their teammates.

| METRIC_NAME              | WEIGHT | INVERTED? |
| ------------------------ | ------ | --------- |
| Crosses Completed per 90 | 0.30   | false     |
| xA per 90                | 0.25   | false     |
| Key Passes per 90        | 0.20   | false     |
| Dribbles Made per 90     | 0.15   | false     |
| Distance Covered per 90  | 0.10   | false     |

### Playmaking Winger

tags: moves inside, expressive
The Playmaking Winger will act as the team's primary source of creativity, starting from a wide position during their team's build-up phase before later drifting inside to find space from which to play killer balls and create chances.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.30   | false     |
| Key Passes per 90         | 0.25   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| Dribbles Made per 90      | 0.15   | false     |
| Pass Completion Ratio     | 0.10   | false     |

### Inside Winger

tags: moves inside, creative threat
Frequently aiming to drift inside into the half-space as their team attacks, the Inside Winger looks to recieve the ball in more central areas of the pitch, while also creating space out wide for overlapping teammates. The role is usually combined with attacking full-back or wing-back that looks to hug the sideline so that both players have their own space to operate in.

| METRIC_NAME                        | WEIGHT | INVERTED? |
| ---------------------------------- | ------ | --------- |
| xA per 90                          | 0.30   | false     |
| Key Passes per 90                  | 0.25   | false     |
| Progressive Passes per 90          | 0.20   | false     |
| Dribbles Made per 90               | 0.15   | false     |
| Open Play Crosses Completed per 90 | 0.10   | false     |

## Out-of-Possession

### Wide Midfielder

tags: none
The Wide Midfielder provides a balance between defensive protection against wide attacking threats, and being in a position to spring a counter in case of a turnover.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Interceptions per 90    | 0.30   | false     |
| Possession Won per 90   | 0.25   | false     |
| Blocks per 90           | 0.20   | false     |
| Clearances per 90       | 0.15   | false     |
| Distance Covered per 90 | 0.10   | false     |

### Tracking Wide Midfielder

tags: tracks back, supports defence
As a Tracker, the Wide Midfielder is instructed to track back defensively as the opposition advance, in order to support the defenders behind them, at times even dropping into the backline as an additional defender.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Distance Covered per 90 | 0.30   | false     |
| Interceptions per 90    | 0.25   | false     |
| Possession Won per 90   | 0.20   | false     |
| Tackles per 90          | 0.15   | false     |
| Headers Won Ratio       | 0.10   | false     |

### Wide Outlet Wide Midfielder

tags: stays high, counter-attacks
As a Wide Outlet, the Wide Midfielder is asked to avoid tracking back to help out the Full-Back behind them as much, instead looking to hold their position more in order to be in a dangerous position for a counter-attack when the ball turns over.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.35   | false     |
| Distance Covered per 90       | 0.25   | false     |
| xG per 90                     | 0.20   | false     |
| Possession Won per 90         | 0.10   | false     |
| Interceptions per 90          | 0.10   | false     |
