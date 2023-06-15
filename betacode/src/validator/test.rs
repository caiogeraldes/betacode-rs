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
