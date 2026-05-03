# Attacking Midfielders

## In-Possession

### Attacking Midfielder

tags: finds space, goal threat
An offensive role which looks to operate between the opposition defensive and midfield line, the Attacking Midfielder looks to recieve passes in pockets of space as much as possible.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xG per 90                 | 0.30   | false     |
| NPxG per 90               | 0.25   | false     |
| Shots per 90              | 0.20   | false     |
| Key Passes per 90         | 0.15   | false     |
| Progressive Passes per 90 | 0.10   | false     |

### Advanced Playmaker

tags: finds space, expressive
The Advanced Playmaker is a creative rolw hich loosk to operate high up the pitch between the opposition's midfield and defence, looking to recieve passes in pockets of space as much as possible.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.30   | false     |
| Key Passes per 90         | 0.25   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| xG per 90                 | 0.15   | false     |
| Pass Completion Ratio     | 0.10   | false     |

### Free Role

tags: expressive, creative threat
A player with a Free Role is given full creative license, allowing them to roam from their position to create attacking opportunities for their team whilst also being encouraged to take more risks with the ball in an effort to both create and score goals.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.25   | false     |
| xG per 90                 | 0.25   | false     |
| Key Passes per 90         | 0.20   | false     |
| Progressive Passes per 90 | 0.15   | false     |
| Possession Lost per 90    | 0.15   | true      |

### Second Striker

tags: moves to ST, goal threat
The Second Striker operates as one of the team's main goalscoring threats.

| METRIC_NAME          | WEIGHT | INVERTED? |
| -------------------- | ------ | --------- |
| xG per 90            | 0.35   | false     |
| NPxG per 90          | 0.25   | false     |
| Shots per 90         | 0.20   | false     |
| xA per 90            | 0.10   | false     |
| Dribbles Made per 90 | 0.10   | false     |

### Channel Midfielder

tags: runs into channels, creative threat
The Channel Midfielder looks to make supporting runs into wide areas, often linking up their team's wide attackers.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| Key Passes per 90         | 0.30   | false     |
| Progressive Passes per 90 | 0.25   | false     |
| Crosses Completed per 90  | 0.20   | false     |
| Dribbles Made per 90      | 0.15   | false     |
| xA per 90                 | 0.10   | false     |

## Out-of-Possession

### Attacking Midfielder

tags: none
The Attacking Midfielder looks to defend central areas of the pitch, often being tasked with preventing access to the opposition's midfield pivots.

| METRIC_NAME                | WEIGHT | INVERTED? |
| -------------------------- | ------ | --------- |
| Pressures Completed per 90 | 0.30   | false     |
| Interceptions per 90       | 0.25   | false     |
| Possession Won per 90      | 0.20   | false     |
| Tackles per 90             | 0.15   | false     |
| Distance Covered per 90    | 0.10   | false     |

### Tracking Attacking Midfielder

tags: tracks back, supports defence
As a Tracker, the Attacking Midfielder is instructed to track back defensively as the opposition advance in order to support the defenders behind them, dropping back into a deeper defensive position.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Distance Covered per 90 | 0.35   | false     |
| Interceptions per 90    | 0.25   | false     |
| Possession Won per 90   | 0.20   | false     |
| Tackles per 90          | 0.10   | false     |
| Headers Won Ratio       | 0.10   | false     |

### Central Outlet Attacking Midfielder

tags: stays high, counter-attacks
As a Central Outlet, the Attacking Midfielder is asked to avoid tracking back to help out defenders behind them as much, instead looking to hold their position more in order to be in a dangerous position for a counter-attack when the ball turns over.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.30   | false     |
| xG per 90                     | 0.30   | false     |
| NPxG per 90                   | 0.20   | false     |
| Distance Covered per 90       | 0.10   | false     |
| Interceptions per 90          | 0.10   | false     |

### Splitting Outlet Attacking Midfielder

tags: stays high, counter-attacks
As a Splitting Outlet, the Attacking Midfielder is asked to avoid tracking back to help out defenders behind them as much, instead looking to hold their position more in order to be in a dangerous position for a counter-attack when the ball turns over.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.30   | false     |
| xA per 90                     | 0.25   | false     |
| Progressive Passes per 90     | 0.20   | false     |
| xG per 90                     | 0.15   | false     |
| Interceptions per 90          | 0.10   | false     |
