## Kodesprog compiler - lexical analyser

This repository is the lexical analyser for the official kodesprog (ks) programming language.

It is custom built to tokenize .ks files, that then will be compiled into an executeable binary.

The ks lexer is written in RUST.

### Lexer

The Lexer struct itself takes a vector of characters as input. As such, it works with rust chars.

```
1 rust char = 4 byte

rust char = scalar unicode = 0x10FFFF
```

