# Tomato Parser

Generate a simple langauge call `tomato`, and implement a parser for it in Rust.

## Sample Tomato Codes

1. [fibonacci](./fib.tomato)
2. [fizzbuzz](./fizzbuzz.tomato)
3. [sum 1 to 10](./sum_1_to_10.tomato)

## Parser with a simple parser

The Rust program in this directory is a simple REPL-like parser for the `tomato` language.

It reads the filename from the command line, and parses the file into an AST.
Then it creates a simple context for a generated AST, and evaluates the AST.

You could find the full source code [here](./src/).
