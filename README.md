# parsley

This program is a set of tools to check [EBNF rules](https://en.wikipedia.org/w/index.php?title=Ebnf).

Note that this project uses a non standard syntax of EBNF that is taught in the course ["Introduction to Programming" at ETH](https://www.lst.inf.ethz.ch/education/archive/Fall2020/einfuehrung-in-die-programmierung-i--252-0027-.html)

It is also still a work in progress.

## Usage
As it's still a work in progress, some commands might change.
I'll try to keep this readme updated at all times.

To get an overview of all available commands, run
```
$ cargo run -- help
```

### Examples
```
$ cargo run -- rules/scream check 'a'
$ cargo run -- rules/scream check 'aAaAaaAaa'
$ cargo run -- rules/scream check 'foo'
```
```
$ cargo run -- rules/scream compare-to rules/long-scream # prints words that differ in those two grammars
```
```
$ cargo run -- rules/scream produce-words 50 # prints up to 50 words that are accepted by this grammar
```
