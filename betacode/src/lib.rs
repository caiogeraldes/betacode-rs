//! Tools for Betacode conversion and validation.
//!
//! *Beware*: the normalization of unicode characters used here is the NFKC, for compatibility.
//!
//! Examples:
//!
//! ```
//! use unicode_normalization::UnicodeNormalization;
//! let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
//! let output = String::from("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
//! let result = betacode::converter::convert(input);
//! assert_eq!(result, output);
//! ```
//!

/// Module containing functions necessary for the betacode conversion.
pub mod converter {
    use regex::Regex;
    use unicode_normalization::UnicodeNormalization;

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
        let re = Regex::new(r"(\|*)([\\/=])(\|*)([()\+])").unwrap();
        let input: String = input.into();
        let output = re.replace_all(&input, "$4$2$1$3".to_string());
        output.into()
    }

    /// Converts the betacode entry from ASCII (with mixed cases) to Greek Unicode.
    fn ascii_to_unicode<T: Into<String>>(input: T) -> String {
        let mut output: String = input.into();
        output = output
            .replace(')', "\u{0313}")
            .replace('(', "\u{0314}")
            .replace('/', "\u{0301}")
            .replace('=', "\u{0342}")
            .replace('\\', "\u{0300}")
            .replace('+', "\u{0308}")
            .replace('|', "\u{0345}")
            .replace('A', "\u{0391}")
            .replace('a', "\u{03b1}")
            .replace('B', "\u{0392}")
            .replace('b', "\u{03b2}")
            .replace('C', "\u{039e}")
            .replace('c', "\u{03be}")
            .replace('D', "\u{0394}")
            .replace('d', "\u{03b4}")
            .replace('E', "\u{0395}")
            .replace('e', "\u{03b5}")
            .replace('F', "\u{03a6}")
            .replace('f', "\u{03c6}")
            .replace('G', "\u{0393}")
            .replace('g', "\u{03b3}")
            .replace('H', "\u{0397}")
            .replace('h', "\u{03b7}")
            .replace('I', "\u{0399}")
            .replace('i', "\u{03b9}")
            .replace('K', "\u{039a}")
            .replace('k', "\u{03ba}")
            .replace('L', "\u{039b}")
            .replace('l', "\u{03bb}")
            .replace('M', "\u{039c}")
            .replace('m', "\u{03bc}")
            .replace('N', "\u{039d}")
            .replace('n', "\u{03bd}")
            .replace('O', "\u{039f}")
            .replace('o', "\u{03bf}")
            .replace('P', "\u{03a0}")
            .replace('p', "\u{03c0}")
            .replace('Q', "\u{0398}")
            .replace('q', "\u{03b8}")
            .replace('R', "\u{03a1}")
            .replace('r', "\u{03c1}")
            .replace('S', "\u{03a3}")
            .replace('s', "\u{03c3}")
            .replace('T', "\u{03a4}")
            .replace('t', "\u{03c4}")
            .replace('U', "\u{03a5}")
            .replace('u', "\u{03c5}")
            .replace('V', "\u{03dc}")
            .replace('v', "\u{03dd}")
            .replace('W', "\u{03a9}")
            .replace('w', "\u{03c9}")
            .replace('X', "\u{03a7}")
            .replace('x', "\u{03c7}")
            .replace('Y', "\u{03a8}")
            .replace('y', "\u{03c8}")
            .replace('Z', "\u{0396}")
            .replace('z', "\u{03b6}")
            .replace("*#1", "\u{03de}") // Koppa
            .replace("#1", "\u{03df}")
            .replace("*#2", "\u{03da}") // Stigma
            .replace("#2", "\u{03db}")
            .replace("*#3", "\u{03d8}") // Archaic Koppa
            .replace("#3", "\u{03d9}")
            .replace("*#5", "\u{03e0}") // Sampi
            .replace("#5", "\u{03e1}")
            .replace(';', "\u{00b3}") // Greek question mark
            .replace(':', "\u{00b7}"); // Middle Dot

        output
    }

    /// Handles the specific rules for different classes of sigma.
    ///
    /// - Finds and replaces the final sigma (all notations);
    /// - Finds and replaces forced medial sigmas and lunate sigmas.
    pub fn sigma_handler<T: Into<String>>(input: T) -> String {
        let re_final_sigma = Regex::new(r"σ([2 .,·;’‐—\n])").unwrap();
        let input: String = input.into();

        let output = re_final_sigma.replace_all(&input, r"ς$1");

        let re_final_sigma = Regex::new(r"σ$").unwrap();
        let output = re_final_sigma.replace_all(&output, r"ς");

        output
            .replace("s1", "\u{03c2}")
            .replace("s3", "\u{03f2}")
            .replace("S3", "\u{03f9}")
    }

    /// Applies the conversion pipeline.
    ///
    /// The conversion pipeline is:
    /// - lower the case of the whole entry;
    /// - substitutes the `*+letter` sequences to upper case letter;
    /// - normalizes the diacritics ordering;
    /// - converts from ascii betacode to unicode Greek;
    /// - applies specific conversion rules to sigmas.
    ///
    pub fn convert<T: Into<String>>(input: T) -> String {
        let mut output = input.into().to_lowercase();
        output = find_upper(output);
        output = reorder_diacritics(output);
        output = ascii_to_unicode(output);
        output = sigma_handler(output);
        output = normalize_unicode(output);

        output
    }
    #[cfg(test)]
    mod tests_converter {
        use super::*;
        #[test]
        fn capital_letters() {
            let result = find_upper("*a".to_string());
            assert_eq!(result, "A".to_string());
            let result = find_upper("*b".to_string());
            assert_eq!(result, "B".to_string());
            let string = String::from("*#3");
            let result = find_upper(string.clone());
            assert_eq!(result, string);
        }
        #[test]
        fn capital_letters_from_uppercase() {
            let result = find_upper("*A".to_string());
            assert_eq!(result, "A".to_string());
        }
        #[test]
        fn reorder_diacritic() {
            let string = "A/)".to_string();
            let result = reorder_diacritics(string);
            assert_eq!(result, "A)/".to_string());
            let string = "A|/)".to_string();
            let result = reorder_diacritics(string);
            assert_eq!(result, "A)/|".to_string());
            let string = "A/|)".to_string();
            let result = reorder_diacritics(string);
            assert_eq!(result, "A)/|".to_string());
            let string = "A/+".to_string();
            let result = reorder_diacritics(string);
            assert_eq!(result, "A+/".to_string());
        }
        #[test]
        fn test_convert() {
            let string = String::from("a)");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ἀ").to_string());
            let string = String::from("a)/");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ἄ").to_string());
            let string = String::from("a)/|");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ᾄ").to_string());
            let string = String::from("a)=|");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ᾆ").to_string());
            let string = String::from("abcdefghiklmnopqrstuvwxyz");
            let result = convert(string);
            assert_eq!(result, "αβξδεφγηικλμνοπθρστυϝωχψζ".to_string());
            let string = String::from("ABCDEFGHIKLMNOPQRSTUVWXYZ");
            let result = convert(string);
            assert_eq!(
                result,
                normalize_unicode("αβξδεφγηικλμνοπθρστυϝωχψζ").to_string()
            );
            let string = String::from("*A*B*C*D*E*F*G*H*I*K*L*M*N*O*P*Q*R*S*T*U*V*W*X*Y*Z");
            let result = convert(string);
            assert_eq!(
                result,
                normalize_unicode("ΑΒΞΔΕΦΓΗΙΚΛΜΝΟΠΘΡΣΤΥϜΩΧΨΖ").to_string()
            );
        }
        #[test]
        fn unicode_normalized() {
            let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
            let output = normalize_unicode("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
            let result = convert(input);
            assert_eq!(result, output);
        }
    }
}

/// Validation module for Betacode texts
pub mod validator {
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
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
            'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
            'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '*',
            '#', '|', ')', '(', '/', '\\', '.', ';', ':', '1', '2', '3', ',', '\'', '+', '=', ' ',
            '\n',
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
    mod test {
        use super::*;
        #[test]
        fn valid_betacode() {
            assert!(validate("a)/").is_ok());
            let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
            assert!(validate(input).is_ok());
        }
        #[test]
        fn invalid_betacode() {
            let input = String::from("ἄ");
            let result = validate(input);
            assert!(result.is_err());
            match result {
                Ok(_) => (),
                Err(e) => {
                    if let ValidationError::InvalidChars(b) = e {
                        assert_eq!(b, vec!['ἄ']);
                    }
                }
            }
        }
    }
}
