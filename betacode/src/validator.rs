use regex::Regex;
use std::fmt;

/// Provides different classes of validation errors.
/// - [ValidationError::InvalidChars]: Denotes cases in which the characters passed are not ASCII
/// or not supported by this implementation of Betacode.
/// - [ValidationError::InvalidDiacriticOrder]: Denotes cases in which the sequence
/// `BREATH/DIAIRESIS + ACCENT + SUB-IOTA` is not followed.
#[derive(Debug)]
pub enum ValidationError {
    NotASCII(Vec<char>),
    InvalidChars(Vec<char>),
    InvalidDiacriticOrder(Vec<String>),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::NotASCII(a) => write!(f, "Non ASCII chars {:?}", a),
            ValidationError::InvalidChars(a) => write!(f, "Invalid characteres {:?}", a),
            ValidationError::InvalidDiacriticOrder(a) => {
                write!(f, "Invalid diacritic order: {:?}", a)
            }
        }
    }
}

fn diacritics_ordered<T: Into<String>>(input: T) -> Result<(), ValidationError> {
    let input: String = input.into();
    let re = Regex::new(r"\|[()/\\+]+|[\\/][()+]|[qrtypsdfgklmnbcxz ][()\\/+|]+").unwrap();

    let matches: Vec<regex::Match> = re.find_iter(&input).collect();
    match matches.len() {
        0 => Ok(()),
        _ => {
            let v: Vec<String> = matches
                .into_iter()
                .map(|m| m.as_str().to_string())
                .collect();
            Err(ValidationError::InvalidDiacriticOrder(v))
        }
    }
}

fn standard_characteres<T: Into<String>>(input: T) -> Result<(), ValidationError> {
    let input: String = input.into();
    let valid_chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '*', '#', '|', ')',
        '(', '/', '\\', '.', ';', ':', '1', '2', '3', ',', '\'', '+', '=', ' ', '\n',
    ];
    match input.chars().all(|c| valid_chars.contains(&c)) {
        true => Ok(()),
        false => {
            let mut invalid_chars: Vec<char> =
                input.chars().filter(|c| !valid_chars.contains(c)).collect();
            invalid_chars.dedup();
            Err(ValidationError::InvalidChars(invalid_chars))
        }
    }
}

/// Validates whether or not a given String or str is proper ASCII Betacode
///
/// Examples:
///
/// If the text is input in proper ASCII Betacode (and the [converter](super::converter) can convert it), it
/// returns Ok().
///
/// ```
/// let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
/// assert!(betacode::validator::validate(input).is_ok());
/// let input = String::from("çi)");
/// assert!(betacode::validator::validate(input).is_err());
/// ```
///
/// Otherwise, it specifies what error occurred.
///
/// For example, if passed a string with non-ASCII characters such as ἄλγεα,
/// it stores a list of all characters that break the validation in the enum
/// [ValidationError::InvalidChars].
///
/// ```
/// let input = String::from("ἄλγεα");
/// let result = betacode::validator::validate(input);
/// assert!(result.is_err());
/// match result {
///     Ok(_) => (),
///     Err(e) => {
///         if let betacode::validator::ValidationError::NotASCII(b) = e {
///             assert_eq!(b, vec!['ἄ', 'λ','γ','ε','α']);
///         }
///     }
/// }
/// ```
///
/// If the string is ASCII, but the proper conversion rule has not been implemented, it stores
/// the list of characters that are not convertable in the enum [ValidationError::InvalidChars]
///
/// ```
/// let input = String::from("9");
/// let result = betacode::validator::validate(input);
/// assert!(result.is_err());
/// match result {
///     Ok(_) => (),
///     Err(e) => {
///         if let betacode::validator::ValidationError::InvalidChars(b) = e {
///             assert_eq!(b, vec!['9']);
///         }
///     }
/// }
/// ```
///
/// If, on other hand, the text contains an order of diacritics that can not
/// be directly converted, it returns the list of sequences that are not valid.
/// The converter module still can convert it, but this is implemented to assure
/// that the corpus is properly built for other tools to operate.
/// It stores all the patterns that break the `BREATH/DIAIRESIS + ACCENT + SUB-IOTA`
/// order in [ValidationError::InvalidDiacriticOrder].
///
///
/// ```
/// let input = String::from("h\\( a/)ndra");
/// let result = betacode::validator::validate(input);
/// assert!(result.is_err());
/// match result {
///     Ok(_) => (),
///     Err(e) => {
///         if let betacode::validator::ValidationError::InvalidDiacriticOrder(b) = e {
///             assert_eq!(b, vec!["\\(".to_string(), "/)".to_string()]);
///         }
///     }
/// }
/// ```
///
///
pub fn validate<T: Into<String>>(input: T) -> Result<(), ValidationError> {
    let input: String = input.into();

    check_ascii(&input)?;
    diacritics_ordered(&input)?;
    standard_characteres(input)?;
    Ok(())
}

fn check_ascii<T: Into<String>>(input: T) -> Result<(), ValidationError> {
    let input: String = input.into();

    if !input.is_ascii() {
        let mut non_ascii_chars: Vec<char> = input.chars().filter(|c| !c.is_ascii()).collect();
        non_ascii_chars.dedup();
        Err(ValidationError::NotASCII(non_ascii_chars))
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod test;
