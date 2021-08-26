# parsley

This program is a set of tools to check [EBNF rules](https://en.wikipedia.org/w/index.php?title=Ebnf).

Note that this project uses a non standard syntax of EBNF that is taught in the course ["Introduction to Programming" at ETH](https://www.lst.inf.ethz.ch/education/archive/Fall2020/einfuehrung-in-die-programmierung-i--252-0027-.html)

## Usage
To get an overview of all available commands, run
```
parsley help
```

### Examples
```
# checks for syntax errors in a file
parsley parse rules/scream
parsley parse rules/errors
```
```
# checks if the given word is contained in the language described by the given grammar
parsley check rules/scream 'a'
parsley check rules/scream 'aAaAaaAaa'
parsley check rules/scream 'foo'
```
```
# checks if the list of words given by a file is contained in the language
parsley check-file rules/scream rules/scream.test
```
```
# prints words that differ in those two grammars
parsley compare rules/scream rules/long-scream 
```
```
# prints up to 50 words that are accepted by this grammar
parsley produce-words rules/scream 50
```
