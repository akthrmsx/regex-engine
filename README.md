# Regex engine

This is simple regex engine written in Rust.

## Supported engine types

* DFA
* VM (TODO)

## Supported features

* `|`
* `*`
* `(` and `)`

## Examples

```sh
$ regex-engine -h
String matcher by regular expression

Usage: regex-engine [OPTIONS] <PATTERN> <TEXT>

Arguments:
  <PATTERN>  Regular expression pattern
  <TEXT>     Target text

Options:
  -t, --type <ENGINE_TYPE>  Engine type [default: dfa] [possible values: dfa, vm]
  -h, --help                Print help (see more with '--help')
```

```sh
$ regex-engine "P(erl|ython|HP)|Ruby" "Perl"
Matched

$ regex-engine "P(erl|ython|HP)|Ruby" "Python"
Matched

$ regex-engine "P(erl|ython|HP)|Ruby" "PHP"
Matched

$ regex-engine "P(erl|ython|HP)|Ruby" "Ruby"
Matched

$ regex-engine "P(erl|ython|HP)|Ruby" "Rust"
Unmatched
```

## References

* [正規表現技術入門 - 最新エンジン実装と理論的背景](https://gihyo.jp/book/2015/978-4-7741-7270-5)
