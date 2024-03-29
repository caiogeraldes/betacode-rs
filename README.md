# betacode-rs

[![Production](https://github.com/caiogeraldes/betacode-rs/actions/workflows/rust-production.yml/badge.svg)](https://github.com/caiogeraldes/betacode-rs/actions/workflows/rust-production.yml)

A rust library and CLI for Betacode conversion.

## Library

### Adding as a dependency

```bash
$ cargo add betacode
```

Or add to `Cargo.toml`:

```toml
[dependencies]
betacode = "1.0.1"
```

### Conversion

Example:

```rust
use betacode::converter;

let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
let output = String::from("μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος");
let result = betacode::converter::convert(input);
assert_eq!(result, output);
```

### Validation

Validating a Betacode text consists in validating whether or not it follows the rules:

- it is composed solely by ASCII characters, else it returns `ValidationError::NotASCII` with the invalid characters;
- its characters are handled by the converter module, else it returns `ValidationError::InvalidChars` with the invalid characters;
- the disposition of its characters is interpretable by the converter module, else it returns `ValidationError::InvalidDiacriticOrder` with the invalid sequences.

The later is arguably the more easily recoverable, by means of the function `converter::reorder_diacritics`.
The former pair might be recovered by ignoring invalid characters.

**Details:**

If the text is input in proper ASCII Betacode (and the `converter`(super::converter) can convert it), it
returns Ok().

```rust
let input = String::from("mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os");
assert!(betacode::validator::validate(input).is_ok());
```

Otherwise, it specifies what error occurred.

For example, if passed a string with non-ASCII characters such as ἄλγεα,
 it stores a list of all characters that break the validation in the enum
 `ValidationError::NotASCII`.
    
 ```rust
 let input = String::from("ἄλγεα");
 let result = betacode::validator::validate(input);
 assert!(result.is_err());
 match result {
     Ok(_) => (),
     Err(e) => {
         if let betacode::validator::ValidationError::NotASCII(b) = e {
             assert_eq!(b, vec!['ἄ', 'λ','γ','ε','α']);
         }
     }
 }
 ```

 If the string is ASCII, but the proper conversion rule has not been implemented, it stores
 the list of characters that are not convertable in the enum `ValidationError::InvalidChars`

```rust
let input = String::from("9");
let result = betacode::validator::validate(input);
assert!(result.is_err());
match result {
    Ok(_) => (),
    Err(e) => {
        if let betacode::validator::ValidationError::InvalidChars(b) = e {
            assert_eq!(b, vec!['9']);
        }
    }
}
```

If, on other hand, the text contains an order of diacritics that can not
be directly converted, it returns the list of sequences that are not valid.
The converter module still can convert it, but this is implemented to assure
that the corpus is properly built for other tools to operate.
It stores all the patterns that break the `BREATH/DIAIRESIS + ACCENT + SUB-IOTA`
order in `ValidationError::InvalidDiacriticOrder`.


 ```rust
 let input = String::from("h\\( a/)ndra");
 let result = betacode::validator::validate(input);
 assert!(result.is_err());
 match result {
     Ok(_) => (),
     Err(e) => {
         if let betacode::validator::ValidationError::InvalidDiacriticOrder(b) = e {
             assert_eq!(b, vec!["\\(".to_string(), "/)".to_string()]);
         }
     }
 }
 ```


## CLI

### Intalation

Assuming you have `rust` installed in your system, clone this repository and run:

```bash
cargo install --path ./cli/
```

It should do the job.

### Usage

The basic usage of the CLI is very straightforward:

```bash
$ betaconvert "mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os"
> μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος
```

The output might be stored into a file by passing the `-o` or `--output` argument:

```bash
$ betaconvert "mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os" -o iliad1.1.txt
$ echo iliad1.1.txt
> μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος
```

If needed, the `-f` or `--file` flag might be passed to assume the passed string as an input file name and not as a string to be converted.

```bash
$ echo "mh=nin a)/eide qea\\ *phlhi+a/dew *a)xilh=os" >> iliad1.1.txt
$ betaconvert -f iliad1.1.txt
> μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος
```
