use super::FieldValue;
use phf::phf_map;

/// Static compile-time hash map mapping 3-letter nationality codes to full country names.
/// Covers all FIFA-recognized nations plus common Football Manager codes.
static NATIONALITIES: phf::Map<&'static str, &'static str> = phf_map! {
    // -----------------------------------------------------------------------
    // UEFA (Europe)
    // -----------------------------------------------------------------------
    "ALB" => "Albania",
    "AND" => "Andorra",
    "ARM" => "Armenia",
    "AUT" => "Austria",
    "AZE" => "Azerbaijan",
    "BLR" => "Belarus",
    "BEL" => "Belgium",
    "BIH" => "Bosnia and Herzegovina",
    "BUL" => "Bulgaria",
    "CRO" => "Croatia",
    "CYP" => "Cyprus",
    "CZE" => "Czech Republic",
    "DEN" => "Denmark",
    "ENG" => "England",
    "EST" => "Estonia",
    "FRO" => "Faroe Islands",
    "FIN" => "Finland",
    "FRA" => "France",
    "GEO" => "Georgia",
    "GER" => "Germany",
    "GIB" => "Gibraltar",
    "GRE" => "Greece",
    "HUN" => "Hungary",
    "ISL" => "Iceland",
    "ISR" => "Israel",
    "ITA" => "Italy",
    "KAZ" => "Kazakhstan",
    "KVX" => "Kosovo",
    "LVA" => "Latvia",
    "LIE" => "Liechtenstein",
    "LTU" => "Lithuania",
    "LUX" => "Luxembourg",
    "MLT" => "Malta",
    "MDA" => "Moldova",
    "MON" => "Monaco",
    "MNE" => "Montenegro",
    "NED" => "Netherlands",
    "MKD" => "North Macedonia",
    "NIR" => "Northern Ireland",
    "NOR" => "Norway",
    "POL" => "Poland",
    "POR" => "Portugal",
    "IRL" => "Republic of Ireland",
    "ROU" => "Romania",
    "RUS" => "Russia",
    "SMR" => "San Marino",
    "SCO" => "Scotland",
    "SRB" => "Serbia",
    "SVK" => "Slovakia",
    "SVN" => "Slovenia",
    "ESP" => "Spain",
    "SWE" => "Sweden",
    "SUI" => "Switzerland",
    "TUR" => "Turkey",
    "UKR" => "Ukraine",
    "WAL" => "Wales",

    // -----------------------------------------------------------------------
    // CONMEBOL (South America)
    // -----------------------------------------------------------------------
    "ARG" => "Argentina",
    "BOL" => "Bolivia",
    "BRA" => "Brazil",
    "CHI" => "Chile",
    "COL" => "Colombia",
    "ECU" => "Ecuador",
    "GUY" => "Guyana",
    "PAR" => "Paraguay",
    "PER" => "Peru",
    "SUR" => "Suriname",
    "URU" => "Uruguay",
    "VEN" => "Venezuela",

    // -----------------------------------------------------------------------
    // CONCACAF (North / Central America & Caribbean)
    // -----------------------------------------------------------------------
    "AIA" => "Anguilla",
    "ATG" => "Antigua and Barbuda",
    "ARU" => "Aruba",
    "BAH" => "Bahamas",
    "BRB" => "Barbados",
    "BLZ" => "Belize",
    "BER" => "Bermuda",
    "BES" => "Bonaire",
    "VGB" => "British Virgin Islands",
    "CAN" => "Canada",
    "CAY" => "Cayman Islands",
    "CRC" => "Costa Rica",
    "CUB" => "Cuba",
    "CUW" => "Curaçao",
    "DMA" => "Dominica",
    "DOM" => "Dominican Republic",
    "SLV" => "El Salvador",
    "GUF" => "French Guiana",
    "GRN" => "Grenada",
    "GLP" => "Guadeloupe",
    "GUA" => "Guatemala",
    "HAI" => "Haiti",
    "HON" => "Honduras",
    "JAM" => "Jamaica",
    "MTQ" => "Martinique",
    "MEX" => "Mexico",
    "MSR" => "Montserrat",
    "NCA" => "Nicaragua",
    "PAN" => "Panama",
    "PUR" => "Puerto Rico",
    "SKN" => "Saint Kitts and Nevis",
    "LCA" => "Saint Lucia",
    "VIN" => "Saint Vincent and the Grenadines",
    "SMA" => "Sint Maarten",
    "TCN" => "Turks and Caicos Islands",
    "TTO" => "Trinidad and Tobago",
    "USA" => "United States",
    "VIR" => "US Virgin Islands",

    // -----------------------------------------------------------------------
    // CAF (Africa)
    // -----------------------------------------------------------------------
    "ALG" => "Algeria",
    "ANG" => "Angola",
    "BEN" => "Benin",
    "BOT" => "Botswana",
    "BFA" => "Burkina Faso",
    "BDI" => "Burundi",
    "CMR" => "Cameroon",
    "CPV" => "Cape Verde",
    "CAF" => "Central African Republic",
    "CHA" => "Chad",
    "COM" => "Comoros",
    "CGO" => "Congo",
    "COD" => "Congo DR",
    "CIV" => "Côte d'Ivoire",
    "DJI" => "Djibouti",
    "EGY" => "Egypt",
    "EQG" => "Equatorial Guinea",
    "ERI" => "Eritrea",
    "ETH" => "Ethiopia",
    "GAB" => "Gabon",
    "GAM" => "Gambia",
    "GHA" => "Ghana",
    "GUI" => "Guinea",
    "GNB" => "Guinea-Bissau",
    "KEN" => "Kenya",
    "LES" => "Lesotho",
    "LBR" => "Liberia",
    "LBY" => "Libya",
    "MAD" => "Madagascar",
    "MWI" => "Malawi",
    "MLI" => "Mali",
    "MTN" => "Mauritania",
    "MRI" => "Mauritius",
    "MAR" => "Morocco",
    "MOZ" => "Mozambique",
    "NAM" => "Namibia",
    "NIG" => "Niger",
    "NGA" => "Nigeria",
    "RWA" => "Rwanda",
    "STP" => "São Tomé and Príncipe",
    "SEN" => "Senegal",
    "SEY" => "Seychelles",
    "SLE" => "Sierra Leone",
    "SOM" => "Somalia",
    "RSA" => "South Africa",
    "SSD" => "South Sudan",
    "SUD" => "Sudan",
    "SWZ" => "Eswatini",
    "TAN" => "Tanzania",
    "TOG" => "Togo",
    "TUN" => "Tunisia",
    "UGA" => "Uganda",
    "ZAM" => "Zambia",
    "ZIM" => "Zimbabwe",

    // -----------------------------------------------------------------------
    // AFC (Asia)
    // -----------------------------------------------------------------------
    "AFG" => "Afghanistan",
    "AUS" => "Australia",
    "BHR" => "Bahrain",
    "BAN" => "Bangladesh",
    "BHU" => "Bhutan",
    "BRU" => "Brunei",
    "CAM" => "Cambodia",
    "CHN" => "China",
    "TPE" => "Chinese Taipei",
    "GUM" => "Guam",
    "HKG" => "Hong Kong",
    "IND" => "India",
    "IDN" => "Indonesia",
    "IRN" => "Iran",
    "IRQ" => "Iraq",
    "JPN" => "Japan",
    "JOR" => "Jordan",
    "PRK" => "North Korea",
    "KOR" => "South Korea",
    "KUW" => "Kuwait",
    "KGZ" => "Kyrgyzstan",
    "LAO" => "Laos",
    "LIB" => "Lebanon",
    "MAC" => "Macau",
    "MAS" => "Malaysia",
    "MDV" => "Maldives",
    "MNG" => "Mongolia",
    "MYA" => "Myanmar",
    "NEP" => "Nepal",
    "OMN" => "Oman",
    "PAK" => "Pakistan",
    "PLE" => "Palestine",
    "PHI" => "Philippines",
    "QAT" => "Qatar",
    "KSA" => "Saudi Arabia",
    "SIN" => "Singapore",
    "SRI" => "Sri Lanka",
    "SYR" => "Syria",
    "TJK" => "Tajikistan",
    "THA" => "Thailand",
    "TLS" => "Timor-Leste",
    "TKM" => "Turkmenistan",
    "UAE" => "United Arab Emirates",
    "UZB" => "Uzbekistan",
    "VIE" => "Vietnam",
    "YEM" => "Yemen",

    // -----------------------------------------------------------------------
    // OFC (Oceania)
    // -----------------------------------------------------------------------
    "ASA" => "American Samoa",
    "COK" => "Cook Islands",
    "FIJ" => "Fiji",
    "KIR" => "Kiribati",
    "MHL" => "Marshall Islands",
    "FSM" => "Micronesia",
    "NCL" => "New Caledonia",
    "NZL" => "New Zealand",
    "NIU" => "Niue",
    "NFK" => "Norfolk Island",
    "PLW" => "Palau",
    "PNG" => "Papua New Guinea",
    "SAM" => "Samoa",
    "SOL" => "Solomon Islands",
    "TAH" => "Tahiti",
    "TGA" => "Tonga",
    "TUV" => "Tuvalu",
    "VAN" => "Vanuatu",
    "WLF" => "Wallis and Futuna",
};

/// Look up a nationality string and return the full country name.
///
/// Resolution logic:
///
/// 1. **Empty / whitespace-only input** → `FieldValue::Null`
/// 2. **3-letter code** (case-insensitive) that matches a known entry
///    → `FieldValue::String(full_name)` (e.g. `"ENG"`, `"eng"`, `"Eng"`
///    all return `"England"`)
/// 3. **Unrecognised 3-letter code** → returned as-is
///    (could be a lesser-known or custom code not in the map)
/// 4. **Full name or any other string** → returned as-is
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::FieldValue;
/// use fm_valuescout_lib::parsers::nationality::lookup_nationality;
///
/// assert!(matches!(
///     lookup_nationality("ENG"),
///     FieldValue::String(v) if v == "England"
/// ));
/// assert!(matches!(
///     lookup_nationality("eng"),
///     FieldValue::String(v) if v == "England"
/// ));
/// assert!(matches!(
///     lookup_nationality(""),
///     FieldValue::Null
/// ));
/// ```
pub fn lookup_nationality(input: &str) -> FieldValue {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return FieldValue::Null;
    }

    let upper = trimmed.to_uppercase();

    // Case-insensitive code lookup: convert to uppercase and match against map keys.
    if let Some(full_name) = NATIONALITIES.get(&upper) {
        return FieldValue::String(full_name.to_string());
    }

    // Unrecognised 3-letter code → return as-is
    if upper.len() == 3 && upper.chars().all(|c| c.is_ascii_alphabetic()) {
        return FieldValue::String(trimmed.to_string());
    }

    // Full name or any other string → return as-is
    FieldValue::String(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Happy path: known codes ----

    #[test]
    fn test_england_uppercase() {
        let result = lookup_nationality("ENG");
        assert!(matches!(result, FieldValue::String(v) if v == "England"));
    }

    #[test]
    fn test_england_lowercase() {
        let result = lookup_nationality("eng");
        assert!(matches!(result, FieldValue::String(v) if v == "England"));
    }

    #[test]
    fn test_england_mixed_case() {
        let result = lookup_nationality("Eng");
        assert!(matches!(result, FieldValue::String(v) if v == "England"));
    }

    #[test]
    fn test_france() {
        let result = lookup_nationality("FRA");
        assert!(matches!(result, FieldValue::String(v) if v == "France"));
    }

    #[test]
    fn test_germany() {
        let result = lookup_nationality("GER");
        assert!(matches!(result, FieldValue::String(v) if v == "Germany"));
    }

    #[test]
    fn test_spain() {
        let result = lookup_nationality("ESP");
        assert!(matches!(result, FieldValue::String(v) if v == "Spain"));
    }

    #[test]
    fn test_italy() {
        let result = lookup_nationality("ITA");
        assert!(matches!(result, FieldValue::String(v) if v == "Italy"));
    }

    #[test]
    fn test_brazil() {
        let result = lookup_nationality("BRA");
        assert!(matches!(result, FieldValue::String(v) if v == "Brazil"));
    }

    #[test]
    fn test_argentina() {
        let result = lookup_nationality("ARG");
        assert!(matches!(result, FieldValue::String(v) if v == "Argentina"));
    }

    #[test]
    fn test_united_states() {
        let result = lookup_nationality("USA");
        assert!(matches!(result, FieldValue::String(v) if v == "United States"));
    }

    #[test]
    fn test_australia() {
        let result = lookup_nationality("AUS");
        assert!(matches!(result, FieldValue::String(v) if v == "Australia"));
    }

    #[test]
    fn test_japan() {
        let result = lookup_nationality("JPN");
        assert!(matches!(result, FieldValue::String(v) if v == "Japan"));
    }

    #[test]
    fn test_nigeria() {
        let result = lookup_nationality("NGA");
        assert!(matches!(result, FieldValue::String(v) if v == "Nigeria"));
    }

    #[test]
    fn test_multi_word_name() {
        let result = lookup_nationality("RSA");
        assert!(matches!(result, FieldValue::String(v) if v == "South Africa"));
    }

    #[test]
    fn test_with_apostrophe() {
        let result = lookup_nationality("CIV");
        assert!(matches!(result, FieldValue::String(v) if v == "Côte d'Ivoire"));
    }

    // ---- Full name passthrough ----

    #[test]
    fn test_full_name_passthrough() {
        let result = lookup_nationality("England");
        assert!(matches!(result, FieldValue::String(v) if v == "England"));
    }

    #[test]
    fn test_full_name_lowercase_passthrough() {
        let result = lookup_nationality("france");
        assert!(matches!(result, FieldValue::String(v) if v == "france"));
    }

    #[test]
    fn test_long_name_passthrough() {
        let result = lookup_nationality("Bosnia and Herzegovina");
        assert!(matches!(result, FieldValue::String(v) if v == "Bosnia and Herzegovina"));
    }

    // ---- Unrecognised codes ----

    #[test]
    fn test_unrecognised_code_returned_as_is() {
        let result = lookup_nationality("XYZ");
        assert!(matches!(result, FieldValue::String(v) if v == "XYZ"));
    }

    #[test]
    fn test_unrecognised_code_lowercase() {
        let result = lookup_nationality("xyz");
        assert!(matches!(result, FieldValue::String(v) if v == "xyz"));
    }

    // ---- Empty / whitespace ----

    #[test]
    fn test_empty_string() {
        assert!(matches!(lookup_nationality(""), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(lookup_nationality("   "), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_surrounding_code() {
        let result = lookup_nationality("  ENG  ");
        assert!(matches!(result, FieldValue::String(v) if v == "England"));
    }

    // ---- Edge cases ----

    #[test]
    fn test_two_letter_input_not_treated_as_code() {
        // "FR" is not 3 chars, so it falls through to "return as-is"
        let result = lookup_nationality("FR");
        assert!(matches!(result, FieldValue::String(v) if v == "FR"));
    }

    #[test]
    fn test_four_letter_input_not_treated_as_code() {
        let result = lookup_nationality("FRA_");
        assert!(matches!(result, FieldValue::String(v) if v == "FRA_"));
    }

    #[test]
    fn test_code_with_digits_not_treated_as_unknown_code() {
        // "AB3" has a digit, so it's not 3-alpha → not treated as an unknown code
        let result = lookup_nationality("AB3");
        assert!(matches!(result, FieldValue::String(v) if v == "AB3"));
    }

    #[test]
    fn test_all_continents_have_at_least_one_entry() {
        // UEFA
        assert!(matches!(lookup_nationality("ENG"), FieldValue::String(_)));
        // CONMEBOL
        assert!(matches!(lookup_nationality("ARG"), FieldValue::String(_)));
        // CONCACAF
        assert!(matches!(lookup_nationality("USA"), FieldValue::String(_)));
        // CAF
        assert!(matches!(lookup_nationality("NGA"), FieldValue::String(_)));
        // AFC
        assert!(matches!(lookup_nationality("JPN"), FieldValue::String(_)));
        // OFC
        assert!(matches!(lookup_nationality("NZL"), FieldValue::String(_)));
    }

    #[test]
    fn test_non_alpha_code_not_confused_with_unknown_code() {
        // "123" has no alpha chars → not a 3-letter code → returned as-is
        let result = lookup_nationality("123");
        assert!(matches!(result, FieldValue::String(v) if v == "123"));
    }

    #[test]
    fn test_unicode_full_name() {
        let result = lookup_nationality("São Tomé and Príncipe");
        assert!(matches!(result, FieldValue::String(v) if v == "São Tomé and Príncipe"));
    }
}
