use std::collections::HashMap;

pub mod appearances;
pub mod computed;
pub mod currency;
pub mod expansion;
pub mod footedness;
pub mod nationality;
pub mod positions;
pub mod row_parser;
pub mod units;
pub mod validator;
pub mod wage;

// ---------------------------------------------------------------------------
// FieldValue — tagged union for parsed CSV cell values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum FieldValue {
    Int(i64),
    Float(f64),
    String(String),
    Currency(f64),
    Wage {
        value: f64,
        denomination: Option<String>,
    },
    Appearances {
        starts: i64,
        subs: i64,
    },
    Position(Vec<String>),
    FootScore {
        score: Option<i64>,
        raw: String,
    },
    Null,
}

// ---------------------------------------------------------------------------
// PlayerRecord — one parsed row from a CSV file
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct PlayerRecord {
    // Identity
    pub uid: Option<String>,
    pub name: Option<String>,
    pub nation: Option<String>,
    pub height: Option<i64>,
    pub left_foot_raw: Option<String>,
    pub left_foot_score: Option<i64>,
    pub right_foot_raw: Option<String>,
    pub right_foot_score: Option<i64>,

    // Season
    pub club: Option<String>,
    pub position_raw: Option<String>,
    pub age: Option<i64>,
    pub minutes: Option<i64>,
    pub starts: Option<i64>,
    pub subs: Option<i64>,
    pub expires: Option<String>,

    // Technical
    pub cor: Option<i64>,
    pub cro: Option<i64>,
    pub dri: Option<i64>,
    pub fin: Option<i64>,
    pub fir: Option<i64>,
    pub fre: Option<i64>,
    pub hea: Option<i64>,
    pub lon: Option<i64>,
    pub lon_thr: Option<i64>,
    pub mar: Option<i64>,
    pub pas: Option<i64>,
    pub pen: Option<i64>,
    pub tck: Option<i64>,
    pub tec: Option<i64>,

    // Mental
    pub agg: Option<i64>,
    pub ant: Option<i64>,
    pub bra: Option<i64>,
    pub cmp: Option<i64>,
    pub cnt: Option<i64>,
    pub dec: Option<i64>,
    pub det: Option<i64>,
    pub fla: Option<i64>,
    pub inf: Option<i64>,
    pub lea: Option<i64>,
    pub otb: Option<i64>,
    pub pos: Option<i64>,
    pub tea: Option<i64>,
    pub vis: Option<i64>,
    pub wor: Option<i64>,

    // Physical
    pub acc: Option<i64>,
    pub agi: Option<i64>,
    pub bal: Option<i64>,
    pub jum: Option<i64>,
    pub nat_fit: Option<i64>,
    pub pac: Option<i64>,
    pub sta: Option<i64>,
    pub str: Option<i64>,

    // Goalkeeper attributes
    pub aer: Option<i64>,
    pub com_gk: Option<i64>,
    pub ecc: Option<i64>,
    pub han: Option<i64>,
    pub kic: Option<i64>,
    pub one_on_one: Option<i64>,
    pub pun: Option<i64>,
    pub ref_gk: Option<i64>,
    pub rus: Option<i64>,
    pub thr: Option<i64>,
    pub cmd: Option<i64>,

    // Attacking & Finishing
    pub goals_xg: Option<f64>,
    pub npxg: Option<f64>,
    pub xg_op: Option<f64>,
    pub xg_per_shot: Option<f64>,
    pub shots: Option<f64>,
    pub shots_on_target: Option<f64>,
    pub sot_pct: Option<f64>,
    pub conv_pct: Option<f64>,
    pub pens_scored: Option<f64>,
    pub pens_taken: Option<f64>,

    // Creativity & Chance Creation
    pub assists: Option<f64>,
    pub xa: Option<f64>,
    pub key_passes: Option<f64>,
    pub chances_created: Option<f64>,
    pub op_key_passes: Option<f64>,
    pub through_balls: Option<f64>,

    // Transition & Ball Progression
    pub dribbles_per_game: Option<f64>,
    pub progressive_passes: Option<f64>,
    pub progressive_runs: Option<f64>,
    pub passes_completed: Option<f64>,
    pub passes_attempted: Option<f64>,
    pub pass_completion_pct: Option<f64>,
    pub crosses_attempted: Option<f64>,
    pub crosses_completed: Option<f64>,
    pub cross_completion_pct: Option<f64>,

    // Defensive Actions
    pub tackles_per_game: Option<f64>,
    pub tackles_completed: Option<f64>,
    pub tackle_completion_pct: Option<f64>,
    pub interceptions_per_game: Option<f64>,
    pub clearances: Option<f64>,
    pub blocks: Option<f64>,
    pub possession_won: Option<f64>,
    pub possession_lost: Option<f64>,

    // Aerial Presence
    pub headers_won: Option<f64>,
    pub headers_lost: Option<f64>,
    pub headers_won_pct: Option<f64>,

    // Goalkeeping Stats
    pub saves: Option<f64>,
    pub save_pct: Option<f64>,
    pub goals_conceded: Option<f64>,
    pub clean_sheets: Option<f64>,
    pub penalties_saved: Option<f64>,
    pub expected_saves: Option<f64>,

    // Discipline
    pub fouls_made: Option<f64>,
    pub fouls_against: Option<f64>,
    pub yellow_cards: Option<f64>,
    pub red_cards: Option<f64>,
    pub offsides: Option<f64>,

    // Match Impact
    pub distance_covered: Option<f64>,
    pub average_rating: Option<f64>,
    pub player_of_match: Option<f64>,

    // Optional extra columns (may or may not be present in CSV)
    pub current_ability: Option<i64>,
    pub potential_ability: Option<i64>,
    pub transfer_value: Option<f64>,
    pub wage_value: Option<f64>,
    pub wage_denomination: Option<String>,
    pub second_nationality: Option<String>,
}

impl Default for PlayerRecord {
    fn default() -> Self {
        Self {
            uid: None,
            name: None,
            nation: None,
            height: None,
            left_foot_raw: None,
            left_foot_score: None,
            right_foot_raw: None,
            right_foot_score: None,
            club: None,
            position_raw: None,
            age: None,
            minutes: None,
            starts: None,
            subs: None,
            expires: None,
            cor: None,
            cro: None,
            dri: None,
            fin: None,
            fir: None,
            fre: None,
            hea: None,
            lon: None,
            lon_thr: None,
            mar: None,
            pas: None,
            pen: None,
            tck: None,
            tec: None,
            agg: None,
            ant: None,
            bra: None,
            cmp: None,
            cnt: None,
            dec: None,
            det: None,
            fla: None,
            inf: None,
            lea: None,
            otb: None,
            pos: None,
            tea: None,
            vis: None,
            wor: None,
            acc: None,
            agi: None,
            bal: None,
            jum: None,
            nat_fit: None,
            pac: None,
            sta: None,
            str: None,
            aer: None,
            com_gk: None,
            ecc: None,
            han: None,
            kic: None,
            one_on_one: None,
            pun: None,
            ref_gk: None,
            rus: None,
            thr: None,
            cmd: None,
            goals_xg: None,
            npxg: None,
            xg_op: None,
            xg_per_shot: None,
            shots: None,
            shots_on_target: None,
            sot_pct: None,
            conv_pct: None,
            pens_scored: None,
            pens_taken: None,
            assists: None,
            xa: None,
            key_passes: None,
            chances_created: None,
            op_key_passes: None,
            through_balls: None,
            dribbles_per_game: None,
            progressive_passes: None,
            progressive_runs: None,
            passes_completed: None,
            passes_attempted: None,
            pass_completion_pct: None,
            crosses_attempted: None,
            crosses_completed: None,
            cross_completion_pct: None,
            tackles_per_game: None,
            tackles_completed: None,
            tackle_completion_pct: None,
            interceptions_per_game: None,
            clearances: None,
            blocks: None,
            possession_won: None,
            possession_lost: None,
            headers_won: None,
            headers_lost: None,
            headers_won_pct: None,
            saves: None,
            save_pct: None,
            goals_conceded: None,
            clean_sheets: None,
            penalties_saved: None,
            expected_saves: None,
            fouls_made: None,
            fouls_against: None,
            yellow_cards: None,
            red_cards: None,
            offsides: None,
            distance_covered: None,
            average_rating: None,
            player_of_match: None,
            current_ability: None,
            potential_ability: None,
            transfer_value: None,
            wage_value: None,
            wage_denomination: None,
            second_nationality: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Schema — column-name → parser + column-name → index mappings
// ---------------------------------------------------------------------------

type FieldParser = fn(&str) -> FieldValue;

pub struct Schema {
    pub parsers: HashMap<String, FieldParser>,
    pub column_index: HashMap<String, usize>,
}

impl Schema {
    pub fn from_headers(headers: &[String]) -> Self {
        let mut column_index = HashMap::new();
        for (i, h) in headers.iter().enumerate() {
            column_index.insert(h.clone(), i);
        }
        Schema {
            parsers: HashMap::new(),
            column_index,
        }
    }
}
