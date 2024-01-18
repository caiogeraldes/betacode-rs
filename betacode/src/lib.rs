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

/// Module containing functions necessary for converting from and into betacode.
pub mod converter;

/// Validation module for Betacode texts
pub mod validator;
