# Strikers

## In-Possession

### Deep-Lying Forward

tags: drops off, links play
With the main function of the Deep-Lying Forward being to link the attack to the midfield, they seek to drop between the lines to recieve passes from deep before supplying linking passes of their own to teammates in a number of ways: moving the ball back to midfield support, spreading it wide to the flanks, or, if they have time to turn, playing in their strike partner.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.30   | false     |
| Key Passes per 90         | 0.25   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| Pass Completion Ratio     | 0.15   | false     |
| NPxG per 90               | 0.10   | false     |

### Centre Forward

tags: goal threat
The Centre Forward, also typically known as the classic "Number 9", is expected to lead the line and look to spearhead attacking moves.

| METRIC_NAME            | WEIGHT | INVERTED? |
| ---------------------- | ------ | --------- |
| xG per 90              | 0.35   | false     |
| NPxG per 90            | 0.25   | false     |
| Shots per 90           | 0.20   | false     |
| Shots on Target per 90 | 0.10   | false     |
| Headers Won per 90     | 0.10   | false     |

### Target Forward

tags: plays on last line, links play
Heavily utilizing their physicality and height to disrupt the opposition's defence and create goalscoring opportunities for their team, the Target Forward is typically the team's attacking outlet during the build-up phase, and are often the target for longer passes and clearances that they knock down, flick on or lay off to their surrounding teammates.

| METRIC_NAME              | WEIGHT | INVERTED? |
| ------------------------ | ------ | --------- |
| Headers Won Ratio        | 0.35   | false     |
| xG per 90                | 0.25   | false     |
| Key Passes per 90        | 0.20   | false     |
| Aerial Challenges per 90 | 0.10   | false     |
| NPxG per 90              | 0.10   | false     |

### Poacher

tags: plays on last line, goal threat
The Poacher sits on the shoulder of the last defender, looking to break the defensive line and run onto through balls from the midfield.

| METRIC_NAME              | WEIGHT | INVERTED? |
| ------------------------ | ------ | --------- |
| xG per 90                | 0.40   | false     |
| NPxG per 90              | 0.25   | false     |
| Shots per 90             | 0.15   | false     |
| Shots on Target Ratio    | 0.10   | false     |
| Average Minutes per Goal | 0.10   | true      |

### Channel Forward

tags: runs into channels, goal threat
A high-energy role, the Channel Forward looks to constantly be a thorn in opposition defender's sides, regularly making supporting runs into the channels for their team.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| xG per 90                     | 0.30   | false     |
| NPxG per 90                   | 0.25   | false     |
| Shots per 90                  | 0.20   | false     |
| High Intensity Sprints per 90 | 0.15   | false     |
| Dribbles Made per 90          | 0.10   | false     |

### False Nine

tags: moves to AMC, links play
An unconventional centre forward, the False Nine regularly drops between the lines of the opposition's defence and midfield, looking to act similarly to an Advanced Playmaker for their team.

| METRIC_NAME               | WEIGHT | INVERTED? |
| ------------------------- | ------ | --------- |
| xA per 90                 | 0.30   | false     |
| Key Passes per 90         | 0.25   | false     |
| Progressive Passes per 90 | 0.20   | false     |
| xG per 90                 | 0.15   | false     |
| Pass Completion Ratio     | 0.10   | false     |

## Out-of-Possession

### Centre Forward

tags: none
The Centre Forward sets the tone for their team's defensive strategy, often starting the press, or screening passes through the middle of the pitch.

| METRIC_NAME                | WEIGHT | INVERTED? |
| -------------------------- | ------ | --------- |
| Pressures Completed per 90 | 0.30   | false     |
| Possession Won per 90      | 0.25   | false     |
| Interceptions per 90       | 0.20   | false     |
| Tackles per 90             | 0.15   | false     |
| Distance Covered per 90    | 0.10   | false     |

### Tracking Centre Forward

tags: tracks back, supports defence
As a Tracker, the Centre Forward is instructed to track back defensively as the opposition advance in order to support the defenders behind them, dropping back into a deeper defensive position.

| METRIC_NAME             | WEIGHT | INVERTED? |
| ----------------------- | ------ | --------- |
| Distance Covered per 90 | 0.35   | false     |
| Interceptions per 90    | 0.25   | false     |
| Possession Won per 90   | 0.20   | false     |
| Tackles per 90          | 0.10   | false     |
| Headers Won Ratio       | 0.10   | false     |

### Central Outlet Centre Forward

tags: stays high, counter-attacks
As a Central Outlet, the Centre Forward is asked to avoid tracking back to help out defenders behind them as much, instead looking to hold their position more in order to be in a dangerous position for a counter-attack when the ball turns over.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.35   | false     |
| xG per 90                     | 0.30   | false     |
| NPxG per 90                   | 0.20   | false     |
| Distance Covered per 90       | 0.10   | false     |
| Interceptions per 90          | 0.05   | false     |

### Splitting Outlet Centre Forward

tags: stays high, counter-attacks
As a Splitting Outlet, the Centre Forward is asked to avoid tracking back to help out defenders behind them as much, instead looking to hold their position more in order to be in a dangerous position for a counter-attack when the ball turns over.

| METRIC_NAME                   | WEIGHT | INVERTED? |
| ----------------------------- | ------ | --------- |
| High Intensity Sprints per 90 | 0.30   | false     |
| Progressive Passes per 90     | 0.30   | false     |
| xA per 90                     | 0.20   | false     |
| xG per 90                     | 0.10   | false     |
| Interceptions per 90          | 0.10   | false     |
