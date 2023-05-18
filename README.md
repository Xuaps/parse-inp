# Parse INP

The main goal of this project is to learn how to code in Rust. Secondary goal is to implement a INP parser.

## Approach

A parser is a software component that analyzes the structure of a given input, typically a sequence of characters or tokens, according to a specified grammar or syntax. Its main purpose is to determine if the input is syntactically valid and, if so, to create a structured representation of the input that can be further processed.

Here is a high-level description of how a parser typically works:

1. Lexical Analysis: The input source code is passed through a lexer or tokenizer, which breaks it down into a sequence of tokens. Tokens represent meaningful units such as identifiers, keywords, operators, and literals.

2. Parsing: A parser analyzes the sequence of tokens according to a grammar or syntax rules. It verifies the correctness of the syntax and creates a parse tree, which is an intermediate representation of the program's structure.

3. Abstract Syntax Tree Construction: The parse tree is transformed into an Abstract Syntax Tree by removing unnecessary nodes and restructuring the tree to represent the higher-level structure of the program. The AST focuses on the essential elements of the syntax and discards details such as punctuation and precedence.


```
        +-------------------+
        |   Input Source    |
        |     Code          |
        +--------+----------+
                 |
                 v
        +--------+----------+
        |   Lexer/Tokenizer |
        |   (Lexical        |
        |   Analysis)       |
        +--------+----------+
                 |
                 v
        +--------+----------+
        |       Parser      |
        |   (Syntactic      |
        |   Analysis)       |
        +--------+----------+
                 |
                 v
        +--------+----------+
        | Abstract Syntax   |
        |   Tree (AST)      |
        +--------+----------+

```

## Lexer

```
+-------------------+
|  Input Source     |
|    Code           |
+-------------------+
         |
         v
+-------------------+
|    Scan Input     |
|                   |
+-------------------+
         |
         v
+-------------------+
|  Apply Pattern    |
|   Matching        |
+-------------------+
         |
         v
+-------------------+
|  Create Token     |
|    Object         |
+-------------------+
         |
         v
+-------------------+
| Repeat Steps 2-3  |
|   Until End       |
|   of Input        |
+-------------------+
```

