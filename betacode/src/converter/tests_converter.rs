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
    assert_eq!(result, compose_unicode("ἀ").to_string());
    let string = String::from("a)/");
    let result = convert(string);
    assert_eq!(result, compose_unicode("ἄ").to_string());
    let string = String::from("a)/|");
    let result = convert(string);
    assert_eq!(result, compose_unicode("ᾄ").to_string());
    let string = String::from("a)=|");
    let result = convert(string);
    assert_eq!(result, compose_unicode("ᾆ").to_string());
    let string = String::from("abcdefghiklmnopqrstuvwxyz");
    let result = convert(string);
    assert_eq!(result, "αβξδεφγηικλμνοπθρστυϝωχψζ".to_string());
    let string = String::from("ABCDEFGHIKLMNOPQRSTUVWXYZ");
    let result = convert(string);
    assert_eq!(
        result,
        compose_unicode("αβξδεφγηικλμνοπθρστυϝωχψζ").to_string()
    );
    let string = String::from("*A*B*C*D*E*F*G*H*I*K*L*M*N*O*P*Q*R*S*T*U*V*W*X*Y*Z");
    let result = convert(string);
    assert_eq!(
        result,
        compose_unicode("ΑΒΞΔΕΦΓΗΙΚΛΜΝΟΠΘΡΣΤΥϜΩΧΨΖ").to_string()
    );
    let string = String::from("aBCDEFGHIKLMNOPQRSTUVWXYZ");
    let result = convert(string);
    assert_eq!(
        result,
        compose_unicode("αΒΞΔΕΦΓΗΙΚΛΜΝΟΠΘΡΣΤΥϜΩΧΨΖ").to_string()
    );
    let string = String::from("*a A");
    let result = convert(string);
    assert_eq!(result, compose_unicode("Α Α").to_string());
}
#[test]
fn unicode_normalized() {
    let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
    let output = compose_unicode("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
    let result = convert(input);
    assert_eq!(result, output);
}
#[test]
fn unordered_diacritics() {
    let input = String::from("mh=nin a/)eide qea\\ *phlhi+a/dew *a)xilh=os");
    let output = String::from("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
    let result = convert(input);
    assert_eq!(result, output);
}
#[test]
fn special_sigma() {
    let input = String::from("mh=nin a/)eide qea\\ *phlhi+a/dew *a)xilh=os3");
    let output = String::from("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆοϲ");
    let result = convert(input);
    assert_eq!(result, output);
    let input = String::from("s3 *s3 s1α");
    let output = String::from("ϲ Ϲ \u{03c2}α");
    let result = convert(input);
    assert_eq!(result, output);
}
#[test]
fn revert_ok() {
    let input = String::from("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
    let output = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
    let result = revert(input);
    assert_eq!(result, output);
}
