/** Maps to backend PlayerMetric enum (SCREAMING_SNAKE_CASE serde). */
export enum PlayerMetric {
	// Attacking
	Goals = 'GOALS',
	GoalsPer90 = 'GOALS_PER90',
	GoalsOutsideBox = 'GOALS_OUTSIDE_BOX',
	GoalsOutsideBoxPer90 = 'GOALS_OUTSIDE_BOX_PER90',
	Xg = 'XG',
	XgPer90 = 'XG_PER90',
	Npxg = 'NPXG',
	NpxgPer90 = 'NPXG_PER90',
	XgOverperformance = 'XG_OVERPERFORMANCE',
	XgOverperformancePer90 = 'XG_OVERPERFORMANCE_PER90',
	XgPerShot = 'XG_PER_SHOT',
	Shots = 'SHOTS',
	ShotsPer90 = 'SHOTS_PER90',
	ShotsOutsideBox = 'SHOTS_OUTSIDE_BOX',
	ShotsOutsideBoxPer90 = 'SHOTS_OUTSIDE_BOX_PER90',
	ShotsOnTarget = 'SHOTS_ON_TARGET',
	ShotsOnTargetPer90 = 'SHOTS_ON_TARGET_PER90',
	ShotsOnTargetRatio = 'SHOTS_ON_TARGET_RATIO',
	ConversionRatio = 'CONVERSION_RATIO',
	PensTaken = 'PENS_TAKEN',
	PensTakenPer90 = 'PENS_TAKEN_PER90',
	PensScored = 'PENS_SCORED',
	PensScoredPer90 = 'PENS_SCORED_PER90',
	PensScoredRatio = 'PENS_SCORED_RATIO',
	FreeKickShots = 'FREE_KICK_SHOTS',
	FreeKickShotsPer90 = 'FREE_KICK_SHOTS_PER90',
	MinutesPerGoal = 'MINUTES_PER_GOAL',
	MinutesPerGoalOrAssist = 'MINUTES_PER_GOAL_OR_ASSIST',
	MinutesPerAssist = 'MINUTES_PER_ASSIST',

	// Creativity
	Assists = 'ASSISTS',
	AssistsPer90 = 'ASSISTS_PER90',
	Xa = 'XA',
	XaPer90 = 'XA_PER90',
	ChancesCreated = 'CHANCES_CREATED',
	ChancesCreatedPer90 = 'CHANCES_CREATED_PER90',
	ClearCutChances = 'CLEAR_CUT_CHANCES',
	ClearCutChancesPer90 = 'CLEAR_CUT_CHANCES_PER90',
	KeyPasses = 'KEY_PASSES',
	KeyPassesPer90 = 'KEY_PASSES_PER90',
	OpKeyPasses = 'OP_KEY_PASSES',
	OpKeyPassesPer90 = 'OP_KEY_PASSES_PER90',
	CrossesAttempted = 'CROSSES_ATTEMPTED',
	CrossesAttemptedPer90 = 'CROSSES_ATTEMPTED_PER90',
	CrossesCompleted = 'CROSSES_COMPLETED',
	CrossesCompletedPer90 = 'CROSSES_COMPLETED_PER90',
	CrossCompletionRatio = 'CROSS_COMPLETION_RATIO',
	OpCrossesAttempted = 'OP_CROSSES_ATTEMPTED',
	OpCrossesAttemptedPer90 = 'OP_CROSSES_ATTEMPTED_PER90',
	OpCrossesCompleted = 'OP_CROSSES_COMPLETED',
	OpCrossesCompletedPer90 = 'OP_CROSSES_COMPLETED_PER90',
	OpCrossCompletionRatio = 'OP_CROSS_COMPLETION_RATIO',

	// Transition
	PassesAttempted = 'PASSES_ATTEMPTED',
	PassesAttemptedPer90 = 'PASSES_ATTEMPTED_PER90',
	PassesCompleted = 'PASSES_COMPLETED',
	PassesCompletedPer90 = 'PASSES_COMPLETED_PER90',
	PassCompletionRatio = 'PASS_COMPLETION_RATIO',
	ProgressivePasses = 'PROGRESSIVE_PASSES',
	ProgressivePassesPer90 = 'PROGRESSIVE_PASSES_PER90',
	Dribbles = 'DRIBBLES',
	DribblesPer90 = 'DRIBBLES_PER90',
	DistanceCovered = 'DISTANCE_COVERED',
	DistanceCoveredPer90 = 'DISTANCE_COVERED_PER90',
	Sprints = 'SPRINTS',
	SprintsPer90 = 'SPRINTS_PER90',
	PossLost = 'POSS_LOST',
	PossLostPer90 = 'POSS_LOST_PER90',

	// Defensive
	TacklesAttempted = 'TACKLES_ATTEMPTED',
	TacklesAttemptedPer90 = 'TACKLES_ATTEMPTED_PER90',
	TacklesCompleted = 'TACKLES_COMPLETED',
	TacklesCompletedPer90 = 'TACKLES_COMPLETED_PER90',
	TackleCompletionRatio = 'TACKLE_COMPLETION_RATIO',
	KeyTackles = 'KEY_TACKLES',
	KeyTacklesPer90 = 'KEY_TACKLES_PER90',
	Interceptions = 'INTERCEPTIONS',
	InterceptionsPer90 = 'INTERCEPTIONS_PER90',
	PossWon = 'POSS_WON',
	PossWonPer90 = 'POSS_WON_PER90',
	PressuresAttempted = 'PRESSURES_ATTEMPTED',
	PressuresAttemptedPer90 = 'PRESSURES_ATTEMPTED_PER90',
	PressuresCompleted = 'PRESSURES_COMPLETED',
	PressuresCompletedPer90 = 'PRESSURES_COMPLETED_PER90',
	PressureSuccessRatio = 'PRESSURE_SUCCESS_RATIO',
	Blocks = 'BLOCKS',
	BlocksPer90 = 'BLOCKS_PER90',
	ShotsBlocked = 'SHOTS_BLOCKED',
	ShotsBlockedPer90 = 'SHOTS_BLOCKED_PER90',
	Clearances = 'CLEARANCES',
	ClearancesPer90 = 'CLEARANCES_PER90',

	// Aerial
	HeadersAttempted = 'HEADERS_ATTEMPTED',
	HeadersAttemptedPer90 = 'HEADERS_ATTEMPTED_PER90',
	HeadersWon = 'HEADERS_WON',
	HeadersWonPer90 = 'HEADERS_WON_PER90',
	HeadersLost = 'HEADERS_LOST',
	HeadersLostPer90 = 'HEADERS_LOST_PER90',
	HeadersWonRatio = 'HEADERS_WON_RATIO',
	KeyHeaders = 'KEY_HEADERS',
	KeyHeadersPer90 = 'KEY_HEADERS_PER90',

	// Goalkeeping
	CleanSheets = 'CLEAN_SHEETS',
	CleanSheetsPer90 = 'CLEAN_SHEETS_PER90',
	GoalsConceded = 'GOALS_CONCEDED',
	GoalsConcededPer90 = 'GOALS_CONCEDED_PER90',
	TotalSaves = 'TOTAL_SAVES',
	TotalSavesPer90 = 'TOTAL_SAVES_PER90',
	SaveRatio = 'SAVE_RATIO',
	XsvPercent = 'XSV_PERCENT',
	Xgp = 'XGP',
	XgpPer90 = 'XGP_PER90',
	SavesHeld = 'SAVES_HELD',
	SavesHeldPer90 = 'SAVES_HELD_PER90',
	SavesParried = 'SAVES_PARRIED',
	SavesParriedPer90 = 'SAVES_PARRIED_PER90',
	SavesTipped = 'SAVES_TIPPED',
	SavesTippedPer90 = 'SAVES_TIPPED_PER90',
	PensFaced = 'PENS_FACED',
	PensFacedPer90 = 'PENS_FACED_PER90',
	PensSaved = 'PENS_SAVED',
	PensSavedPer90 = 'PENS_SAVED_PER90',
	PensSavedRatio = 'PENS_SAVED_RATIO',

	// Discipline
	FoulsMade = 'FOULS_MADE',
	FoulsMadePer90 = 'FOULS_MADE_PER90',
	FoulsAgainst = 'FOULS_AGAINST',
	FoulsAgainstPer90 = 'FOULS_AGAINST_PER90',
	YellowCards = 'YELLOW_CARDS',
	YellowCardsPer90 = 'YELLOW_CARDS_PER90',
	RedCards = 'RED_CARDS',
	RedCardsPer90 = 'RED_CARDS_PER90',
	Offsides = 'OFFSIDES',
	OffsidesPer90 = 'OFFSIDES_PER90',
	Mlg = 'MLG',
	MlgPer90 = 'MLG_PER90',

	// Match Impact
	Rating = 'RATING',
	Pom = 'POM',
	GameWinRatio = 'GAME_WIN_RATIO',

	// Derived
	CleanSheetsRatio = 'CLEAN_SHEETS_RATIO',
}

export interface Archetype {
	id: string;
	position: string;
	phase: 'in-possession' | 'out-of-possession';
	name: string;
	tags: string[];
	description: string;
	metrics: ArchetypeMetric[];
	isUserCreated: boolean;
}

export interface ArchetypeMetric {
	metric: PlayerMetric;
	weight: number;
	inverted: boolean;
}

export interface MetricInfo {
	id: PlayerMetric;
	displayName: string;
}
