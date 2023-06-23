use crate::validator::mixed_case;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const BETA_MID_VALUES: [&str; 67] = [
    ")", "(", "/", "=", "\\", "+", "|", "A", "a", "B", "b", "C", "c", "D", "d", "E", "e", "F", "f",
    "G", "g", "H", "h", "I", "i", "K", "k", "L", "l", "M", "m", "N", "n", "O", "o", "P", "p", "Q",
    "q", "R", "r", "S", "s", "T", "t", "U", "u", "V", "v", "W", "w", "X", "x", "Y", "y", "Z", "z",
    ";", ":", "*#1", "#1", "*#2", "#2", "*#3", "#3", "*#5", "#5",
];

const UNI_VALUES: [&str; 67] = [
    "\u{0313}", "\u{0314}", "\u{0301}", "\u{0342}", "\u{0300}", "\u{0308}", "\u{0345}", "\u{0391}",
    "\u{03b1}", "\u{0392}", "\u{03b2}", "\u{039e}", "\u{03be}", "\u{0394}", "\u{03b4}", "\u{0395}",
    "\u{03b5}", "\u{03a6}", "\u{03c6}", "\u{0393}", "\u{03b3}", "\u{0397}", "\u{03b7}", "\u{0399}",
    "\u{03b9}", "\u{039a}", "\u{03ba}", "\u{039b}", "\u{03bb}", "\u{039c}", "\u{03bc}", "\u{039d}",
    "\u{03bd}", "\u{039f}", "\u{03bf}", "\u{03a0}", "\u{03c0}", "\u{0398}", "\u{03b8}", "\u{03a1}",
    "\u{03c1}", "\u{03a3}", "\u{03c3}", "\u{03a4}", "\u{03c4}", "\u{03a5}", "\u{03c5}", "\u{03dc}",
    "\u{03dd}", "\u{03a9}", "\u{03c9}", "\u{03a7}", "\u{03c7}", "\u{03a8}", "\u{03c8}", "\u{0396}",
    "\u{03b6}", "\u{00b3}", "\u{00b7}", "\u{03de}", "\u{03df}", "\u{03da}", "\u{03db}", "\u{03d8}",
    "\u{03d9}", "\u{03e0}", "\u{03e1}",
];

lazy_static! {
    static ref BETA_TO_UNI: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        for (b, u) in BETA_MID_VALUES.iter().zip(UNI_VALUES.iter()) {
            m.insert(*b, *u);
        }
        m
    };
}
lazy_static! {
    static ref RE_UNORDERED_DIACRITICS: Regex = Regex::new(r"(\|*)([\\/=])(\|*)([()\+])").unwrap();
}
lazy_static! {
    static ref RE_FINAL_SIGMA_CHAR: Regex = Regex::new(r"σ([2 .,·;’‐—\n])").unwrap();
}
lazy_static! {
    static ref RE_FINAL_SIGMA_END: Regex = Regex::new(r"σ$").unwrap();
}

const SPECIAL_SIGMAS: [&str; 3] = ["σ1", "σ3", "Σ3"];

fn normalize_unicode<T: Into<String>>(input: T) -> String {
    let input: &str = &input.into();
    input.nfkc().collect::<String>()
}

/// Locates upper case characters marked by "*" and replaces them
/// with the proper ASCII uppercase character.
///
/// Converting the notation "*a" for uppercase alpha to "A":
///
/// ```
///    let string = String::from("*a");
///    let result = betacode::converter::find_upper(string);
///    assert_eq!(result, "A".to_string());
/// ```
///
/// Mantaining the notation of additional characteres as "*#3",
/// otherwise, it could break the pipeline.
///
/// ```
///    let string = String::from("*#3"); // Archaic Koppa
///    let result = betacode::converter::find_upper(string.clone());
///    assert_eq!(result, string);
/// ```
pub fn find_upper<T: Into<String>>(input: T) -> String {
    let mut ascii_chars: Vec<char> = input.into().chars().collect();
    let ascii_enum = ascii_chars.clone();

    let mut output = String::new();
    for (i, character) in ascii_enum.iter().enumerate() {
        if character == &'*' && ascii_chars[i + 1].is_alphabetic() {
            ascii_chars[i + 1] = ascii_chars[i + 1].to_ascii_uppercase();
        }
    }

    for character in ascii_chars {
        output.push(character);
    }

    let re = Regex::new(r"\*([A-Za-z])").unwrap();
    output = re.replace_all(&output, r"$1").to_string();

    output
}

/// Reorder diacritics to the rule: BREATH/DIAIRESIS + ACCENT + SUB-IOTA
///
/// # Examples
///
/// Moving breath before accent
/// ```
/// let string = "A/)".to_string();
/// let result = betacode::converter::reorder_diacritics(string);
/// assert_eq!(result, "A)/".to_string());
/// ```
///
/// Moving breath before accent and sub-iota after all diacritics
///
/// ```
/// let string = "A|/)".to_string();
/// let result = betacode::converter::reorder_diacritics(string);
/// assert_eq!(result, "A)/|".to_string());
/// ```
/// Moving diairesis before accent
///
/// ```
/// let string = "A/+".to_string();
/// let result = betacode::converter::reorder_diacritics(string);
/// assert_eq!(result, "A+/".to_string());
/// ```
pub fn reorder_diacritics<T: Into<String>>(input: T) -> String {
    let input: String = input.into();
    let output = RE_UNORDERED_DIACRITICS.replace_all(&input, "$4$2$1$3".to_string());
    output.into()
}

/// Converts the betacode entry from ASCII (with mixed cases) to Greek Unicode.
fn ascii_to_unicode<T: Into<String>>(input: T) -> String {
    let mut output: String = input.into();
    BETA_MID_VALUES.iter().for_each(|c| {
        output = output.replace(*c, BETA_TO_UNI.get(c).unwrap());
    });

    output
}

/// Handles the specific rules for final sigmas.
///
pub fn sigma_handler<T: Into<String>>(input: T) -> String {
    let input: String = input.into();

    let output = RE_FINAL_SIGMA_CHAR.replace_all(&input, r"ς$1");
    RE_FINAL_SIGMA_END.replace_all(&output, r"ς").into()
}

/// Finds and replaces forced medial sigmas and lunate sigmas.
///
pub fn special_sigma<T: Into<String>>(input: T) -> String {
    input
        .into()
        .replace("σ1", "\u{03c2}")
        .replace("σ3", "\u{03f2}")
        .replace("Σ3", "\u{03f9}")
}

/// Applies the conversion pipeline.
///
/// The conversion pipeline is:
/// - lower the case of the whole entry if needed;
///     - text in all upper case will be lowercased
/// - substitutes the `*+letter` sequences to upper case letter;
/// - normalizes the diacritics ordering;
/// - converts from ascii betacode to unicode Greek;
/// - applies specific conversion rules to sigmas.
///
pub fn convert<T: Into<String>>(input: T) -> String {
    let mut output = input.into();

    // Handles valid mixed case
    match mixed_case(&output) {
        Ok(_) => {
            if output.contains('*') {
                output = output.to_lowercase();
                output = find_upper(output);
            } else if output.find(char::is_lowercase).is_none() {
                output = output.to_lowercase();
            }
        }
        Err(_) => panic!("Mixed case notation"),
    }
    // Checks for unordered diacritics
    if RE_UNORDERED_DIACRITICS.is_match(&output) {
        output = reorder_diacritics(output);
    }

    // Main conversion algorithm
    output = ascii_to_unicode(output);

    // Handles final sigma rules
    if RE_FINAL_SIGMA_END.is_match(&output) || RE_FINAL_SIGMA_CHAR.is_match(&output) {
        output = sigma_handler(output);
    }

    // Normalizes output
    output = normalize_unicode(output);

    // Handles special sigma classes
    if SPECIAL_SIGMAS.iter().any(|c| output.contains(c)) {
        output = special_sigma(output);
    }

    output
}
#[cfg(test)]
mod tests_converter;
