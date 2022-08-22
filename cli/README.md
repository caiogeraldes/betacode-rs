# betaconvert

A rust CLI for Betacode conversion.

## Usage

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
