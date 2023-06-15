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
