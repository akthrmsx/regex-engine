# toy-regex

This is toy regex engine written in Rust.

## Engine types

* DFA
* VM (TODO)

## Supported features

* `|`
* `*`
* `(` and `)`

## Examples

```sh
$ toy-regex -h
Regular expression matcher by DFA

Usage: toy-regex <PATTERN> <TEXT>

Arguments:
  <PATTERN>  Regular expression pattern
  <TEXT>     Target text

Options:
  -h, --help  Print help
```

```sh
$ toy-regex "P(erl|ython|HP)|Ruby" "Perl"
Matched

$ toy-regex "P(erl|ython|HP)|Ruby" "Python"
Matched

$ toy-regex "P(erl|ython|HP)|Ruby" "PHP"
Matched

$ toy-regex "P(erl|ython|HP)|Ruby" "Ruby"
Matched

$ toy-regex "P(erl|ython|HP)|Ruby" "Rust"
Unmatched
```

## References

* [正規表現技術入門 - 最新エンジン実装と理論的背景](https://gihyo.jp/book/2015/978-4-7741-7270-5)
