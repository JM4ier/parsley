# parsley

This program is a set of tools to check [EBNF rules](https://en.wikipedia.org/w/index.php?title=Ebnf).

Note that this project uses a non standard syntax of EBNF that is taught in the course ["Introduction to Programming" at ETH](https://www.lst.inf.ethz.ch/education/archive/Fall2020/einfuehrung-in-die-programmierung-i--252-0027-.html)

## Usage
To get an overview of all available commands, run
```
parsley help
```

To check for syntax errors in a file, use the subcommand `parse`.
```
# This should not contain any error
parsley parse rules/scream

# This should contain a wide range of syntax errors
parsley parse rules/errors
```

To check if a word is contained in a language described by some EBNF rules, use the subcommand `check`.
```
parsley check rules/scream 'a'
parsley check rules/scream 'aAaAaaAaa'
parsley check rules/scream 'foo'
```

To see if a whole list of words is contained in a language, use the subcommand `check-file`.
Note: the words need to be separated by a newline.
```
parsley check-file rules/scream rules/scream.test
```

To compare two grammars, and see if there are words that are accepted by one but not the other, use the subcommand `compare`.
There is an optional parameter to specify how many words to check.
```
parsley compare rules/scream rules/long-scream 
```

To get a list of words that fit to a set of rules, use subcommand `produce-words`.
There is an optional argument to specify how many words.
```
parsley produce-words rules/scream
parsley produce-words rules/scream 50
```
