Requirements for building this project:

- The *Glasgow Haskell Compiler* (GHC) version `~8.10.7` (on path)
- The *cabal* package manager (cabal-install)
- The *stack* building tool
- *alex* (a lexer generator)
- *happy* (a parser generator)

These are included in the Haskell toolchain. To download it, go
to the [Haskell downloads page](https://www.haskell.org/downloads/)
and follow the instructions on the page.

*Note* that setting up in Windows will require a [MSYS2](https://www.msys2.org/)
(a GNU environment) installed. It is better to allow it to install MSYS2,
even if one is already present in the computer, as updates to your main
installation are known to break the toolchain.
